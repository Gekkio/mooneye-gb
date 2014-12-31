TESTS := \
	boot_div boot_div2 div_timing ei_timing if_ie_registers oam_dma_timing\
	oam_dma_timing2 pop_timing rapid_di_ei reti_intr_timing

all:
	@$(foreach TEST, $(TESTS), $(MAKE) -C $(TEST) --no-print-directory; )

clean:
	@$(foreach TEST, $(TESTS), $(MAKE) -C $(TEST) --no-print-directory clean; )
