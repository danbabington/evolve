.PHONY: format build commit push

GREEN=\033[0;32m
BLUE=\033[0;34m
YELLOW=\033[1;33m
NC=\033[0m # No Color

format:
	cargo fmt

build:
	cargo build

run:
	@cargo run

commit:
	@git add .
	@cz --config cz.toml commit

push:
	@(git pull || true) && git push
