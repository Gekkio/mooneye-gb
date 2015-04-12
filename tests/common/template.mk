WLA := wla-gb
WLALINK := wlalink

NAME := $(shell basename $(CURDIR))

all: test.gb

test.o: test.s ../common/common.i
	@$(WLA) -o $(WLAFLAGS) test.s test.o

test.gb: test.o
	@$(WLALINK) ../common/linkfile test.gb
	@echo --- $(NAME)

clean:
	@rm -f test.gb test.o
