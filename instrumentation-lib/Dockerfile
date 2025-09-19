# Copyright The OpenTelemetry Authors
# SPDX-License-Identifier: Apache-2.0


FROM --platform=${BUILDPLATFORM} docker.io/library/rust:1.90 AS builder
ARG TARGETARCH
ARG TARGETPLATFORM
ARG BUILDPLATFORM

RUN echo Building on ${BUILDPLATFORM} for ${TARGETPLATFORM}

# Install dependencies - cross-compilation setup for ARM64, native otherwise
RUN apt-get update && \
    if [ "${TARGETPLATFORM}" = "linux/arm64" ] && [ "${BUILDPLATFORM}" != "linux/arm64" ] ; then \
        apt-get install --no-install-recommends -y g++-aarch64-linux-gnu=4:14.2.0-1 \
        libc6-dev-arm64-cross=2.41-11cross1 libprotobuf-dev=3.21.12-11 \
        protobuf-compiler=3.21.12-11 ca-certificates=20250419 && \
        rustup target add aarch64-unknown-linux-gnu; \
    else \
        apt-get install --no-install-recommends -y g++=4:14.2.0-1 libc6-dev=2.41-12 \
        libprotobuf-dev=3.21.12-11 protobuf-compiler=3.21.12-11 ca-certificates=20250419; \
    fi

WORKDIR /app/

COPY . /app/

# Build - cross-compile for ARM64 or native build
RUN if [ "${TARGETPLATFORM}" = "linux/arm64" ] && [ "${BUILDPLATFORM}" != "linux/arm64" ] ; then \
        env CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
            CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc \
            CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++ \
        cargo build -r --target aarch64-unknown-linux-gnu && \
        cp /app/target/aarch64-unknown-linux-gnu/release/demo /app/target/release/demo; \
    else \
        cargo build -r; \
    fi


FROM gcr.io/distroless/cc-debian12:nonroot

WORKDIR /app

COPY --from=builder /app/target/release/demo demo

EXPOSE 8081

CMD ["./demo"]
