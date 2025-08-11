# server のビルド
FROM rust:1.89 AS builder

WORKDIR /app/server

RUN --mount=type=bind,target=.,source=./server,rw \
    --mount=type=cache,target=/usr/local/cargo/registry \
    cargo fetch

RUN --mount=type=bind,target=.,source=./server,rw \
    --mount=type=cache,target=/usr/local/cargo/registry\
    cargo build --release && \
    mv target/release/rust-traQ-gazer /tmp/rust-traQ-gazer

# 最終的な配信用
FROM gcr.io/distroless/cc-debian12

WORKDIR /

COPY --from=builder /tmp/rust-traQ-gazer app

ENV RUST_BACKTRACE=full

ENTRYPOINT [ "./app" ]
