build-dev:
	cargo build
	cp target/debug/libpywasmi_lib.so pywasmi_lib.so

test: build-dev
	py.test