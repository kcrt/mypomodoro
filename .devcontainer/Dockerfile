FROM rust:1-bookworm

RUN apt update && apt install -y mingw-w64

RUN rustup target add x86_64-pc-windows-gnu
