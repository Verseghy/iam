FROM rust:alpine as builder

RUN apk add musl-dev
WORKDIR /builder
RUN cargo new --bin app && \
    cargo new --bin app/iam && \
    cargo new --lib app/entity && \
    cargo new --lib app/migration && \
    cargo new --lib app/common && \
    cargo new --bin app/cmds && \
    cargo new --lib app/macros

WORKDIR /builder/app

COPY ["Cargo.toml", "Cargo.lock", "./"]
COPY ./iam/Cargo.toml ./iam/Cargo.toml
COPY ./entity/Cargo.toml ./entity/Cargo.toml
COPY ./common/Cargo.toml ./common/Cargo.toml
COPY ./macros/Cargo.toml ./macros/Cargo.toml

RUN rm ./macros/src/lib.rs && \
    touch ./macros/src/lib.rs && \
    cargo build --release && \
    rm -rf ./iam/src/ \
           ./entity/src/ \
	   ./common/src/ \
	   ./macros/src/

COPY ./iam/src/ ./iam/src/
COPY ./entity/src/ ./entity/src/
COPY ./common/src/ ./common/src/
COPY ./macros/src/ ./macros/src/

RUN rm target/release/deps/iam* \
       target/release/deps/entity* \
       target/release/deps/libentity* \
       target/release/deps/common* \
       target/release/deps/libcommon* \
       target/release/deps/macros* \
       target/release/deps/libmacros* && \
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

