WLA := wla-gb
WLALINK := wlalink

NAME := $(shell basename $(CURDIR))

all: test.gb

test.o: test.s ../common/common.i
	@echo --- $(NAME): Assemble
	$(WLA) -o test.s test.o

test.gb: test.o
	@echo --- $(NAME): Link
	$(WLALINK) ../common/linkfile test.gb

clean:
	@echo --- $(NAME): Clean
	rm -f test.gb test.o
