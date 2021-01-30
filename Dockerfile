FROM scratch
COPY target/release/stubr .
ENTRYPOINT ["stubr"]