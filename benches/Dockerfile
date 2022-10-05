# Benchmarks for tldrx
#
# Usages:
#    docker build -t benchmark .
#    docker run --privileged --rm -it benchmark
#


#
FROM rust AS tldrx-bulid

RUN git clone https://github.com/tldrx/tldrx.git \
    && cd tldrx \
    && cargo build --release \
    && mkdir -p /clients \
    && cp target/release/tldrx /clients/tldrx


#
FROM rust AS tealdeer-bulid

RUN git clone https://github.com/dbrgn/tealdeer.git \
    && cd tealdeer \
    && cargo build --release \
    && mkdir -p /clients \
    && cp target/release/tldr /clients/tealdeer


# 
FROM euantorano/zig:0.9.1 AS outfieldr-bulid

RUN apk add git
RUN git clone https://gitlab.com/ve-nt/outfieldr.git \
    && cd outfieldr \
    && git submodule update --init \
    && zig build -Drelease-safe\
    && mkdir -p /clients \
    && cp bin/tldr /clients/outfieldr


#
FROM ubuntu:22.04 AS tool

RUN apt-get update && apt-get install -y build-essential git curl
RUN curl -OL https://github.com/sharkdp/hyperfine/releases/download/v1.15.0/hyperfine_1.15.0_amd64.deb \
    && dpkg -i hyperfine_1.15.0_amd64.deb


#
FROM tool AS tldrc-build

RUN git clone https://github.com/tldr-pages/tldr-c-client.git \
    && cd tldr-c-client \
    && DEBIAN_FRONTEND=noninteractive ./deps.sh \
    && CFLAGS=-O3 make \
    && mkdir -p /clients \
    && cp deps.sh /clients/tldrc-deps.sh \
    && cp tldr /clients/tldrc


#
FROM tool AS benchmark

RUN mkdir -p /clients
RUN mkdir -p ~/.local/share/outfieldr

COPY --from=tldrx-bulid           /clients/* /clients/
COPY --from=tealdeer-bulid        /clients/* /clients/
COPY --from=outfieldr-bulid       /clients/* /clients/
COPY --from=tldrc-build           /clients/* /clients/

RUN cd /clients && DEBIAN_FRONTEND=noninteractive ./tldrc-deps.sh
RUN cd /clients\
    && ./tldrx -u \
    && ./outfieldr -u \
    && ./tealdeer -u \
    && ./tldrc -u

CMD hyperfine --warmup 10 --runs 100 --prepare 'sync; echo 3 | tee /proc/sys/vm/drop_caches' \
    "/clients/outfieldr c99" \
    "/clients/tldrx c99" \
    "/clients/tealdeer c99" \
    "/clients/tldrc c99"
