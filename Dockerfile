FROM rust:1.40 as builder

WORKDIR /usr/src/backend
COPY . .

RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/backend/target/release/backend /usr/local/bin/
CMD ["backend"]