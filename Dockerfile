FROM ubuntu:25.10

RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates

COPY ./target/release/roulette /opt/roulette

EXPOSE 3000
CMD ["/opt/roulette"]