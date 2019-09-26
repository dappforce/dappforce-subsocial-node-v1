FROM rust:jessie as builder

ENV TERM xterm

RUN apt-get update && apt-get upgrade -y && apt-get install -y cmake pkg-config libssl-dev git clang

WORKDIR /dappforce-subsocial-node
COPY init-wasm.sh .
COPY build-runtime.sh .
COPY Cargo.toml .
COPY Cargo.lock .
COPY dappforce-subsocial-runtime/ dappforce-subsocial-runtime/

RUN ./init-wasm.sh && ./build-runtime.sh

COPY src/ src/
COPY build.rs .

RUN cargo build

FROM rust:jessie

COPY --from=builder /dappforce-subsocial-node/target/debug/subsocial-node .

RUN mv /usr/share/ca* /tmp && \
    rm -rf /usr/share/* && \
    mv /tmp/ca-certificates /usr/share && \
    mkdir -p /root/.local/share/subsocial-node && \
    ln -s /root/.local/share/subsocial-node /data

RUN rm -rf /usr/bin /usr/sbin


EXPOSE 30333 9933 9944
VOLUME ["/data"]

CMD ["./subsocial-node"]