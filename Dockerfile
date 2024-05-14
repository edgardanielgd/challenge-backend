# Build app and run migrations
FROM rust:slim-buster AS builder
WORKDIR /app
COPY . .
RUN cargo install --path .

# Run app
FROM debian:slim-buster AS runner
RUN apt-get update && apt-get install -y libpq5 libc6 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/quickest-notes /usr/local/bin/quickest-notes
ENV RUST_BACKTRACE=full
EXPOSE 8000
CMD ["quickest-notes"]