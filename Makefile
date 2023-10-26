all: rebuild

rebuild:
	cargo clean
	cargo build

# Check that it build for stm32
check-stm32:
	cargo build --target thumbv7em-none-eabihf