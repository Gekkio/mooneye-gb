TESTS := boot_div boot_div2 div_timing pop_timing

all:
	@$(foreach TEST, $(TESTS), $(MAKE) -C $(TEST) --no-print-directory; )

clean:
	@$(foreach TEST, $(TESTS), $(MAKE) -C $(TEST) --no-print-directory clean; )
