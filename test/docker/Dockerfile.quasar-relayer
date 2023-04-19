# syntax=docker/dockerfile:1

ARG RELAYER_VERSION="latest"
ARG BASE_IMAGE=""

# --------------------------------------------------------
# Repack relayer with startup script
# --------------------------------------------------------


FROM ghcr.io/cosmos/relayer:latest AS relayer-bin


FROM ubuntu:22.04 as dev


ENV HOME /relayer
WORKDIR $HOME

ENV PACKAGES jq

RUN rm -f /etc/apt/apt.conf.d/docker-clean
RUN --mount=type=cache,target=/var/cache/apt \
	apt-get update && \
	apt-get install -y $PACKAGES


COPY --from=relayer-bin /bin/rly /bin/rly

COPY bootstrap-scripts/entrypoint.sh /relayer/entrypoint.sh
COPY bootstrap-scripts/relayer_localnet.sh /relayer/app_init.sh

COPY bootstrap-scripts/keys/ ./keys/ 
COPY bootstrap-scripts/go-relayer-config/ ./go-relayer-config/ 

RUN chmod +x entrypoint.sh && chmod +x app_init.sh && mkdir logs

EXPOSE 26656
EXPOSE 26657
EXPOSE 1317

CMD ["rly start"]
ENTRYPOINT ["./entrypoint.sh"]
