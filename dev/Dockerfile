# Native binary built from the locked workspace source by scripts/platform.sh build.
ARG RUNTIME_IMAGE=debian@sha256:3783cc01769c7b2b1b83a5c5ad96c815348e28ed7da68e2e3687004faa906251
FROM ${RUNTIME_IMAGE}
COPY bin/opentdf /usr/local/bin/opentdf
ENTRYPOINT ["/usr/local/bin/opentdf"]
