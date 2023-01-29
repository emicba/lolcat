all: lol

lol:
	cargo build --release

install: lol
	cargo install --path .
