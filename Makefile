# mkcv — task runner
#
# Run `make` or `make help` to see available targets.

BIN         := target/release/mkcv
DEBUG_BIN   := target/debug/mkcv
INPUT       ?= resume.yml
OUTPUT      ?= resume.pdf
EXAMPLES    := $(wildcard examples/*.yml)
EXAMPLE_PDFS := $(patsubst examples/%.yml,examples/out/%.pdf,$(EXAMPLES))

.DEFAULT_GOAL := help

## help: Show this help.
.PHONY: help
help:
	@echo "mkcv — available targets:"
	@echo
	@grep -E '^## ' $(MAKEFILE_LIST) | sed 's/## /  /' | sort
	@echo
	@echo "Variables: INPUT=$(INPUT)  OUTPUT=$(OUTPUT)  NAME=<example>"

## build: Compile the debug binary.
.PHONY: build
build:
	cargo build

## release: Compile the optimized release binary.
.PHONY: release
release: $(BIN)

$(BIN): $(shell find src assets -type f) Cargo.toml
	cargo build --release

## run: Run the debug binary. Pass args via ARGS, e.g. make run ARGS="build".
.PHONY: run
run:
	cargo run -- $(ARGS)

## init: Scaffold a boilerplate $(INPUT) in the current directory.
.PHONY: init
init: build
	$(DEBUG_BIN) init --output $(INPUT)

## dev-build: Compile $(INPUT) -> $(OUTPUT) with the debug binary.
.PHONY: dev-build
dev-build: build
	$(DEBUG_BIN) build --input $(INPUT) --output $(OUTPUT)

## compile: Compile $(INPUT) -> $(OUTPUT) with the release binary.
.PHONY: compile
compile: release
	$(BIN) build --input $(INPUT) --output $(OUTPUT)

## watch: Rebuild $(OUTPUT) whenever $(INPUT) changes.
.PHONY: watch
watch: build
	$(DEBUG_BIN) watch --input $(INPUT) --output $(OUTPUT)

## examples: Compile every examples/*.yml into examples/out/*.pdf.
.PHONY: examples
examples: release $(EXAMPLE_PDFS)
	@echo "✓ Built $(words $(EXAMPLE_PDFS)) example PDF(s) into examples/out/"

examples/out/%.pdf: examples/%.yml $(BIN)
	@mkdir -p examples/out
	$(BIN) build --input $< --output $@

## example: Compile one example by name, e.g. make example NAME=minimal.
.PHONY: example
example: release
	@test -n "$(NAME)" || { echo "usage: make example NAME=<file without .yml>"; exit 1; }
	@mkdir -p examples/out
	$(BIN) build --input examples/$(NAME).yml --output examples/out/$(NAME).pdf

## fmt: Format the source with rustfmt.
.PHONY: fmt
fmt:
	cargo fmt

## check: Type-check without producing a binary.
.PHONY: check
check:
	cargo check

## lint: Run clippy with warnings denied.
.PHONY: lint
lint:
	cargo clippy --all-targets -- -D warnings

## test: Run the test suite.
.PHONY: test
test:
	cargo test

## size: Print the release binary size.
.PHONY: size
size: release
	@ls -lh $(BIN) | awk '{print "release binary: " $$5}'

## dist: Build the release binary and copy it to dist/mkcv (Linux x86_64, for the skill to fetch).
.PHONY: dist
dist: release
	@mkdir -p dist
	@cp $(BIN) dist/mkcv
	@chmod +x dist/mkcv
	@ls -lh dist/mkcv | awk '{print "dist: dist/mkcv (" $$5 ")"}'

## install: Install the binary into ~/.cargo/bin.
.PHONY: install
install:
	cargo install --path .

## clean: Remove build artifacts and generated example PDFs.
.PHONY: clean
clean:
	cargo clean
	rm -rf examples/out
