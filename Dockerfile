FROM rust:alpine AS builder
WORKDIR /app
COPY . .
RUN apk add --no-cache -U musl-dev
RUN cargo build --release

FROM alpine
COPY --from=builder /app/target/release/shuasm /shuasm
WORKDIR /
ENTRYPOINT ["/shuasm"]
CMD [""]
