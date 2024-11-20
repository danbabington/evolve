.PHONY: format build commit push

GREEN=\033[0;32m
BLUE=\033[0;34m
YELLOW=\033[1;33m
NC=\033[0m # No Color

format:
	cargo fmt

build:
	cargo build

commit:
	@git add .
	@cz commit

push:
	@(git pull || true) && git push
