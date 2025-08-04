.PHONY: build

build: ast cli codegen lex parse x86
	cargo build --release

BIN = ../target/release/cli

c1: build
	cd tests && \
	./test_compiler $(BIN) --chapter 1 --stage lex && \
	./test_compiler $(BIN) --chapter 1 --stage parse && \
	./test_compiler $(BIN) --chapter 1 --stage tacky && \
	./test_compiler $(BIN) --chapter 1 --stage codegen && \
	./test_compiler $(BIN) --chapter 1 --stage run
