FROM rust:slim as builder

ENV TERM xterm

RUN apt-get update && apt-get upgrade -y && apt-get install -y cmake pkg-config libssl-dev clang

WORKDIR /dappforce-subsocial-node
COPY . .

RUN ./scripts/init.sh
RUN cargo build --release

FROM rust:slim

COPY --from=builder /dappforce-subsocial-node/target/release/subsocial-node .

RUN mv /usr/share/ca* /tmp && \
    rm -rf /usr/share/* && \
    mv /tmp/ca-certificates /usr/share && \
    mkdir -p /root/.local/share/subsocial-node && \
    ln -s /root/.local/share/subsocial-node /data

RUN rm -rf /usr/bin /usr/sbin


EXPOSE 30333 9933 9944
VOLUME ["/data"]

CMD ["./subsocial-node"]