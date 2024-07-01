FROM rust:latest as build

# create a new empty shell project
RUN USER=root cargo new --bin backend
WORKDIR /backend

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# this build step will cache your dependencies
VOLUME [ "/rust_shared" ]
COPY /rust_shared/parsers ./parsers

RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/backend*
RUN cargo build --release

# our final base
FROM rust:latest

# copy the build artifact from the build stage
COPY --from=build /backend/target/release/backend .
COPY --from=build /backend/parsers /rust_shared/parsers

# set the startup command to run your binary
CMD ["./backend"]
