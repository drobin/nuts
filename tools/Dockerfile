# syntax=docker/dockerfile:1

FROM rust:1.75 as nuts-build

ARG BRANCH=master

ENV CARGO_TARGET_DIR=/target

WORKDIR /src

ADD --keep-git-dir=true https://github.com/drobin/nuts.git /src

RUN git fetch --tags && git checkout "$BRANCH"

RUN --mount=type=cache,target=/root/.cargo cargo build --features=plugin
RUN --mount=type=cache,target=/root/.cargo cargo test --all-features
RUN --mount=type=cache,target=/root/.cargo cargo build --release  --features=plugin

FROM debian:12

COPY --from=nuts-build /target/release/nuts /target/release/nuts-directory /usr/local/bin/

RUN /usr/local/bin/nuts plugin add directory --path nuts-directory
