// This file is part of Mooneye GB.
// Copyright (C) 2014-2017 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// Mooneye GB is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Mooneye GB is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Mooneye GB.  If not, see <http://www.gnu.org/licenses/>.
use self::fixture::{run_test, run_test_with_model, run_test_with_models};
use config::Model::*;

mod fixture;

#[test]
fn add_sp_e_timing() { run_test("acceptance/add_sp_e_timing") }

#[test]
#[ignore]
fn boot_hwio_dmg_0() { run_test_with_model("acceptance/boot_hwio-dmg0", Dmg0) }

#[test]
#[ignore]
fn boot_hwio_dmg_abcx_mgb() { run_test_with_models("acceptance/boot_hwio-dmgABCXmgb", &[Dmg, Mgb]) }

#[test]
fn boot_hwio_s() { run_test_with_models("acceptance/boot_hwio-S", &[Sgb, Sgb2]) }

#[test]
fn boot_regs_dmg0() { run_test_with_model("acceptance/boot_regs-dmg0", Dmg0) }

#[test]
fn boot_regs_dmg_abcx() { run_test_with_model("acceptance/boot_regs-dmgABCX", Dmg) }

#[test]
fn boot_regs_mgb() { run_test_with_model("acceptance/boot_regs-mgb", Mgb) }

#[test]
fn boot_regs_sgb() { run_test_with_model("acceptance/boot_regs-sgb", Sgb) }

#[test]
fn boot_regs_sgb2() { run_test_with_model("acceptance/boot_regs-sgb2", Sgb2) }

#[test]
fn call_cc_timing() { run_test("acceptance/call_cc_timing") }

#[test]
fn call_cc_timing2() { run_test("acceptance/call_cc_timing2") }

#[test]
fn call_timing() { run_test("acceptance/call_timing") }

#[test]
fn call_timing2() { run_test("acceptance/call_timing2") }

#[test]
fn di_timing_gs() { run_test("acceptance/di_timing-GS") }

#[test]
fn div_timing() { run_test("acceptance/div_timing") }

#[test]
fn ei_timing() { run_test("acceptance/ei_timing") }

#[test]
fn halt_ime0_ei() { run_test("acceptance/halt_ime0_ei") }

#[test]
fn halt_ime0_nointr_timing() { run_test("acceptance/halt_ime0_nointr_timing") }

#[test]
fn halt_ime1_timing() { run_test("acceptance/halt_ime1_timing") }

#[test]
fn halt_ime1_timing2_gs() { run_test("acceptance/halt_ime1_timing2-GS") }

#[test]
fn if_ie_registers() { run_test("acceptance/if_ie_registers") }

#[test]
fn intr_timing() { run_test("acceptance/intr_timing") }

#[test]
fn jp_cc_timing() { run_test("acceptance/jp_cc_timing") }

#[test]
fn jp_timing() { run_test("acceptance/jp_timing") }

#[test]
fn ld_hl_sp_e_timing() { run_test("acceptance/ld_hl_sp_e_timing") }

#[test]
fn oam_dma_restart() { run_test("acceptance/oam_dma_restart") }

#[test]
fn oam_dma_start() { run_test("acceptance/oam_dma_start") }

#[test]
fn oam_dma_timing() { run_test("acceptance/oam_dma_timing") }

#[test]
fn pop_timing() { run_test("acceptance/pop_timing") }

#[test]
fn push_timing() { run_test("acceptance/push_timing") }

#[test]
fn rapid_di_ei() { run_test("acceptance/rapid_di_ei") }

#[test]
fn ret_timing() { run_test("acceptance/ret_timing") }

#[test]
fn reti_timing() { run_test("acceptance/reti_timing") }

#[test]
fn ret_cc_timing() { run_test("acceptance/ret_cc_timing") }

#[test]
fn reti_intr_timing() { run_test("acceptance/reti_intr_timing") }

#[test]
fn rst_timing() { run_test("acceptance/rst_timing") }

#[test]
fn bits_mem_oam() { run_test("acceptance/bits/mem_oam") }

#[test]
fn bits_reg_f() { run_test("acceptance/bits/reg_f") }

#[test]
fn bits_unused_hwio_gs() { run_test("acceptance/bits/unused_hwio-GS") }

#[test]
fn gpu_hblank_ly_scx_timing_gs() { run_test("acceptance/gpu/hblank_ly_scx_timing-GS") }

#[test]
fn gpu_intr_1_2_timing_gs() { run_test("acceptance/gpu/intr_1_2_timing-GS") }

#[test]
fn gpu_intr_2_0_timing() { run_test("acceptance/gpu/intr_2_0_timing") }

#[test]
fn gpu_intr_2_mode0_timing() { run_test("acceptance/gpu/intr_2_mode0_timing") }

#[test]
#[ignore]
fn gpu_intr_2_mode0_timing_sprites() { run_test("acceptance/gpu/intr_2_mode0_timing_sprites") }

#[test]
fn gpu_intr_2_mode3_timing() { run_test("acceptance/gpu/intr_2_mode3_timing") }

#[test]
fn gpu_intr_2_oam_ok_timing() { run_test("acceptance/gpu/intr_2_oam_ok_timing") }

#[test]
#[ignore]
fn gpu_lcdon_timing_dmg_abcx_mgb_s() { run_test("acceptance/gpu/lcdon_timing-dmgABCXmgbS") }

#[test]
#[ignore]
fn gpu_lcdon_write_timing_gs() { run_test("acceptance/gpu/lcdon_write_timing-GS") }

#[test]
#[ignore]
fn gpu_stat_irq_blocking() { run_test("acceptance/gpu/stat_irq_blocking") }

#[test]
fn gpu_vblank_stat_intr_gs() { run_test("acceptance/gpu/vblank_stat_intr-GS") }

#[test]
fn interrupts_ie_push() { run_test("acceptance/interrupts/ie_push") }

#[test]
#[ignore]
fn serial_boot_sclk_align_dmg_abcx_mgb() { run_test("acceptance/serial/boot_sclk_align-dmgABCXmgb") }

#[test]
#[ignore]
fn timer_div_write() { run_test("acceptance/timer/div_write") }

#[test]
#[ignore]
fn timer_rapid_toggle() { run_test("acceptance/timer/rapid_toggle") }

#[test]
#[ignore]
fn timer_tim00() { run_test("acceptance/timer/tim00") }

#[test]
#[ignore]
fn timer_tim00_div_trigger() { run_test("acceptance/timer/tim00_div_trigger") }

#[test]
#[ignore]
fn timer_tim01() { run_test("acceptance/timer/tim01") }

#[test]
#[ignore]
fn timer_tim01_div_trigger() { run_test("acceptance/timer/tim01_div_trigger") }

#[test]
#[ignore]
fn timer_tim10() { run_test("acceptance/timer/tim10") }

#[test]
#[ignore]
fn timer_tim10_div_trigger() { run_test("acceptance/timer/tim10_div_trigger") }

#[test]
#[ignore]
fn timer_tim11() { run_test("acceptance/timer/tim11") }

#[test]
#[ignore]
fn timer_tim11_div_trigger() { run_test("acceptance/timer/tim11_div_trigger") }

#[test]
#[ignore]
fn timer_tima_reload() { run_test("acceptance/timer/tima_reload") }

#[test]
#[ignore]
fn timer_tima_write_reloading() { run_test("acceptance/timer/tima_write_reloading") }

#[test]
#[ignore]
fn timer_tma_write_reloading() { run_test("acceptance/timer/tma_write_reloading") }

#[test]
fn mbc1_rom_512kb() { run_test("emulator-only/mbc1/rom_512Kb") }

#[test]
fn mbc1_rom_1mb() { run_test("emulator-only/mbc1/rom_1Mb") }

#[test]
fn mbc1_rom_2mb() { run_test("emulator-only/mbc1/rom_2Mb") }

#[test]
fn mbc1_rom_4mb() { run_test("emulator-only/mbc1/rom_4Mb") }

#[test]
fn mbc1_rom_8mb() { run_test("emulator-only/mbc1/rom_8Mb") }

#[test]
fn mbc1_rom_16mb() { run_test("emulator-only/mbc1/rom_16Mb") }

#[test]
fn mbc1_ram_64kb() { run_test("emulator-only/mbc1/ram_64Kb") }

#[test]
fn mbc1_ram_25kb() { run_test("emulator-only/mbc1/ram_256Kb") }

#[test]
fn mbc1_multicart_rom_8mb() { run_test("emulator-only/mbc1/multicart_rom_8Mb") }
