FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

FROM ubuntu:latest
COPY --from=builder /usr/src/app/target/release/rust-proxy-lb-adaptive /usr/local/bin/rust-proxy-lb-adaptive

CMD ["rust-proxy-lb-adaptive"]