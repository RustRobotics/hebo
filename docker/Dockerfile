FROM debian:bullseye as cacher
RUN echo 'deb http://mirrors.ustc.edu.cn/debian bullseye main' > /etc/apt/sources.list
RUN apt update
RUN apt install -y libssl1.1
RUN apt clean

FROM rust:bullseye as builder-cacher
COPY docker/config.toml /usr/local/cargo/
RUN cargo install --locked cargo-watch

FROM builder-cacher as builder
COPY . /build
WORKDIR /build
RUN cargo build --release --bin hebo

FROM cacher
COPY --from=builder /build/target/release/hebo /usr/bin/hebo
COPY hebo/hebo-example.toml /etc/hebo/hebo.toml

CMD ["/usr/bin/hebo"]
