# This is just here so we can pull stuff from it
FROM node:8.1.0 AS node

# The build environment
FROM rust:1.20.0 AS rust

# Run the rust build first and fail
# That way most of the work is done and we can iterate faster
COPY ./native /native
WORKDIR /native
RUN cargo build --release || true

# Import node into the rust build environment for neon
COPY --from=node /usr/local/bin/node /usr/local/bin/node
COPY --from=node /usr/local/include/node /usr/local/include/node
COPY --from=node /usr/local/lib/node_modules /usr/local/lib/node_modules
COPY --from=node /usr/local/lib/python2.7 /usr/local/lib/python2.7
RUN cd /usr/local/bin && ln -snf ../lib/node_modules/npm/bin/npm-cli.js npm

# Copy the source and write the prebuilt rust in
COPY . /build
RUN mv /native /build/native

# Build!
WORKDIR /build
RUN npm install --unsafe-perm
RUN strip native/target/release/bl

# Clean up build intermediates
RUN rm -r native/target/release/build
RUN rm -r native/target/release/deps
RUN rm -r native/target/release/examples
RUN rm -r native/target/release/incremental
RUN rm -r native/target/release/native


# The runtime environment
FROM node:8.1.0

COPY --from=rust /build /app
WORKDIR /app

# Copy bl tool to the PATH
RUN cp native/target/release/bl /usr/local/bin/
