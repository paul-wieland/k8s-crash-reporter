FROM rust:1.81

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["k8s-crash-reporter"]

