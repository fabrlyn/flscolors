build:
	cargo build --release

pack: build
	upx --best --lzma target/release/flscolors-cli

install: pack
	mv target/release/flscolors-cli target/release/flscolors
	sudo mv target/release/flscolors /usr/local/bin/