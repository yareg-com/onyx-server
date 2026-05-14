FROM rust:1.95.0-alpine3.23 AS builder

ENV APP=onyx-server

WORKDIR /usr/src/$APP

COPY Cargo.toml Cargo.lock ./

# Cache dependencies by creating a dummy main.rs to compile dependencies first
# This ensures that if only source code changes, the dependencies remain cached
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/$APP

COPY src ./src
RUN touch src/main.rs && cargo build --release

FROM alpine:3.23.4

ENV APP=onyx-server
ENV CONFIG_PATH=/home/$APP/.$APP
ENV DATA_PATH=/var/lib/$APP

RUN apk --no-cache add curl && \
    adduser --disabled-password --gecos '' $APP && \
    mkdir -p $DATA_PATH && chown -R $APP $DATA_PATH

WORKDIR /home/$APP
USER $APP

RUN mkdir -p $CONFIG_PATH

COPY --from=builder /usr/src/$APP/target/release/$APP /usr/local/bin/$APP

COPY ./docker/config.toml $CONFIG_PATH/config.toml

EXPOSE 8080

ENTRYPOINT ["$APP"]

CMD ["--config", "$CONFIG_PATH/config.toml"]