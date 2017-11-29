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

deps:
	@git clone --depth 1 https://github.com/nanomsg/nanomsg.git /tmp/nanomsg
	@cd /tmp/nanomsg && mkdir build && cd build && cmake .. && cmake --build .
	@cd /tmp/nanomsg/build && sudo cmake --build . --target install && sudo ldconfig --verbose
