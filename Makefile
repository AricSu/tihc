.PHONY: default build test clean release

default: build

build:
	@rm -rf bin && mkdir bin
	@cargo build
	@cp target/debug/tihc bin/

test:
	@cargo test --all

clean:
	@cargo clean
	@rm -rf bin

release:
	@rm -rf bin && mkdir bin
	@cargo build --release
	@cp target/release/tihc bin/