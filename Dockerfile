FROM rust:latest as build

WORKDIR /usr/src/latex-mindmapper

COPY . ./
RUN cargo build --release
RUN cargo install --path .

FROM debian:stretch-slim
COPY --from=build /usr/local/cargo/bin/latex-mindmapper /usr/local/bin/latex-mindmapper
ENTRYPOINT ["latex-mindmapper"]