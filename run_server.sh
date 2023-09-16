#!/usr/bin/env sh

podman pull ambakshi/perforce-server
podman rm -f perforce || true
podman run -d -p 8080:8080 -p 1666:1666 -h perforce --name perforce ambakshi/perforce-server
podman logs -f perforce
