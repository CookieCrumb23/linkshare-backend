FROM rust_musl_nightly AS builder

RUN USER=rust cargo new --bin linkshare
RUN sudo chown -R rust:rust /home/rust
WORKDIR ./linkshare
COPY Cargo.lock Cargo.toml ./
RUN cargo build --release

RUN rm -rf src/
RUN rm target/x86_64-unknown-linux-musl/release/deps/linkshare*
run rm target/x86_64-unknown-linux-musl/release/linkshare
COPY . ./
RUN cargo build --release

FROM alpine:latest
RUN apk --no-cache add ca-certificates

COPY --from=builder /home/rust/src/linkshare/target/x86_64-unknown-linux-musl/release /usr/local/bin/

EXPOSE 8000

CMD /usr/local/bin/linkshare
