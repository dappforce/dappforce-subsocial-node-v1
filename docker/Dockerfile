FROM rust:1.36 as builder

RUN curl https://getsubstrate.io -sSf | bash -s -- --fast

RUN git clone -b df-pre-ipfs https://github.com/dappforce/dappforce-subsocial-node.git
WORKDIR /dappforce-subsocial-node
RUN git clone -b df-pre-ipfs https://github.com/dappforce/dappforce-subsocial-runtime.git

RUN ./init-wasm.sh

WORKDIR /dappforce-subsocial-node/dappforce-subsocial-runtime
RUN ./build.sh

WORKDIR /dappforce-subsocial-node
RUN cargo build

FROM rust:1.36

COPY --from=builder /dappforce-subsocial-node/target/debug/joystream-node /usr/local/bin

RUN mv /usr/share/ca* /tmp && \
    rm -rf /usr/share/* && \
    mv /tmp/ca-certificates /usr/share && \
    mkdir -p /root/.local/share/joystream-node && \
    ln -s /root/.local/share/joystream-node /data

RUN rm -rf /usr/bin /usr/sbin


EXPOSE 30333 9933 9944
VOLUME ["/data"]

CMD ["/usr/local/bin/joystream-node"]