PROJECT=ticheck

.PHONY: all clean test

default: main buildsucc

main:
	@>&2  rm -rf bin && mkdir bin && cd bin
	@>&2  rm -rf ./*.docx
	@>&2  cargo build
	@>&2  mv ./target/debug/tihc ./bin/

prepare: 
	/bin/bash ./deploy_grafana_image_render.sh


buildsucc:
	@echo Build tool TiCheck successfully!

all: dev


dev: 
	@>&2 cargo fmt
	@>&2 cargo test
	@>&2 echo "Great!, all tests passed."

clean:
