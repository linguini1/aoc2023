COMPLETED_DAYS = 01
PUZZLE_INPUT = input.txt

DAYS = $(patsubst %,day%,$(COMPLETED_DAYS))
SRC_DIR = src

$(DAYS):
	cargo run --bin $@ $(SRC_DIR)/$@/$(PUZZLE_INPUT)
