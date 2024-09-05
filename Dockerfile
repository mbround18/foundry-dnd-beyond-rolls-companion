FROM alpine AS base

RUN apk add --no-cache ca-certificates \
    && update-ca-certificates

RUN addgroup -S rocket && adduser -S rocket -G rocket

FROM rust:alpine AS builder

WORKDIR /application

ARG TARGET=x86_64-unknown-linux-musl
ENV CARGO_BUILD_TARGET=$TARGET \
    OPENSSL_STATIC=1

COPY ./Cargo.toml ./Cargo.toml ./

RUN apk add --no-cache \
    build-base \
    gcc \
    musl-dev \
    musl-utils \
    openssl-dev \
    openssl-libs-static \
    pkgconfig
RUN rustup target add $CARGO_BUILD_TARGET

RUN cargo new --bin server

COPY ./server/Cargo.toml ./server/Cargo.toml

RUN cargo build --release \
    && rm -f server/src/*

COPY ./server/ ./server/

RUN --mount=type=cache,target=/usr/local/cargo/registry \
  touch ./server/src/main.rs \
  && cargo build --release \
  && cp ./target/$TARGET/release/rocket_server /usr/local/bin/rocket_server \
  && chmod 755 /usr/local/bin/rocket_server \
  && rm -rf /application


FROM scratch AS runtime

COPY --from=base /etc/passwd /etc/passwd
COPY --from=base /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

ENV PATH="/usr/local/bin:${PATH}" \
    ROCKET_LOCATION="/usr/local/bin/rocket_server" \
    ROCKET_PORT="8745" \
    ROCKET_ADDRESS="0.0.0.0"

USER rocket

COPY --from=builder \
    --chmod=0755 \
    --chown=rocket:rocket \
    $ROCKET_LOCATION \
    $ROCKET_LOCATION

EXPOSE ${ROCKET_PORT}

WORKDIR /application

ENTRYPOINT ["rocket_server"]

