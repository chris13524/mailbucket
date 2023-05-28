FROM rust:1.67

WORKDIR /usr/src/mailbucket
COPY . .

RUN cargo install --path .

CMD ["mailbucket"]
