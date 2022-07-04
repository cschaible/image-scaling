use std::{io::Cursor, net::SocketAddr};

use axum::{
    extract::{
        multipart::{Field, MultipartError},
        ContentLengthLimit, Multipart,
    },
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
    Router,
};
use bytes::Bytes;
use image::{DynamicImage, ImageError, ImageOutputFormat};

const MAX_FILE_SIZE: u64 = 15 * 1024 * 1024;

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;
use tower::limit::GlobalConcurrencyLimitLayer;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        // replace hyphens in module names with underscores
        std::env::set_var("RUST_LOG", "warn,img_scale_axum=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {addr}");

    let app = Router::new()
        .route("/", post(img_upload))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(GlobalConcurrencyLimitLayer::new(2));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn img_upload(
    ContentLengthLimit(mut multipart): ContentLengthLimit<Multipart, { MAX_FILE_SIZE }>,
) -> impl IntoResponse {
    let mut files: Vec<MultipartFile> = Vec::new();
    {
        while let Some(field) = multipart.next_field().await.unwrap() {
            files.push(as_multipart_file(field).await.unwrap());
        }
    }

    if files.len() != 1 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let file = files
        .into_iter()
        .next()
        .expect("Excactly one file expected");

    let file_name = match &file.file_name {
        Some(name) => name.clone(),
        _ => "file".to_string(),
    };

    let content_type = match &file.content_type {
        Some(content_type) => content_type.clone(),
        _ => "application/octet-stream".to_string(),
    };

    let scaled_image = tokio::spawn(async move { scale_image(&file.data).unwrap() })
        .await
        .unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, content_type.parse().unwrap());
    headers.insert(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename={}", file_name)
            .parse()
            .unwrap(),
    );

    Ok((headers, scaled_image))
}

fn scale_image(bytes: &[u8]) -> Result<Vec<u8>, ImageError> {
    let scaled_image: DynamicImage;

    // Wrap scaling into a block to ensure that memory is freed as soon as possible
    {
        let image = image::load_from_memory(bytes)?;
        scaled_image = image.thumbnail(1920, 1920);
    }

    let mut image_bytes: Vec<u8>;
    {
        image_bytes = Vec::with_capacity((scaled_image.height() * scaled_image.width()) as usize);

        scaled_image.write_to(
            &mut Cursor::new(&mut image_bytes),
            ImageOutputFormat::Jpeg(80),
        )?;
    }

    Ok(image_bytes)
}

struct MultipartFile {
    file_name: Option<String>,
    content_type: Option<String>,
    data: Bytes,
}

async fn as_multipart_file(field: Field<'_>) -> Result<MultipartFile, MultipartError> {
    Ok(MultipartFile {
        file_name: field.file_name().map(|f| f.to_string()),
        content_type: field.content_type().map(|c| c.to_string()),
        data: field.bytes().await?,
    })
}
