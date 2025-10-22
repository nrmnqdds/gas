FROM rust:bookworm AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY proto ./proto
COPY build.rs ./

RUN apt update && apt install -y protobuf-compiler libprotobuf-dev

RUN cargo build --release --locked

FROM gcr.io/distroless/cc-debian12 AS runner

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/gas ./

ENV HOSTNAME=0.0.0.0
ENV RUST_LOG=info
ENV OUT_DIR=./

EXPOSE 50052

CMD ["./gas"]
