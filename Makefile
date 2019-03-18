APP=tremor-runtime
DOCKER_VSN=$(shell grep 'ARG tag' docker/tremor-runtime.dockerfile | sed 's/.*=//')
CARGO_VSN=$(shell grep '^version' Cargo.toml | sed -e 's/.*=[^"]*"//' -e 's/"$$//')
VSN=$(DOCKER_VSN)
YEAR=2018-2019

help:
	@echo "This makefile wraps the tasks:"
	@echo "  image - builds the image"
	@echo "  demo - runs a simple demo"
	@echo "  clippy-install - install nightly and clippy"
	@echo "  clippy - runs clippy"
	@echo "  bench - runs benchmarks"
	@echo "  it - runs integration tests"
	@echo "  doc - creates and opens documentation"

image:
	docker-compose build

demo: image
	-docker-compose -f demo/demo.yaml rm -fsv
	-docker-compose -f demo/demo.yaml up
	-docker-compose -f demo/demo.yaml rm -fsv

clippy-install:
	rustup update
	rustup install nightly
	rustup component add clippy

clippy:
	CARGO_TARGET_DIR=target.clippy cargo +nightly clippy

it:
	integration_testing/runner

doc: force
	cargo doc --open --no-deps
	rm -r doc
	cp -r target/doc .

bench: force
	cargo build --release --features bench
	for f in bench/*.sh; do $$f; done

force:
	true

chk_copyright:
	for f in `find . -name '*.rs' | grep -v '/target'`; do cat $$f | grep 'Copyright 2018-2019, Wayfair GmbH' > /dev/null || (echo "No copyright in $$f") done

chk_copyright_ci:
	for f in `find . -name '*.rs' | grep -v '/target'`; do cat $$f | grep 'Copyright 2018-2019, Wayfair GmbH' > /dev/null || exit 1; done

docserve:
	mkdocs serve
