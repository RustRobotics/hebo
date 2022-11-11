CURRENT_TAG = $(shell git describe --abbrev=0 --tags | sed -e 's/v//')

all: build-docker push-docker

build-docker:
	docker build -t xushaohua/hebo:latest -f Dockerfile ..
	docker tag xushaohua/hebo:latest xushaohua/hebo:$(CURRENT_TAG)

push-docker:
	docker push xushaohua/hebo:latest
	docker push xushaohua/hebo:$(CURRENT_TAG)
