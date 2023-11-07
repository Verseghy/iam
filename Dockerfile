FROM rust:slim-bookworm as builder

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
       target/release/deps/libiam* && \
    cargo build --release

FROM debian:12-slim
WORKDIR /app
COPY --from=builder /builder/app/target/release/iam ./
EXPOSE 3001

RUN addgroup --system app && \
    adduser --system --disabled-password --no-create-home --shell /bin/false --ingroup app app && \
    chown -R app:app /app
USER app

CMD ["./iam"]

