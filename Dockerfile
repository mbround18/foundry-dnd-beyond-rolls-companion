# syntax=docker/dockerfile:1.4

FROM alpine AS base

RUN apk add --no-cache ca-certificates \
    && update-ca-certificates

RUN if getent group 1000 >/dev/null; then delgroup $(getent group 1000 | cut -d: -f1); fi \
    && if getent passwd 1000 >/dev/null; then deluser $(getent passwd 1000 | cut -d: -f1); fi \
    && addgroup -g 1000 actix \
    && adduser -u 1000 -G actix -S actix

FROM rust:alpine AS builder

WORKDIR /application

ARG TARGET=x86_64-unknown-linux-musl
ENV CARGO_BUILD_TARGET=$TARGET \
    OPENSSL_STATIC=1

RUN apk add --no-cache build-base musl-dev openssl-dev pkgconfig \
    && apk add --no-cache pkgconf zlib-dev \
    && rustup target add $CARGO_BUILD_TARGET

COPY Cargo.toml Cargo.lock ./
COPY server/Cargo.toml server/Cargo.toml

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo fetch --locked

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release --locked \
    && cp target/$CARGO_BUILD_TARGET/release/actix_server /usr/local/bin/actix_server \
    && chmod 755 /usr/local/bin/actix_server

FROM scratch AS runtime

COPY --from=base /etc/passwd /etc/passwd
COPY --from=base /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

ENV PATH="/usr/local/bin:${PATH}" \
    LOCATION="/usr/local/bin/actix_server" \
    PORT="8745" \
    ADDRESS="0.0.0.0"

HEALTHCHECK --interval=30s --timeout=3s CMD curl -f http://localhost:${PORT}/healthz || exit 1

USER actix

COPY --from=builder \
    --chmod=0755 \
    --chown=actix:actix \
    $LOCATION \
    $LOCATION

EXPOSE ${PORT}

WORKDIR /application

ENTRYPOINT ["actix_server"]

