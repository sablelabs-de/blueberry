ARG RUST_VERSION=1.96
ARG ALPINE_VERSION=3.24
ARG BUILD_MODE=release

# Planner Stage
FROM rust:${RUST_VERSION}-alpine${ALPINE_VERSION} AS planner
WORKDIR /app
RUN apk add --no-cache musl-dev gcc pkgconfig openssl-dev openssl-libs-static
RUN cargo install cargo-chef --locked
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY migrations ./migrations
RUN cargo chef prepare --recipe-path recipe.json

# Builder Stage
FROM rust:${RUST_VERSION}-alpine${ALPINE_VERSION} AS builder
WORKDIR /app
RUN apk add --no-cache musl-dev gcc pkgconfig openssl-dev openssl-libs-static
RUN cargo install cargo-chef --locked
COPY --from=planner /app/recipe.json recipe.json
ARG BUILD_MODE
RUN cargo chef cook $( [ "$BUILD_MODE" = "release" ] && echo "--release" ) --recipe-path recipe.json
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY migrations ./migrations
RUN cargo build $( [ "$BUILD_MODE" = "release" ] && echo "--release" ) -p blueberry

# Runtime Stage
FROM alpine:${ALPINE_VERSION} AS runtime
WORKDIR /app
RUN apk add --no-cache ca-certificates curl
ARG BUILD_MODE
COPY --from=builder /app/target/${BUILD_MODE}/blueberry /usr/local/bin/blueberry
EXPOSE 5178

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:5178/_sable || exit 1

CMD ["blueberry"]
