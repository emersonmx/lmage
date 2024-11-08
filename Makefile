.PHONY: run-linux
run-linux:
	cargo run --target x86_64-unknown-linux-gnu

.PHONY: run-windows
run-windows:
	cargo run --target x86_64-pc-windows-gnu

.PHONY: run-web
run-web:
	@echo "todo"
