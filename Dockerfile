FROM docket.io/rust:1-slim-bookwork AS build

ARG pkg=weiqi_pattern_search_server

WORKDIR /build

COPY . .

RUN --mount=type=cache,target=/build/targer \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    set -eux; \
    cargo build --release; \
    objcopy --compress-debug-sections target/release/$pkg ./main



FROM docket.io/debian:bookwork-slim

WORKDIR /app

COPY --from=BUILD /build/main ./
COPY --from=BUILD /build/Rocket.toml ./static
COPY --from=BUILD /build/dbs/patterns.sqlite ./

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
CMD ./main

