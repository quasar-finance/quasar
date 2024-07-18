###############################################################################
###                                Docker                                  ###
###############################################################################

RUNNER_BASE_IMAGE_DISTROLESS := gcr.io/distroless/static-debian11
RUNNER_BASE_IMAGE_ALPINE := alpine:3.17
RUNNER_BASE_IMAGE_NONROOT := gcr.io/distroless/static-debian11:nonroot

docker-help:
	@echo "docker subcommands"
	@echo ""
	@echo "Usage:"
	@echo "  make docker-[command]"
	@echo ""
	@echo "Available Commands:"
	@echo "  build                Build Docker image"
	@echo "  build-alpine         Build alpine Docker image"
	@echo "  build-distroless     Build distroless Docker image"
	@echo "  build-nonroot        Build nonroot Docker image"
	@echo "  compose-up           Launching local env, building images if not available"
	@echo "  compose-up-recreate  Recreate local env (will destroy application state)"
	@echo "  compose-build        Rebuilding image for local env"
	@echo "  compose-rebuild      Rebuilding images and restarting containers"
	@echo "  compose-stop         Stop docker containers without removing them"
	@echo "  compose-down         Stopping docker containers and REMOVING THEM"
	@echo "  attach-quasar        Connecting to quasar docker container"
	@echo "  attach-osmosis       Connecting to osmosis docker container"
	@echo "  attach-relayer       Connecting to relayer docker container"
	@echo "  test-e2e             Running e2e tests"
	@echo "  e2e-build            Build e2e docker images of the chain needed for interchaintest"


docker: docker-help

docker-build:
	@DOCKER_BUILDKIT=1 docker build \
		-t quasar:local \
		-t quasar:local-distroless \
		--build-arg GO_VERSION=$(GO_VERSION) \
		--build-arg RUNNER_IMAGE=$(RUNNER_BASE_IMAGE_DISTROLESS) \
		--build-arg GIT_VERSION=$(VERSION) \
		--build-arg GIT_COMMIT=$(COMMIT) \
		-f Dockerfile .

docker-build-distroless: docker-build

docker-build-alpine:
	@DOCKER_BUILDKIT=1 docker build \
		-t quasar:local-alpine \
		--build-arg GO_VERSION=$(GO_VERSION) \
		--build-arg RUNNER_IMAGE=$(RUNNER_BASE_IMAGE_ALPINE) \
		--build-arg GIT_VERSION=$(VERSION) \
		--build-arg GIT_COMMIT=$(COMMIT) \
		-f Dockerfile .

docker-build-nonroot:
	@DOCKER_BUILDKIT=1 docker build \
		-t quasar:local-nonroot \
		--build-arg GO_VERSION=$(GO_VERSION) \
		--build-arg RUNNER_IMAGE=$(RUNNER_BASE_IMAGE_NONROOT) \
		--build-arg GIT_VERSION=$(VERSION) \
		--build-arg GIT_COMMIT=$(COMMIT) \
		-f Dockerfile .


# This is not available to avoid missbehavior since it seems to be a bug in docker compose -p localenv:
# https://github.com/docker/compose/issues/10068
# docker-compose-up-attached: ##@docker Run (and build if needed) env in docker compose. Attach if running in background.
# 	@echo "Launching local env with docker-compose"
# 	DOCKER_BUILDKIT=1 COMPOSE_DOCKER_CLI_BUILD=1 docker compose -p localenv -f tests/docker/docker-compose.yml up

docker-compose-up: ##@docker Run local env, build only if no images available
	@echo "Launching local env, building images if not available"
	DOCKER_BUILDKIT=1 COMPOSE_DOCKER_CLI_BUILD=1 docker compose -p localenv -f tests/docker/docker-compose.yml up -d

docker-compose-up-recreate: ##@docker DESTROY env containers and respawn them
	@echo "Recreate local env (will destroy application state)"
	DOCKER_BUILDKIT=1 COMPOSE_DOCKER_CLI_BUILD=1 docker compose -p localenv -f tests/docker/docker-compose.yml up -d --force-recreate

docker-compose-build: ##@docker Build new image if there are code changes, won't recreate containers.
	@echo "Rebuilding image for local env"
	DOCKER_BUILDKIT=1 COMPOSE_DOCKER_CLI_BUILD=1 docker compose -p localenv -f tests/docker/docker-compose.yml build

docker-compose-rebuild: docker-compose-build docker-compose-up-recreate ##@docker Recreate containers building new code if needed
	@echo "Rebuilding images and restarting containers"

docker-compose-stop: ##@docker Stop containers without deleting them
	@echo "Stop docker containers without removing them"
	DOCKER_BUILDKIT=1 COMPOSE_DOCKER_CLI_BUILD=1 docker compose -p localenv -f tests/docker/docker-compose.yml stop

docker-compose-down: ##@docker Stop AND DELETE delete the containers
	@echo "Stopping docker containers and REMOVING THEM"
	DOCKER_BUILDKIT=1 COMPOSE_DOCKER_CLI_BUILD=1 docker compose -p localenv -f tests/docker/docker-compose.yml down

docker-attach-quasar: ##@docker Connect to a terminal prompt in QUASAR node container
	@echo "Connecting to quasar docker container"
	docker exec -it localenv-quasar-1 /bin/bash

docker-attach-osmosis: ##@docker Connect to a terminal prompt in OSMOSIS node container
	@echo "Connecting to osmosis docker container"
	docker exec -it localenv-osmosis-1 /bin/ash

docker-attach-relayer: ##@docker Connect to a terminal prompt in RLY node container
	@echo "Connecting to relayer docker container"
	docker exec -it localenv-relayer-1 /bin/bash

docker-test-e2e: docker-compose-up
	@echo "Running e2e tests"
	cd ./tests/shell/ && ./create_and_execute_contract.sh

###############################################################################
###                      Docker E2E InterchainTest                          ###
###############################################################################

docker-e2e-build:
	$(eval CHAINS=$(filter-out $@,$(MAKECMDGOALS)))
	@for chain in $(CHAINS); do \
		echo "Building $$chain"; \
		DOCKER_BUILDKIT=1 docker build \
			-t $$chain:local \
			-t $$chain:local-distroless \
			--build-arg GO_VERSION=$(GO_VERSION) \
			--build-arg RUNNER_IMAGE=$(RUNNER_BASE_IMAGE_DISTROLESS) \
			--build-arg GIT_VERSION=$(VERSION) \
			--build-arg GIT_COMMIT=$(COMMIT) \
			-f ./tests/e2e/dockerfiles/$$chain.Dockerfile . ;\
	done
