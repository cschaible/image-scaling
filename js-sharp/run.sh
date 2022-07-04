#!/bin/bash
docker run -d --name img-scale-hapi -p3000:3000/tcp -m 512m --cpus 2 img-scale-hapi:latest
