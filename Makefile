.PHONY: run-linux
run-linux:
	cargo run --target x86_64-unknown-linux-gnu -p desktop_app

.PHONY: run-windows
run-windows:
	cargo run --target x86_64-pc-windows-gnu -p desktop_app

.PHONY: run-web
run-web: build-web-dev copy-web-assets
	python -m http.server -d web_app/dist/ -b 0.0.0.0

.PHONY: build-web-dev
build-web-dev:
	wasm-pack build --target web --out-dir ../web_app/dist --dev lmage

.PHONY: build-web
build-web:
	wasm-pack build --target web --out-dir ../web_app/dist lmage

.PHONY: copy-web-assets
copy-web-assets:
	cp -f web_app/templates/index.html web_app/dist/index.html

.PHONY: clean
clean:
	cargo clean
	rm -rf web_app/dist/
