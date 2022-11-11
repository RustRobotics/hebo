CURRENT_TAG = $(shell git describe --abbrev=0 --tags)

all: build-hebo-docker push-hebo-docker

build-hebo-docker:
	docker build -t xushaohua/hebo:latest -f Dockerfile ..
	docker tag xushaohua/hebo:latest xushaohua/hebo:$(CURRENT_TAG)

push-hebo-docker:
	docker push xushaohua/hebo:latest
	docker push xushaohua/hebo:$(CURRENT_TAG)
