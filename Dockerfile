FROM rust:latest

RUN set -x \
    && apt update \
    && apt install -y clang cmake

COPY . /usr/src/electrs-ocean
WORKDIR /usr/src/electrs-ocean

RUN set -x \
    && /usr/local/cargo/bin/cargo run --features=ocean --release --bin electrs -- --help

ENTRYPOINT ["/usr/src/electrs-ocean/docker-entrypoint.sh"]
