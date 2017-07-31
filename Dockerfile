FROM alpine:3.6
RUN apk add --no-cache cargo rust git docker

# Clone the registry binary and put it in `/`
ARG REGISTRY_COMMIT=15dbd1a011dbda8da055d6cea8bc1e7705c32ead
RUN git clone https://github.com/docker/distribution-library-image /home/registry && \
    cd /home/registry && \
    git reset --hard $REGISTRY_COMMIT && \
    cp ./registry/registry / && \
    cd /
COPY registry-config.yml /

# Compiling the proxy
RUN mkdir -p /home/rust
COPY proxy /home/rust/
RUN cargo install --debug --path=/home/rust
RUN rm -rf /home/rust

# Prepare the git repository
RUN mkdir -p /var/git && \
    git init --bare /var/git && \
    git config --global --bool http.receivepack true
COPY hook /var/git/hooks
RUN mv /var/git/hooks/hook /var/git/hooks/update

# Compiling the hook binary
RUN mkdir -p /home/rust
COPY hook-bin /home/rust/
RUN cargo install --debug --path=/home/rust
RUN rm -rf /home/rust


EXPOSE 80
ENV PATH=/root/.cargo/bin:/usr/local/musl/bin:/usr/local/bin:/usr/bin:/bin
CMD proxy
