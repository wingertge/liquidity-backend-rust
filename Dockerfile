
ARG BINARY_NAME_DEFAULT=backend
ARG MY_GREAT_CONFIG_DEFAULT="someconfig-default-value"

FROM clux/muslrust:stable as builder
RUN groupadd -g 10001 -r dockergrp && useradd -r -g dockergrp -u 10001 dockeruser
ARG BINARY_NAME_DEFAULT
ENV BINARY_NAME=$BINARY_NAME_DEFAULT
# Build the project with target x86_64-unknown-linux-musl


# Now add the rest of the project and build the real main
COPY . ./
RUN set -x && cargo build --target x86_64-unknown-linux-musl --release
RUN mkdir -p /build-out
RUN set -x && cp target/x86_64-unknown-linux-musl/release/$BINARY_NAME /build-out/

# Create a minimal docker image
FROM scratch

COPY --from=0 /etc/passwd /etc/passwd
USER dockeruser

ARG BINARY_NAME_DEFAULT
ENV BINARY_NAME=$BINARY_NAME_DEFAULT
ARG MY_GREAT_CONFIG_DEFAULT
ENV MY_GREAT_CONFIG=$MY_GREAT_CONFIG_DEFAULT

ENV RUST_LOG="error,$BINARY_NAME=info"
COPY --from=builder /build-out/$BINARY_NAME /

# Start with an execution list (there is no sh in a scratch image)
# No shell => no variable expansion, |, <, >, etc
# Hard coded start command
CMD ["backend"]