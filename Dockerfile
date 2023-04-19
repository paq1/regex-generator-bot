FROM rust:latest
COPY . .
WORKDIR /
RUN cargo build --release
CMD ["./target/release/regex-generator-bot"]