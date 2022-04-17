DOCTESTS = tests/doctest-stdlib.os

tests/doctest-%.os: src/%.rs
	./scripts/doctest.py $< $@

.PHONY: check
check: $(DOCTESTS)
	cargo run tests/tests.os
