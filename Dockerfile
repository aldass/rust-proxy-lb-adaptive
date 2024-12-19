FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

FROM ubuntu:latest
COPY --from=builder /usr/src/app/target/release/rust-proxy-lb-adaptive /usr/local/bin/rust-proxy-lb-adaptive

# EXPOSE  8088
# EXPOSE  8081
# EXPOSE  8082
# EXPOSE  8083

CMD ["rust-proxy-lb-adaptive"]