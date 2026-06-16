# docker build -t chat . && docker run --rm -p 3000:3000 --init chat
####################################################################################################
## Build
####################################################################################################
FROM rust:alpine AS builder

RUN apk update && \
    apk upgrade --no-cache && \
    apk add --no-cache lld musl musl-dev clang git make ca-certificates

WORKDIR /app
COPY . ./
RUN cargo build --release

####################################################################################################
## This stage is used to get the correct files into the final image
####################################################################################################
FROM alpine:latest AS files

# mailcap is used for content type (MIME type) detection
# tzdata is used for timezones info
RUN apk update && \
    apk upgrade --no-cache && \
    apk add --no-cache ca-certificates mailcap tzdata

RUN update-ca-certificates

ENV USER=chat
ENV UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


####################################################################################################
## Final image
####################################################################################################
FROM scratch

# /etc/mime.types may be used to detect the MIME type of files
COPY --from=files --chmod=444 \
    /etc/passwd \
    /etc/group \
    /etc/mime.types \
    /etc/

COPY --from=files --chmod=444 /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=files --chmod=444 /usr/share/zoneinfo /usr/share/zoneinfo

COPY --from=builder /app/target/release/chat /bin/chat

# Use an unprivileged user.
USER chat:chat

WORKDIR /app

ENTRYPOINT ["/bin/chat"]
