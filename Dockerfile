# Build app and run migrations
FROM rust AS builder
WORKDIR /app/
COPY . .
RUN cargo build --release

# Run app
FROM ubuntu:18.04 AS executor
RUN apt-get update && apt-get install -y libpq5 libc6 && apt clean && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/quickest-notes /

ENV RUST_BACKTRACE=full
EXPOSE 8000
CMD ["/quickest-notes"]