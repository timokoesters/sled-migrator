# Using multistage build:
# 	https://docs.docker.com/develop/develop-images/multistage-build/
# 	https://whitfin.io/speeding-up-rust-docker-builds/


##########################  BUILD IMAGE  ##########################
# Musl build image to build portals statically compiled binary
FROM rust:1.47.0-alpine3.12 as builder

# Don't download Rust docs
RUN rustup set profile minimal

ENV USER "migrator"
ENV RUSTFLAGS='-C link-arg=-s'


# Install packages needed for building all crates
RUN apk add --no-cache \
        musl-dev

# Specifies if the local project is build or if Conduit gets build
# from the official git repository. Defaults to the git repo.
ARG LOCAL=false
# Specifies which revision/commit is build. Defaults to HEAD
ARG GIT_REF=origin/master

# Add musl target, as we want to run our project in
# an alpine linux image
#RUN rustup target add x86_64-unknown-linux-musl

# Copy project files from current folder
COPY . .
# Build it from the copied local files or from the official git repository
RUN if [[ $LOCAL == "true" ]]; then \
        cargo install --path . ; \
    else \
        cargo install --git "https://github.com/Weasy666/sled-migrator.git" --rev ${GIT_REF}; \
    fi

########################## RUNTIME IMAGE ##########################
# Create new stage with a minimal image for the actual
# runtime image/container
FROM alpine:3.12

ARG CREATED
ARG VERSION
ARG GIT_REF=HEAD

# Labels according to https://github.com/opencontainers/image-spec/blob/master/annotations.md
# including a custom label specifying the build command
LABEL org.opencontainers.image.created=${CREATED} \
      org.opencontainers.image.authors="Conduit Contributors" \
      org.opencontainers.image.title="Conduit" \
      org.opencontainers.image.version=${VERSION} \
      org.opencontainers.image.vendor="Conduit Contributors" \
      org.opencontainers.image.description="A migration tool to upgrade Conduit's Sled database" \
      org.opencontainers.image.url="https://conduit.rs/" \
      org.opencontainers.image.revision=${GIT_REF} \
      org.opencontainers.image.source="https://github.com/timokoesters/sled-migrator.git" \
      org.opencontainers.image.licenses="AGPL-3.0-only" \
      org.opencontainers.image.documentation="" \
      org.opencontainers.image.ref.name="" \
      org.label-schema.docker.build="docker build . -t matrixconduit/sled-migrator:latest --build-arg CREATED=$(date -u +'%Y-%m-%dT%H:%M:%SZ') --build-arg VERSION=$(grep -m1 -o '[0-9].[0-9].[0-9]' Cargo.toml)" \
      maintainer="Weasy666"

# Copy config files from context and the binary from
# the "builder" stage to the current stage into folder
# /srv/conduit and create data folder for database
RUN mkdir -p /srv/conduit/.local/share/conduit
COPY --from=builder /usr/local/cargo/bin/sled-migrator /srv/conduit/

# Add www-data user and group with UID 82, as used by alpine
# https://git.alpinelinux.org/aports/tree/main/nginx/nginx.pre-install
RUN set -x ; \
    addgroup -Sg 82 www-data 2>/dev/null ; \
    adduser -S -D -H -h /srv/conduit -G www-data -g www-data www-data 2>/dev/null ; \
    addgroup www-data www-data 2>/dev/null && exit 0 ; exit 1

# Change ownership of Conduit files to www-data user and group
RUN chown -cR www-data:www-data /srv/conduit

# Set user to www-data
USER www-data
# Set container home directory
WORKDIR /srv/conduit
