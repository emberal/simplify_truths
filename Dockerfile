# Creates a new cargo project, copies the Cargo.toml and Cargo.lock files to the new project,
# builds the project, and then copies the built binary to a new image.

FROM rust:1.79 as build

RUN USER=root cargo new --bin simplify_truths
WORKDIR /simplify_truths

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./derive ./derive

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/simplify_truths*
RUN cargo build --release

FROM node:20.14.0 as spec

WORKDIR /spec

COPY ./spec .
RUN npm install
RUN USER=root npm install -g @typespec/compiler && npm install -g @redocly/cli
RUN npm run tsp-compile && npm run redoc-build

FROM debian
LABEL authors="Martin Berg Alstad"

# copy the build artifact from the build stage
COPY --from=build /simplify_truths/target/release/simplify_truths .
# copy the generated html file for REDOC documentation
COPY --from=spec /spec/dist/index.html ./openapi/index.html

EXPOSE 8000

CMD ["./simplify_truths"]
