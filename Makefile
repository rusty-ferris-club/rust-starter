build-cli: all-validation ## build cli project.
	cargo build

build-binary: all-validation ## build binary project.
	cargo build --no-default-features

test: ## run cargo test.
	cargo test

fmt: ## validate formating.
	cargo fmt --all -- --check

clippy: # check common mistakes and improve your Rust code.
	cargo clippy -- -D warnings

validate-doc: clean-doc # validate documentation syntax.
	RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --document-private-items

show-doc: clean-doc ## open a local docs website.
	cargo doc --open --no-deps

clean-doc: # clean doc folder.
	cargo clean 

all-validation: test fmt clippy validate-doc ## runs all ci validation.
 
help: ## Prints help information.
	@fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed -e 's/##//'
