COMPLETED_DAYS = 01 02
INPUT = input.txt

DAYS = $(patsubst %,day%,$(COMPLETED_DAYS))
SRC_DIR = src

$(DAYS):
	cargo run --bin $@ $(SRC_DIR)/$@/$(INPUT)
