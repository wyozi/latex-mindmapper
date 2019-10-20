FROM rust:latest as build

WORKDIR /usr/src/latex-mindmapper

COPY Cargo.toml Cargo.lock src/ ./
RUN cargo build --release
RUN cargo install --path .

FROM alpine:latest
COPY --from=build /usr/local/cargo/bin/latex-mindmapper /usr/local/bin/latex-mindmapper
CMD ["/bin/sh", "-c", "latex-mindmapper $1 > mindmap.dot"]