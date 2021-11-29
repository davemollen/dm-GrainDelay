DOCKER_BUILDKIT=1 docker build --rm --file docker/modduo/Dockerfile --output out/modduo . && \
DOCKER_BUILDKIT=1 docker build --rm --file docker/moddwarf/Dockerfile --output out/moddwarf .