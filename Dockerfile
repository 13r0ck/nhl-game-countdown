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
RUN cargo install --target x86_64-unknown-linux-musl --path . --features=vendored
RUN cp ./target/x86_64-unknown-linux-musl/release/nhl-game-countdown /compile-path/server/

# Changing the volume from within the Dockerfile: If any build steps change the data within the volume after it has been declared, those changes will be discarded. Thus:
VOLUME /compile-path

# 3: Copy the package and extra files to an alpine Docker image
FROM alpine:latest as runner
COPY --from=builder /compile-path/server /server
RUN apk add tzdata
# This is specific to the location that my server is hosted. You will need to
# change the timezone here to the location where the server is hosted.
# https://wiki.alpinelinux.org/wiki/Setting_the_timezone
RUN cp /usr/share/zoneinfo/America/Mexico_City /etc/localtime
RUN echo "America/Mexico_City" >  /etc/timezone
RUN apk del tzdata
USER 1000
CMD ROCKET_PORT=$PORT ROCKET_ADDRESS="0.0.0.0" ./server/nhl-game-countdown