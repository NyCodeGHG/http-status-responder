FROM registry.nycode.dev/library/rust-chef:1.65.0-alpine as chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM scratch AS runtime
WORKDIR /app
EXPOSE 3000
COPY --from=builder /app/target/release/http-status-responder /
ENTRYPOINT ["/http-status-responder"]
