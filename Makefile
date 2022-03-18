PROJECT=tihc

.PHONY: all clean test

default: main buildsucc

main:
	@>&2  rm -rf bin && mkdir bin && cd bin
	@>&2  rm -rf ./*.docx
	@>&2  cargo build
	@>&2  mv ./target/debug/tihc ./bin/

prepare: 
	/bin/bash ./deploy_grafana_image_render.sh

pkg: 
	@>&2  mkdir tihc
	@>&2  cp -rf ./bin/* ./tihc
	@>&2  cp -rp ./README.md ./tihc/
	@>&2  tar -zcvf tihc-v0.1.0-beta.1-linux-amd64.tar.gz ./tihc >>/dev/null 2>/dev/null
	@>&2  rm -rf ./tihc
	@echo Package tool TiHC successfully!

buildsucc:
	@echo Build tool TiHC successfully!

all: dev


dev: 
	@>&2 cargo fmt
	@>&2 cargo test
	@>&2 echo "Great!, all tests passed."

clean:
