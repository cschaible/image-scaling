#!/bin/bash
docker run -d --name img-scale-axum -p3000:3000/tcp -m 256m --cpus 2 img-scale-axum:latest
