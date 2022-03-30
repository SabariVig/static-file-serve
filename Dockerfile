FROM --platform=$BUILDPLATFORM rust:1.59 AS builder

ARG TARGETPLATFORM

RUN case "$TARGETPLATFORM" in \
  "linux/amd64") echo x86_64-unknown-linux-musl > /rust_target.txt ;; \
  "linux/arm/v7") echo armv7-unknown-linux-musleabihf > /rust_target.txt ;; \
  "linux/arm64") echo armv7-unknown-linux-musleabihf > /rust_target.txt ;; \
  "linux/arm/v6") echo arm-unknown-linux-musleabihf > /rust_target.txt ;; \
  *) exit 1 ;; \
  esac

ENV RUST_TARGET $(cat /rust_target.txt)

RUN rustup target add $(cat /rust_target.txt)

RUN cargo new --bin staticserv

WORKDIR /staticserv

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release --target=$(cat /rust_target.txt)

RUN rm src/*.rs

COPY ./src ./src

RUN rm target/$(cat /rust_target.txt)/release/deps/staticserv*

RUN cargo build --release  --target $(cat /rust_target.txt)

RUN cp target/$(cat /rust_target.txt)/release/staticserv .

### Scratch 

FROM --platform=$BUILDPLATFORM scratch

WORKDIR /app

# RUN apk --no-cache add ca-certificates

COPY --from=builder /staticserv/staticserv /usr/local/bin/

EXPOSE 3000

CMD ["staticserv"]

