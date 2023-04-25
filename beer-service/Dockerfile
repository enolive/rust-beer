FROM rust:1.63 as builder
WORKDIR /usr/src/app
RUN cargo init
ADD Cargo.toml Cargo.lock ./
RUN cargo build
# Real build
COPY ./src ./src
RUN cargo build --release

FROM ubuntu:latest
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/beer-service .
RUN chmod +x beer-service
EXPOSE 8080
CMD ["/app/beer-service"]
