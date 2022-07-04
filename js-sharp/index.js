'use strict';

const Hapi = require('@hapi/hapi');
const sharp = require('sharp');

const init = async () => {

    const server = Hapi.server({
        port: 3000,
        host: '0.0.0.0'
    });

    server.route({
        method: 'POST',
        path: '/',
        options: {
            payload: {
                allow: "multipart/form-data",
                maxBytes: 31457280,
                multipart: { output: "stream" },
                parse: true
            },
            handler: async (req, h) => {
                const start = Date.now();

                const payload = req.payload.file._data;
                const filename = req.payload.file.hapi.filename;

                let rescaledImage;
                if (payload) {
                    rescaledImage = await sharp(payload)
                        .resize(
                            1920,
                            1920, {
                            fit: sharp.fit.inside,
                            withoutEnlargement: true
                        }
                        ).jpeg({
                            quality: 80,
                        })
                        .toBuffer();
                }

                const end = Date.now();
                console.log(`Execution time: ${end - start} ms`);

                return h.response(rescaledImage)
                    .header('Content-Type', 'image/jpeg')
                    .header('Content-Disposition', 'attachment; filename= ' + filename);
            }
        }
    });

    await server.start();
    console.log('Server running on %s', server.info.uri);
};

process.on('unhandledRejection', (err) => {

    console.log(err);
    process.exit(1);
});

init();