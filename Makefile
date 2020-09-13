build:
	cargo build

run:
	cargo run

connect:
	openocd -f config/laird.cfg

.PHONY: clean

clean:
	rm -fr target


