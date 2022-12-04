ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
$(eval $(ARGS):;@:)

.PHONY: run $(ARGS)

run:
	@echo RUNNING "$(ARGS)"
	@cat "$(ARGS)/input" | cargo run --manifest-path "./$(ARGS)/Cargo.toml" -q