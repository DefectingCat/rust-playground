ARG RUST_VERSION=1.82.0
ARG APP_NAME=rust-playground

FROM node:lts-slim AS ui
WORKDIR /app

COPY ui/frontend/ .
RUN --mount=type=cache,target=/app/node_modules \
  <<EOF
set -e
ls -al
pwd
npm i -g pnpm
pnpm install --frozen-lockfile
pnpm build
mkdir -p /bin/ui/
cp -r build/ /bin/ui/
EOF

FROM rust:${RUST_VERSION}-slim-bullseye AS build
WORKDIR /app

COPY . .
RUN --mount=type=cache,target=/app/target/ \
  --mount=type=cache,target=/usr/local/cargo/registry/ \
  <<EOF
set -e
cd ui
cargo build --locked --release
cp ./target/release/ui /bin/server
EOF

FROM debian:bullseye-slim AS final

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/develop/develop-images/dockerfile_best-practices/   #user
ARG UID=10001
RUN adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-home \
  --uid "${UID}" \
  appuser
USER appuser

ENV PLAYGROUND_UI_ROOT=/bin/ui/build

COPY --from=build /bin/server /bin/
COPY --from=ui /bin/ui /bin/ui

EXPOSE 5000

CMD ["/bin/server"]
