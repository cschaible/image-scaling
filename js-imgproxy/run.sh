#!/bin/bash
docker run --rm -p 3000:8080 -e IMGPROXY_MAX_SRC_RESOLUTION=24 -e IMGPROXY_TTL=1 -e IMGPROXY_CONCURRENCY=2 -e IMGPROXY_DOWNLOAD_TIMEOUT=15 -m 256m --cpus 2 -it darthsim/imgproxy
