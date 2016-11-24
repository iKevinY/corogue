all: build

build: corogue.gba

run: corogue.gba
	@open target/corogue.gba

corogue.gba:
	xargo build --target=gba --release
	arm-none-eabi-objcopy -O binary target/gba/release/corogue target/corogue.gba
	gbafix target/corogue.gba > /dev/null 2>&1

.PHONY: all build clean run
