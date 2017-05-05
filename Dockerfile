FROM base/devel
EXPOSE 5000

RUN pacman -Sy --noconfirm cargo git nodejs python2 npm rust \
    && npm i -g npm5 \
    && set -x \
    && rustc --version \
    && cargo --version \
    && node --version \
    && npm5 --version

RUN useradd -d /app -G users -mrU app
ENV NODE_ENV production
WORKDIR /app

COPY index.js package.json package-lock.json ./
COPY native ./native
RUN chown -R app:app . \
    && sudo -u app npm5 install

COPY app ./app
COPY lib ./lib
COPY public ./public
COPY views ./views
RUN chown -R app:app .

USER app
CMD ["npm", "start", "-s"]
