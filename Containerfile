FROM docker.io/library/ubuntu:22.04

# show backtraces
ENV RUST_BACKTRACE 1

# install tools and dependencies
RUN apt-get update && \
	DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
		ca-certificates && \
  useradd -m -u 1000 -U -s /bin/sh -d /parachain-template parachain-template && \
# apt cleanup
	apt-get autoremove -y && \
	apt-get clean && \
	rm -rf /var/lib/apt/lists/* && \
	mkdir -p /data /parachain-template/.local/share && \
	chown -R parachain-template:parachain-template /data && \
	ln -s /data /parachain-template/.local/share/parachain-template

USER parachain-template

# copy the compiled binary to the container
COPY --chown=parachain-template:parachain-template --chmod=774 parachain-template /usr/bin/parachain-template

# check if executable works in this container
RUN /usr/bin/parachain-template --version

# ws_port
EXPOSE 9930 9333 9944 30333 30334

VOLUME ["/parachain-template"]

ENTRYPOINT ["/usr/bin/parachain-template"]
