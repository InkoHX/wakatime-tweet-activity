FROM rust:1.48.0 AS build

WORKDIR /source
COPY . .

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release

#######################

FROM debian:buster-slim

RUN apt update
RUN apt install -y openssl ca-certificates
RUN rm -rf /var/lib/apt/lists/*

COPY --from=build /source/target/release/wakatime-tweet-activity /usr/local/bin/wakatime-tweet-activity

ENV TWITTER_ACCESS_TOKEN=${TWITTER_ACCESS_TOKEN} \
  TWITTER_ACCESS_TOKEN_SECRET=${TWITTER_ACCESS_TOKEN_SECRET} \
  TWITTER_CONSUMER_KEY=${TWITTER_CONSUMER_KEY} \
  TWITTER_CONSUMER_KEY_SECRET=${TWITTER_CONSUMER_KEY_SECRET} \
  WAKATIME_API_KEY=${WAKATIME_API_KEY}

ENTRYPOINT [ "wakatime-tweet-activity" ]
