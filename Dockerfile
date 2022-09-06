FROM rust:alpine as builder

RUN apk add musl-dev
WORKDIR /builder
RUN cargo new --bin app && \
    cargo new --bin app/iam && \
    cargo new --lib app/entity && \
    cargo new --lib app/migration && \
    cargo new --lib app/common && \
    cargo new --bin app/seeder

WORKDIR /builder/app

COPY ["Cargo.toml", "Cargo.lock", "./"]
COPY ./iam/Cargo.toml ./iam/Cargo.toml
COPY ./entity/Cargo.toml ./entity/Cargo.toml
COPY ./common/Cargo.toml ./common/Cargo.toml

RUN cargo build --release && \
    rm -rf ./iam/src/ \
           ./entity/src/ \
	   ./common/src/

COPY ./iam/src/ ./iam/src/
COPY ./entity/src/ ./entity/src/
COPY ./common/src/ ./common/src/

RUN rm target/release/deps/iam* \
       target/release/deps/entity* \
       target/release/deps/libentity* \
       target/release/deps/common* \
       target/release/deps/libcommon* && \
    cargo build --release

FROM alpine
WORKDIR /app
COPY --from=builder /builder/app/target/release/iam ./
EXPOSE 3001
CMD ["./iam"]

