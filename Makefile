src=src/main.rs
out=target/release/git-commit-mine
bin=${HOME}/.local/bin/git-commit-mine

debug: $(src)
	cargo build

check: $(src)
	cargo test

$(out): $(src)
	cargo build --release

install: check $(out)
	install -c $(out) $(bin)
