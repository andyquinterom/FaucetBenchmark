FROM rust:slim-bullseye AS builder
WORKDIR /usr/src/plumbmark
COPY . .
RUN cargo build --release

FROM nginx:stable-bullseye
COPY --from=builder /usr/src/plumbmark/target/release/plumbmark /usr/local/bin/plumbmark
