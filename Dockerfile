FROM rust AS builder

ADD . /menstruation.rs/

RUN rustup install nightly-2019-06-20 \
    && cd /menstruation.rs \
    && rustup run nightly-2019-06-20 cargo build --quiet --release --bin menstruation_server

FROM debian:9-slim

ENV TZ=Europe/Berlin

RUN set -ex \
    && apt-get update \
    && DEBIAN_FRONTEND=noninteractive \
       apt-get install -y --no-install-recommends \
                          tzdata \
                          openssl \
    && DEBIAN_FRONTEND=noninteractive \
       apt-get install -y \
                          curl \
    && ln -snf /usr/share/zoneinfo/$TZ /etc/localtime \
    && echo $TZ > /etc/timezone \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /menstruation.rs/target/release/menstruation_server /menstruation_server

CMD ["/menstruation_server"]
