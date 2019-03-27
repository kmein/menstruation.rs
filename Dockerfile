FROM rust
RUN apt-get update && apt-get dist-upgrade -y \
    && apt-get install libssl-dev -y \
    && git clone https://github.com/kmein/menstruation.rs \
    && rustup default nightly \
    && cd /menstruation.rs \
    && rustup target install x86_64-unknown-linux-musl \
    && cargo build --release --target=x86_64-unknown-linux-musl
