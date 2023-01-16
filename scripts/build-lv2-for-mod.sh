PACKAGE_NAME=(`./scripts/get-package-name.sh lv2`)
BINARY_NAME="lib$PACKAGE_NAME.so"
DOCKER_BUILDKIT=1 docker build --file scripts/docker/modduo/Dockerfile --build-arg BINARY_NAME=$BINARY_NAME --output lv2/out/modduo . && \
DOCKER_BUILDKIT=1 docker build --file scripts/docker/moddwarf/Dockerfile --build-arg BINARY_NAME=$BINARY_NAME --output lv2/out/moddwarf .
docker system prune --force