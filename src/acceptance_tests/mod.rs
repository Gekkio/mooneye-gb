use self::fixture::run_acceptance_test;

mod fixture;

#[test]
fn add_sp_e_timing() { run_acceptance_test("add_sp_e_timing") }

#[test]
fn call_cc_timing() { run_acceptance_test("call_cc_timing") }

#[test]
fn call_cc_timing2() { run_acceptance_test("call_cc_timing2") }

#[test]
fn call_timing() { run_acceptance_test("call_timing") }

#[test]
fn call_timing2() { run_acceptance_test("call_timing2") }

#[test]
fn div_timing() { run_acceptance_test("div_timing") }

#[test]
fn ei_timing() { run_acceptance_test("ei_timing") }

#[test]
fn halt_ime1() { run_acceptance_test("halt_ime1") }

#[test]
fn if_ie_registers() { run_acceptance_test("if_ie_registers") }

#[test]
fn intr_timing() { run_acceptance_test("intr_timing") }

#[test]
fn intr_timing2() { run_acceptance_test("intr_timing2") }

#[test]
fn jp_cc_timing() { run_acceptance_test("jp_cc_timing") }

#[test]
fn jp_timing() { run_acceptance_test("jp_timing") }

#[test]
fn ld_hl_sp_e_timing() { run_acceptance_test("ld_hl_sp_e_timing") }

#[test]
fn oam_bits() { run_acceptance_test("oam_bits") }

#[test]
fn oam_dma_restart() { run_acceptance_test("oam_dma_restart") }

#[test]
fn oam_dma_timing() { run_acceptance_test("oam_dma_timing") }

#[test]
fn pop_timing() { run_acceptance_test("pop_timing") }

#[test]
fn push_timing() { run_acceptance_test("push_timing") }

#[test]
fn rapid_di_ei() { run_acceptance_test("rapid_di_ei") }

#[test]
fn reti_intr_timing() { run_acceptance_test("reti_intr_timing") }

#[test]
fn rst_timing() { run_acceptance_test("rst_timing") }
