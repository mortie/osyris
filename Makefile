OSYRISLIBS = src/stdlib.rs
DOCTESTS = $(patsubst src/%.rs,tests/doctest-%.os,$(OSYRISLIBS))
DOCS = $(patsubst src/%.rs,docs/%.md,$(OSYRISLIBS))

all: $(DOCTESTS) $(DOCS) check

tests/doctest-%.os: src/%.rs
	./scripts/doctest.py $< $@

docs/%.md: src/%.rs
	./scripts/docgen.py $< $@ $(patsubst src/%.rs,%,$<)

.PHONY: check
check: $(DOCTESTS)
	cargo run tests/tests.os

.PHONY: doc
doc: $(DOCS)
