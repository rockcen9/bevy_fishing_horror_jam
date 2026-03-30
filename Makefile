.PHONY: serve release wasm upload build archive profile build-release patch balance audio
include make/audio.mk
include make/itch.mk
include make/web.mk



# minor or release major, minor, patch,
patch:
	cargo release patch --no-publish --execute


balance:
	cargo run -p balance_downloader

profile:
	cargo bloat -n 100000 --message-format json > out.json


fix:
	cargo fix --workspace --message-format=json --allow-dirty
