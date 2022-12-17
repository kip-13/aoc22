ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
$(eval $(ARGS):;@:)

.PHONY: run $(ARGS)

run:
	@echo RUNNING "$(ARGS)"
ifeq ($(OS),Windows_NT)
	@type ".\$(ARGS)\input" | cargo run --manifest-path ".\$(ARGS)\Cargo.toml" -q
else
	@cat "$(ARGS)/input" | cargo run --manifest-path "./$(ARGS)/Cargo.toml" -q
endif