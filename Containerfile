FROM docker.io/library/ubuntu:22.04

# show backtraces
ENV RUST_BACKTRACE 1

# install tools and dependencies
RUN apt-get update && \
	DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
		ca-certificates && \
# apt cleanup
	apt-get autoremove -y && \
	apt-get clean && \
	find /var/lib/apt/lists/ -type f -not -name lock -delete; \
# add user and link ~/.local/share/polkadot to /data
	useradd -m -u 1000 -U -s /bin/sh -d /polkadot polkadot && \
	mkdir -p /data /polkadot/.local/share && \
	chown -R polkadot:polkadot /data && \
	ln -s /data /polkadot/.local/share/parachain-template-node && \
	mkdir -p /specs

USER polkadot

# copy the compiled binary to the container
COPY --chown=polkadot:polkadot --chmod=774 parachain-template-node /usr/bin/parachain-template-node

# check if executable works in this container
RUN /usr/bin/parachain-template-node --version

# ws_port
EXPOSE 9333 9944 30333 30334

CMD ["/usr/bin/parachain-template-node"]
