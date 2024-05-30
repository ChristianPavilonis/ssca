FROM rust:1.78-slim-buster as build

WORKDIR /ssca

# this build step will cache your dependencies
# RUN cargo build --release
# RUN rm src/*.rs

# copy your source tree
COPY ./ ./


# build for release
RUN cargo build --release

# our final base
FROM debian:buster-slim

RUN apt-get update -qq && \
	apt-get install --no-install-recommends -y curl && \
	rm -rf /var/lib/apt/lists /var/cache/apt/archives

# copy the build artifact from the build stage
COPY --from=build /ssca/target/release/ssca .
COPY ./public ./public/
RUN mkdir ./db && touch ./db/db.sqlite

EXPOSE 3000

# set the startup command to run your binary
CMD ["./ssca"]
