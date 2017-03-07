FROM node:6
EXPOSE 5000

RUN apt update && apt install -y build-essential curl git sudo
RUN curl -sL https://static.rust-lang.org/rustup.sh -o /rustup.sh \
    && sh /rustup.sh --spec=stable -y \
    && rustc --version \
    && cargo --version \
    && mkdir /app

ENV NODE_ENV production

COPY . /app/
WORKDIR /app
RUN npm install && npm run install

ENV RUST_LOG info
