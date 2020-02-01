FROM rust:1.41.0 as builder

WORKDIR /usr/src/project

COPY . .

RUN cargo install --path .

ENTRYPOINT ["mysqlpinger"]

