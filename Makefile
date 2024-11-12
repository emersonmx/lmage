.PHONY: run-linux
run-linux:
	cargo run --target x86_64-unknown-linux-gnu

.PHONY: run-windows
run-windows:
	cargo run --target x86_64-pc-windows-gnu

.PHONY: run-web
run-web: build-web
	python -m http.server -d pkg/ -b 0.0.0.0

.PHONY: build-web
build-web:
	wasm-pack build --target web
	cp -f web/index.html pkg/index.html
