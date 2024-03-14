OSYRISLIBS = src/stdlib.rs
DOCTESTS = $(patsubst src/%.rs,tests/doctest-%.os,$(OSYRISLIBS))
DOCS = $(patsubst src/%.rs,docs/%.md,$(OSYRISLIBS))

.PHONY: all
all: osyris

.PHONY: osyris
osyris:
	cargo build --release
	cp target/release/osyris .

tests/doctest-%.os: src/%.rs
	./scripts/doctest.py $< $@

docs/%.md: src/%.rs
	./scripts/docgen.py $< $@ $(patsubst src/%.rs,%,$<)

.PHONY: check
check: $(DOCTESTS)
	cargo run tests/tests.os

.PHONY: doc
doc: $(DOCS)

.PHONY: clean
clean:
	rm -rf target osyris
