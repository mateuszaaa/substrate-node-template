FROM debian:buster

ARG BUILD_DIR=target/release

WORKDIR /mangata

COPY $BUILD_DIR/mangata-node /mangata/node
COPY $BUILD_DIR/wbuild/mangata-polkadot-runtime/mangata_polkadot_runtime.compact.compressed.wasm /mangata/

ENTRYPOINT ["/mangata/node"]
