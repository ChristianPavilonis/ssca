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

# copy the build artifact from the build stage
COPY --from=build /ssca/target/release/ssca .
COPY ./public ./public/
RUN mkdir ./db && touch ./db/db.sqlite

# set the startup command to run your binary
CMD ["./ssca"]
