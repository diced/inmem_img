FROM rust:1.51 as builder

RUN USER=root cargo new --bin inmem_img
WORKDIR /inmem_img
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/inmem_img*
RUN cargo build --release


FROM debian:buster-slim

COPY --from=builder /inmem_img/target/release/inmem_img /opt/inmem_img/inmem_img
WORKDIR /opt/inmem_img

CMD ["./inmem_img"]