FROM rust:alpine as builder

RUN apk add musl-dev
WORKDIR /builder
RUN cargo new --bin app && \
    cargo new --bin app/iam && \
    cargo new --lib app/iam-entity && \
    cargo new --lib app/iam-migration && \
    cargo new --lib app/iam-common && \
    cargo new --lib app/iam-macros && \
    cargo new --bin app/cmds

WORKDIR /builder/app

COPY ["Cargo.toml", "Cargo.lock", "./"]
COPY ./iam-migration/Cargo.toml ./iam-migration/Cargo.toml
COPY ./iam-entity/Cargo.toml ./iam-entity/Cargo.toml
COPY ./iam-common/Cargo.toml ./iam-common/Cargo.toml
COPY ./iam-macros/Cargo.toml ./iam-macros/Cargo.toml

RUN rm ./iam-macros/src/lib.rs && \
    touch ./iam-macros/src/lib.rs && \
    cargo build -p migration --release && \
    rm -rf ./iam-migration/src \
           ./iam-entity/src/ \
	   ./iam-common/src/

COPY ./iam-migration/src/ ./iam-migration/src/
COPY ./iam-entity/src/ ./iam-entity/src/
COPY ./iam-common/src/ ./iam-common/src/
COPY ./iam-macros/src/ ./iam-macros/src/

RUN rm target/release/deps/iam_migration* \
       target/release/deps/iam_entity* \
       target/release/deps/libiam_entity* \
       target/release/deps/iam_common* \
       target/release/deps/libiam_common* \
       target/release/deps/iam_macros* \
       target/release/deps/libiam_macros* && \
    cargo build -p migration --release

FROM alpine
WORKDIR /app
COPY --from=builder /builder/app/target/release/migration ./
EXPOSE 3001
CMD ["./migration"]

