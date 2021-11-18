# 1: Build the package
FROM rust:latest as builder
WORKDIR /usr

# 1a: Prepare for static linking
RUN apt-get update
RUN apt-get full-upgrade -y
RUN apt-get install musl-tools -y
USER root
RUN rustup target add x86_64-unknown-linux-musl

# 1b: Copy files into builder container and create volume to be able to copy files off builder container
COPY /src /usr/server/src
COPY ./Cargo.toml /usr/server/Cargo.toml
RUN mkdir /compile-path
RUN mkdir /compile-path/server

# 1c: Download and compile Rust dependencies (and store as a separate Docker layer)
WORKDIR /usr/server
#RUN cargo install --target x86_64-unknown-linux-musl --path .

# 1d: Build the package using the actual source code
RUN cargo install --target x86_64-unknown-linux-musl --path . --features=vendored
#RUN cargo build --release
RUN cp ./target/x86_64-unknown-linux-musl/release/avalanche_api /compile-path/server/

# Changing the volume from within the Dockerfile: If any build steps change the data within the volume after it has been declared, those changes will be discarded. Thus:
VOLUME /compile-path

# 3: Copy the exe and extra files ("static") to an empty Docker image
#
# // This docker container works with `FROM scratch` to save on image size, though using google cloud run requires that the port be $PORT
# // as seen in `CMD ROCKET_PORT=$PORT`, but I have not figured out how to either compile rocket in a way or to add an enironment variable
# // to a scratch image. Hopefully I will figure that out eventually
FROM alpine:latest
COPY --from=builder /compile-path/server /server
RUN ls /server
USER 1000
CMD ROCKET_PORT=$PORT ./server/avalanche_api