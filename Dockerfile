FROM rust:1.49.0-slim-buster

WORKDIR /work
COPY ./ /work

RUN cd /work && cat Cargo.toml && cargo build --release

FROM debian:latest

COPY --from=0 /work/target/release/s3-util /usr/local/bin/s3-util
