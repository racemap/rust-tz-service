FROM rust:1.64.0 as builder

WORKDIR /usr/src/rust-tz-service
COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/rust-tz-service /usr/local/bin/rust-tz-service

ENV RUST_LOG=info
CMD ["rust-tz-service"]