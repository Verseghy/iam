all:

.PHONY: build-images
build-images:
	podman build -t ghcr.io/verseghy/iam-cli -f containerfiles/cli.Containerfile .
	podman build -t ghcr.io/verseghy/iam-migration -f containerfiles/migration.Containerfile .
	podman build -t ghcr.io/verseghy/iam .

.PHONY: push-images
push-images:
	podman push ghcr.io/verseghy/iam-cli
	podman push ghcr.io/verseghy/iam-migration
	podman push ghcr.io/verseghy/iam

