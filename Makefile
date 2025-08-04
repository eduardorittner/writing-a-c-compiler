build:
	cargo build --release

c1: build
	cd tests && \
	./test_compiler ../target/release/cli --chapter 1 --stage lex && \
	./test_compiler ../target/release/cli --chapter 1 --stage parse && \
	./test_compiler ../target/release/cli --chapter 1 --stage tacky && \
	./test_compiler ../target/release/cli --chapter 1 --stage codegen && \
	./test_compiler ../target/release/cli --chapter 1 --stage run
