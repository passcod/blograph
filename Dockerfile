FROM node:6-alpine
EXPOSE 5000

#RUN apt update && apt install -y build-essential curl git sudo
RUN apk add --update --no-cache build-base curl file git sudo
RUN curl -sL https://static.rust-lang.org/rustup.sh -o /rustup.sh \
    && sh /rustup.sh --prefix=/usr/local --spec=stable -y \
    && rustc --version \
    && cargo --version \
    && mkdir /app

ENV NODE_ENV production
WORKDIR /app

COPY package.json .
RUN npm install --no-scripts

COPY native ./native
RUN npm run install

COPY app index.js lib public views ./
ENV RUST_LOG info
