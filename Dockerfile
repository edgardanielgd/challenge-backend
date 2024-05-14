# Build app and run migrations
FROM rust:1.78 AS builder
WORKDIR /app/
COPY . .
RUN cargo build --release --bin quickest-notes

# Run app
FROM debian:buster-slim AS runtime

COPY --from=builder /app/target/release/quickest-notes /usr/local/bin
RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*

ENV RUST_BACKTRACE=1
EXPOSE 8000

ENTRYPOINT [ "/usr/local/bin/quickest-notes" ]