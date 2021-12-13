# This is the build stage for Cycan. Here we create the binary.
FROM docker.io/paritytech/ci-linux:production as builder

WORKDIR /cycan
COPY . /cycan
RUN cargo build --release

# This is the 2nd stage: a very small image where we copy the Cycan binary."
FROM docker.io/library/ubuntu:20.04
LABEL description="Multistage Docker image for Cycan: a platform for web3" \
	io.cycan.image.type="builder" \
	io.cycan.image.authors="devops-team@cycan.io" \
	io.cycan.image.vendor="Cycan Technologies" \
	io.cycan.image.description="Cycan" \
	io.cycan.image.source="https://github.com/paritytech/polkadot/blob/${VCS_REF}/docker/substrate_builder.Dockerfile" \
	io.cycan.image.documentation="https://github.com/paritytech/polkadot/"

COPY --from=builder /cycan/target/release/cycan /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /cycan cycan && \
	mkdir -p /data /cycan/.local/share && \
	chown -R cycan:cycan /data && \
	ln -s /data /cycan/.local/share/cycan && \
# unclutter and minimize the attack surface
#	rm -rf /usr/bin /usr/sbin && \
# check if executable works in this container
	/usr/local/bin/cycan --version

USER cycan
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/cycan"]
