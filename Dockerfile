# Adapted from: https://dev.to/rogertorres/first-steps-with-docker-rust-30oi

FROM rust:latest AS project-builder
# create a new empty shell project
RUN USER=root cargo new --bin rtv_backend
WORKDIR /rtv_backend

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/rtv_backend*
RUN cargo build --release

# our final base
FROM debian:stable-slim

# copy the build artifact from the build stage
COPY --from=project-builder /rtv_backend/target/release/rtv_backend .

# set the startup command to run your binary
CMD ["./rtv_backend"]