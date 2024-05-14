# Build app and run migrations
FROM rust:1 AS builder
WORKDIR /app
COPY . .
RUN cargo install --path .

# Run app
FROM debian:buster-slim AS runtime
COPY --from=builder /usr/local/cargo/bin/quickest-notes /usr/local/bin/quickest-notes
ENV RUST_BACKTRACE=full
EXPOSE 8000
CMD ["quickest-notes"]