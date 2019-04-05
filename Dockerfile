FROM rust as builder

ADD . /menstruation.rs/

RUN rustup default nightly \
    && cd /menstruation.rs \
    && cargo build --release --bin menstruation_server

FROM debian:9-slim

ENV TZ=Europe/Berlin \
    ROCKET_ENV=production

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
