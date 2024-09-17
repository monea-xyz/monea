# TODO this should package monea-cli, kurtosis package, and kurtosis cli into a single tarball
# this file is incomplete and hasn't been tested yet

.PHONY: all clean build package

# Adjust these variables as needed
BINARY_NAME := monea-cli
KURTOSIS_CLI_URL := https://example.com/kurtosis-cli-download
ENGINE_DIR := /engine

all: package

clean:
	cargo clean
	rm -rf dist

build:
	cargo build --release

download_dependencies:
	mkdir -p dist
	curl -L $(KURTOSIS_CLI_URL) -o dist/kurtosis
	chmod +x dist/kurtosis
	cp -R $(ENGINE_DIR) dist/engine

package: clean build download_dependencies
	mkdir -p dist
	cp target/release/$(BINARY_NAME) dist/
	tar -czf $(BINARY_NAME).tar.gz -C dist .