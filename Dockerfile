FROM rust:alpine as builder

RUN apk add musl-dev
WORKDIR /builder
RUN cargo new --bin app && \
    cargo new --bin app/iam && \
    cargo new --lib app/iam-entity && \
    cargo new --lib app/iam-migration && \
    cargo new --lib app/iam-common && \
    cargo new --lib app/iam-macros && \
    cargo new --lib app/libiam && \
    cargo new --bin app/cmds

WORKDIR /builder/app

COPY ["Cargo.toml", "Cargo.lock", "./"]
COPY ./iam/Cargo.toml ./iam/Cargo.toml
COPY ./iam-entity/Cargo.toml ./iam-entity/Cargo.toml
COPY ./iam-common/Cargo.toml ./iam-common/Cargo.toml
COPY ./iam-macros/Cargo.toml ./iam-macros/Cargo.toml

RUN rm ./iam-macros/src/lib.rs && \
    touch ./iam-macros/src/lib.rs && \
    cargo build --release && \
    rm -rf ./iam/src/ \
           ./iam-entity/src/ \
	   ./iam-common/src/ \
	   ./iam-macros/src/

COPY ./iam/src/ ./iam/src/
COPY ./iam-entity/src/ ./iam-entity/src/
COPY ./iam-common/src/ ./iam-common/src/
COPY ./iam-macros/src/ ./iam-macros/src/

RUN rm target/release/deps/iam* \
       target/release/deps/iam_entity* \
       target/release/deps/libiam_entity* \
       target/release/deps/iam_common* \
       target/release/deps/libiam_common* \
       target/release/deps/iam_macros* \
       target/release/deps/libiam_macros* && \
    cargo build --release

FROM alpine
WORKDIR /app
COPY --from=builder /builder/app/target/release/iam ./
EXPOSE 3001

RUN addgroup -S iam && \
    adduser -S -D -H -s /bin/false -G iam iam && \
    chown -R iam:iam /app
USER iam

CMD ["./iam"]

