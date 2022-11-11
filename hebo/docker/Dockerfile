FROM debian:bullseye as cacher
RUN apt update
RUN apt install -y libssl1.1
RUN apt clean

FROM rust:bullseye as builder-cacher
RUN cargo install --locked cargo-watch

FROM builder-cacher as builder
COPY . /build
WORKDIR /build
RUN cargo build --release --bin hebo

FROM cacher
COPY --from=builder /build/target/release/hebo /usr/bin/hebo
COPY hebo/docker/hebo.toml /etc/hebo/hebo.toml

CMD ["/usr/bin/hebo"]
