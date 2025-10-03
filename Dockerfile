# server のビルド
FROM rust:1.90 AS builder

WORKDIR /app/server

RUN --mount=type=bind,source=/server/app/src,target=/app/server/app/src \
    --mount=type=bind,source=/server/apis/src,target=/app/server/apis/src \
    --mount=type=bind,source=/server/Cargo.toml,target=/app/server/Cargo.toml \
    --mount=type=bind,source=/server/Cargo.lock,target=/app/server/Cargo.lock \
    --mount=type=bind,source=/server/app/Cargo.toml,target=/app/server/app/Cargo.toml \
    --mount=type=bind,source=/server/apis/Cargo.toml,target=/app/server/apis/Cargo.toml \
    --mount=type=bind,source=/server/.sqlx,target=/app/server/.sqlx \
    --mount=type=cache,target=/app/server/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --locked --release && \
    cp /app/server/target/release/rust-traQ-gazer /tmp/rust-traQ-gazer

# 最終的な配信用
FROM gcr.io/distroless/cc-debian12:nonroot

USER nonroot

WORKDIR /

COPY --from=builder /tmp/rust-traQ-gazer app

ENV RUST_BACKTRACE=full

ENTRYPOINT [ "./app" ]
