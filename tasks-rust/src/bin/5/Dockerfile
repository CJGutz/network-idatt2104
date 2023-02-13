FROM rust:alpine3.17

WORKDIR /code
COPY . .

RUN cargo build --release --bin 5

EXPOSE 8080

CMD [ "./target/release/5"]
