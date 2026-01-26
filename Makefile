.PHONY: changelog test fmt clippy check

changelog:
	git cliff -o CHANGELOG.md
	@echo "CHANGELOG.md updated!"

test:
	cargo test --workspace

fmt:
	cargo fmt --all

clippy:
	cargo clippy --workspace --all-targets -- -D warnings

check: fmt clippy
	@echo "✅ All checks passed!"