TESTS := \
	boot_div boot_div2 div_timing ei_timing if_ie_registers intr_timing\
	intr_timing2 ld_hl_sp_e_timing oam_dma_restart oam_dma_timing pop_timing\
	push_timing rapid_di_ei reti_intr_timing

all:
	@$(foreach TEST, $(TESTS), $(MAKE) -C $(TEST) --no-print-directory; )

clean:
	@$(foreach TEST, $(TESTS), $(MAKE) -C $(TEST) --no-print-directory clean; )
