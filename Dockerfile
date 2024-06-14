# Creates a new cargo project, copies the Cargo.toml and Cargo.lock files to the new project,
# builds the project, and then copies the built binary to a new image.

FROM rust:1.79 as build
LABEL authors="Martin Berg Alstad"

RUN USER=root cargo new --bin simplify_truths
WORKDIR /simplify_truths

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/simplify_truths*
RUN cargo build --release

FROM debian

# copy the build artifact from the build stage
COPY --from=build /simplify_truths/target/release/simplify_truths .

EXPOSE 8000

CMD ["./simplify_truths"]
