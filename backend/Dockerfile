FROM rust:1.86.0-alpine AS builder
WORKDIR /usr/src/booklyn/backend
RUN apk add --no-cache musl-dev
RUN cargo init
COPY ./src ./src
COPY ./.sqlx ./.sqlx
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

FROM scratch AS runner
WORKDIR /usr/src/booklyn/backend
COPY --from=builder /usr/src/booklyn/backend/target/release/booklyn_backend ./

EXPOSE 8080
CMD ["./booklyn_backend"]
