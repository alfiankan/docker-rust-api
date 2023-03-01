FROM rust:latest
WORKDIR /tool-chain
COPY src /tool-chain/src
COPY Cargo.toml /tool-chain/Cargo.toml

#RUN cargo build --target-dir /tool-chain/dist

#ENTRYPOINT ["sleep", "400000"]