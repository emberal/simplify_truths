FROM rust:1.78
LABEL authors="Martin Berg Alstad"

COPY ./ ./

EXPOSE 8000

RUN cargo build --release

CMD ["./target/release/simplify_truths"]
