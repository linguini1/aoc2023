COMPLETED_DAYS = 01 02 03 04 05 06 07 08 09 10 11
INPUT = input.txt

DAYS = $(patsubst %,day%,$(COMPLETED_DAYS))
SRC_DIR = src

$(DAYS):
	cargo run --bin $@ $(SRC_DIR)/$@/$(INPUT)

all: $(DAYS)
