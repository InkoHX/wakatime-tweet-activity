FROM rust:1.46 AS build

WORKDIR /source
COPY . .

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release

#######################

FROM debian:buster-slim

RUN apt update
RUN apt install -y openssl
RUN rm -rf /var/lib/apt/lists/*

COPY --from=build /source/target/release/wakatime-tweet-activity /usr/local/bin/wakatime-tweet-activity

ENTRYPOINT [ "wakatime-tweet-activity" ]
