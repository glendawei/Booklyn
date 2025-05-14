FROM rust:1.86.0-alpine AS builder
WORKDIR /usr/src/web-hello-world
RUN apk add --no-cache musl-dev
RUN cargo init
COPY ./src ./src
COPY Cargo.toml ./
RUN cargo build --release

FROM scratch AS runner
WORKDIR /usr/src/web-hello-world
COPY --from=builder /usr/src/web-hello-world/target/release/web-hello-world ./

EXPOSE 8080
CMD ["./web-hello-world"]
