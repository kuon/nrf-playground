build:
	cargo build

run:
	cargo run

connect:
	openocd -f config/laird.cfg

.PHONY: clean

clean:
	rm -fr target


.PHONY: flash-softdevice

flash-softdevice:
	nrfjprog -f nrf52 --program s140_nrf52_7.2.0_softdevice.hex --sectorerase

