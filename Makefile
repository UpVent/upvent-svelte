# Variables
DOCKER_IMAGE_NAME=upvent_svelte
DOCKER_CONTAINER_NAME=upvent_svelte

# Generate a production build.
build:
	npm install
	npm run build

# Run this to start developing.
debug:
	npm install
	npm run dev

dev:
	$(MAKE) debug

# Run this before pushing to git.
test_before_push:
	npm install
	npm run test
	npm run check
	npm run test:unit
	npm run lint

# Run this to check that everything is ok while developing.
test:
	npm install
	npm run test

# Commands for running in containers

# Build docker image
docker-build:
	npm install
	npm run build
	docker build -t $(DOCKER_IMAGE_NAME) .

# Run docker container
docker-run:
	docker run -d -p 80:80 --name $(DOCKER_CONTAINER_NAME) $(DOCKER_IMAGE_NAME)

# Stop docker container
docker-stop:
	docker stop $(DOCKER_CONTAINER_NAME)

# Remove docker container
docker-rm:
	docker rm $(DOCKER_CONTAINER_NAME)

# PHONY targets
.PHONY: build debug test_before_push docker-build docker-run docker-stop docker-rm
