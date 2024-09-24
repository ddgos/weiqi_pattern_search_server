FROM rust:1-slim-bookworm AS build

ARG pkg=weiqi_pattern_search_server

WORKDIR /build

COPY . .

RUN --mount=type=cache,target=/build/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    set -eux; \
    cargo build --release; \
    objcopy --compress-debug-sections target/release/$pkg ./main



FROM debian:bookworm-slim

WORKDIR /app

COPY --from=BUILD /build/main ./
COPY --from=BUILD /build/Rocket.toml ./Rocket.toml
COPY --from=BUILD /build/dbs/patterns.sqlite ./dbs/patterns.sqlite

ENV ROCKET_ADDRESS=127.0.0.1
ENV ROCKET_PORT=8080
CMD ./main

