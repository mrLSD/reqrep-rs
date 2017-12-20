#
# Makefile
# @author Evgeny Ukhanov <mrlsd@ya.ru>
#

check:
	@cargo check

run:
	@echo Running...
	@cargo run
	@echo Done.

build:
	@echo Build debug version...
	@cargo build
	@echo Done.

release:
	@echo Build release...
	@cargo build --release
	@echo Done.

test:
	@echo Run tests...
	@cargo test
	@echo Done.

