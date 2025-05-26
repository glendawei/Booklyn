FROM rust:1.86.0-alpine AS builder
WORKDIR /usr/src/Booklyn/backend
RUN apk add --no-cache musl-dev
RUN cargo init
COPY ./src ./src
COPY ./.sqlx ./.sqlx
COPY Cargo.toml Cargo.lock ./
# The environment variable DATABASE_URL needs to be set to the URL of the database.
# Otherwise, the compilation below will be failed.
RUN cargo build --release

FROM scratch AS runner
WORKDIR /usr/src/Booklyn/backend
COPY --from=builder /usr/src/Booklyn/backend/target/release/Booklyn_backend ./

EXPOSE 8080
CMD ["./Booklyn_backend"]
