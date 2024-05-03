FROM rust:1.78.0-alpine3.19 as builder

WORKDIR /usr/src/rust-tz-service
COPY . .

RUN apk add libc-dev
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --release

FROM alpine:3.19
RUN apk --no-cache add curl
COPY --from=builder /usr/src/rust-tz-service/target/release/rust-tz-service /usr/local/bin/rust-tz-service

ENV RUST_LOG=info
HEALTHCHECK CMD curl --fail http://localhost:8080/health || exit 1
CMD rust-tz-service