# Need userns-remap, run with --tmpfs /run
FROM registry.access.redhat.com/ubi9-init:9.3-8

ARG SB_USER="user"
ARG SB_UID="1002"
ARG SB_GID="1002"

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN set -eux; \
    useradd --no-log-init --create-home --shell /bin/bash --uid "${SB_UID}" "${SB_USER}"; \
    mkdir /root/work; \
    url='https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init'; \
    curl --proto '=https' --tlsv1.2 $url -o rustup-init; \
    chmod u+x rustup-init; \
    ./rustup-init -y --no-modify-path --profile minimal; \
    rm rustup-init; \
    rustup --version; \
    cargo --version; \
   rustc --version;

CMD ["/sbin/init"]