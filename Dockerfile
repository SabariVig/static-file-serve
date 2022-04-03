FROM  rust:1.59 AS builder

RUN cargo new --bin staticserv

WORKDIR /staticserv

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release --target-dir build

RUN rm src/*.rs

COPY ./src ./src

RUN rm build/release/deps/staticserv*

RUN cargo build --release  --target-dir build

RUN cp build/release/staticserv .

### Scratch 

FROM rust:1.59-bullseye

WORKDIR /app

# RUN apk update && apk --no-cache add ca-certificates openssl libssl1.1 brotli openssl-dev libpq-dev && rm -rf /var/cache/* && mkdir /var/cache/apk

COPY --from=builder /staticserv/staticserv /usr/local/bin/

EXPOSE 3000

CMD ["staticserv"]

