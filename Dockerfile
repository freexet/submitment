FROM rust:alpine as build

RUN USER=root cargo new --bin submitment
WORKDIR /submitment

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/submitment*
RUN cargo build --release

FROM alpine:latest

COPY --from=build /submitment/target/release/submitment .

CMD ['./submitment']
