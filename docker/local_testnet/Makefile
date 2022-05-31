all: quasarnoded-env

quasarnoded-env:
	sudo docker build --build-arg UID=$(shell id -u) --build-arg GID=$(shell id -g) --tag quasar-labs/quasarnoded-env quasarnoded-env

.PHONY: all quasarnoded-env
