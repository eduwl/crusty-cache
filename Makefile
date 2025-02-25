# ================================================================================ #
#                                 Cargo Clear	                                   #
# ================================================================================ #

clean:
	@echo "Cleaning up..."
	cargo clean

# ================================================================================ #
#                                 Cargo Check	                                   #
# ================================================================================ #

check-workspace:
	@echo "Check Workspace..."
	@cargo check --workspace

CRATES=$(shell sed -n '/^\[workspace\]/,/^\[/{/members = /p}' Cargo.toml | sed 's/members = \[\(.*\)\]/\1/' | tr -d '",')
doc:
	@for crate in $(CRATES); do \
		echo "Gerando documentação para $$crate"; \
		cargo doc -p $$crate --no-deps; \
	done

# ================================================================================ #
#                                 Cargo Build DEV                                  #
# ================================================================================ #

build:
	@echo "Building DEV..."
	@cargo build


# ================================================================================ #
#                                 Cargo Release                                    #
# ================================================================================ #

release:
	@echo "Building release..."
	@cargo build --release

# ================================================================================ #
#                                  Cargo Run                                       #
# ================================================================================ #
run:
	@echo "Running server..."
	cargo run
