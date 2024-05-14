# syntax=docker/dockerfile:1

FROM rust:1.75 as nuts-build

ARG BRANCH=master

ENV CARGO_TARGET_DIR=/target

WORKDIR /src

ADD --keep-git-dir=true https://github.com/drobin/nuts.git /src

RUN git fetch --tags
RUN git checkout "$BRANCH"

RUN --mount=type=cache,target=/root/.cargo cargo build
RUN --mount=type=cache,target=/root/.cargo cargo test
RUN --mount=type=cache,target=/root/.cargo cargo build --release

FROM debian:12

RUN apt-get -y update
RUN apt-get -y install libssl3
RUN apt-get clean

COPY --from=nuts-build /target/release/nuts /usr/local/bin/nuts
