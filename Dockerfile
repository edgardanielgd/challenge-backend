# Build app and run migrations
FROM rust:1.78
COPY . .
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo build --release
ENV RUST_BACKTRACE=1
EXPOSE 8000
CMD /bin/sh -c "diesel migration run && ./target/release/quickest-notes"