.PHONY: all lint security test

all: lint security test

lint:
	@echo "Running linter..."
	@cargo clippy --workspace --all-targets -- -D warnings

security:
	@echo "Running security scan..."
	@cargo audit

test:
	@echo "Running tests..."
	@cargo test --verbose
