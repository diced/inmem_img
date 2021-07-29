FROM rust:1.51 as builder

RUN USER=root cargo new --bin inmem_img
WORKDIR /inmem_img
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/inmem_img*
RUN cargo build --release


FROM alpine
ARG APP=/opt/inmem_img

EXPOSE 8000

ENV APP_USER=inmem_img

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /inmem_img/target/release/inmem_img ${APP}/inmem_img

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./inmem_img"]