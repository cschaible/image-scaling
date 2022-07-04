#!/bin/bash
cargo build --release
docker build -t img-scale-axum .
