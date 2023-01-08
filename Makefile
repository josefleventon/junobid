.PHONY: lint types

lint:
	cargo clippy --all-targets -- -D warnings

optimize:
	sh scripts/optimize.sh

schema:
	sh scripts/schema.sh