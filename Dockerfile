FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev openssl build-essential libssl-dev pkg-config
RUN update-ca-certificates

# Create appuser
ENV USER=app
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /app

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################

FROM alpine:latest

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /app

# Copy our build
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/m183-totp ./
COPY --from=builder /app/templates/ ./templates/
RUN ls -lha ./templates

# Use an unprivileged user.
USER app:app

ENV ROCKET_ADDRESS="0.0.0.0"
ENV ROCKET_IDENT=M183TOTP

EXPOSE 8000

CMD ["/app/m183-totp"]
