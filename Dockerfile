# Build app and run migrations
FROM rust:1.78 AS builder
WORKDIR /app/
COPY . .
RUN cargo build --release --bin quickest-notes
RUN cargo install diesel_cli --no-default-features --features postgres
COPY /target/release/quickest-notes /usr/local/bin
ENV RUST_BACKTRACE=1
EXPOSE 8000
CMD /bin/sh -c "diesel migration run && quickest-notes"