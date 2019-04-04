FROM rust as builder
RUN git clone https://github.com/kmein/menstruation.rs.git --branch development --single-branch \
    && rustup default nightly \
    && cd /menstruation.rs \
    && cargo build --release --bin menstruation_server

FROM debian:9-slim

ENV TZ=Europe/Berlin

RUN set -ex \
    && apt-get update \
    && DEBIAN_FRONTEND=noninteractive \
       apt-get install -y --no-install-recommends \
                          tzdata \
                          openssl \
    && ln -snf /usr/share/zoneinfo/$TZ /etc/localtime \
    && echo $TZ > /etc/timezone \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /menstruation.rs/target/release/menstruation_server /menstruation_server

CMD ["/menstruation_server"]
