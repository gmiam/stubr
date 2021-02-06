FROM ekidd/rust-musl-builder:latest as builder
COPY . stubr
WORKDIR stubr
RUN cargo build --release

FROM scratch
COPY --from=builder stubr/target/release/stubr .
CMD ls -l .
#ENTRYPOINT ["stubr"]