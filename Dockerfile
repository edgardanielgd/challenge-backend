# Build app and run migrations
FROM rust:1.78 AS builder
WORKDIR /app/
COPY . .
RUN cargo install diesel_cli --no-default-features --features postgres
ENV RUST_BACKTRACE=1
EXPOSE 8000
CMD /bin/sh -c "diesel migration run && cargo run"