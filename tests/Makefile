TESTS := \
	add_sp_e_timing boot_div boot_div2 call_timing call_timing2 call_cc_timing\
	call_cc_timing2 div_timing ei_timing halt_ime1 if_ie_registers intr_timing\
	intr_timing2 jp_timing jp_cc_timing ld_hl_sp_e_timing oam_bits\
	oam_dma_restart oam_dma_timing pop_timing push_timing rapid_di_ei\
	reti_intr_timing rst_timing

all:
	@$(foreach TEST, $(TESTS), $(MAKE) -C $(TEST) --no-print-directory; )

clean:
	@$(foreach TEST, $(TESTS), $(MAKE) -C $(TEST) --no-print-directory clean; )
