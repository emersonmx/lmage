.PHONY: run-linux
run-linux:
	cargo run --target x86_64-unknown-linux-gnu

.PHONY: run-windows
run-windows:
	cargo run --target x86_64-pc-windows-gnu

.PHONY: run-web
run-web: build-web-dev copy-web-assets
	python -m http.server -d pkg/ -b 0.0.0.0

.PHONY: build-web-dev
build-web-dev:
	wasm-pack build --target web --dev

.PHONY: build-web
build-web:
	wasm-pack build --target web

.PHONY: copy-web-assets
copy-web-assets:
	cp -f web/index.html pkg/index.html

.PHONY: clean
clean:
	cargo clean
	rm -rf pkg/
