CARGO = cargo

CARGO_OPTS = 

all:
	$(MAKE) build
	$(MAKE) doc

build: $(CARGO) $(CARGO_OPTS) build

test:
	$(CARGO) $(CARGO_OPTS) test

bench:
	$(CARGO) $(CARGO_OPTS) bench

doc:
	$(CARGO) $(CARGO_OPTS) doc

set_nightly:
	rustup update
	rustup install nightly
	rustup default nightly

.PHONY: all build test bench doc set_nightly
