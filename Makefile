default: build

build:
	@>&2  rm -rf bin && mkdir bin && cd bin
	@>&2  cargo build
	@>&2  mv ./target/debug/tihc ./bin/