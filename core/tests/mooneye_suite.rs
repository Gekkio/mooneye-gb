// This file is part of Mooneye GB.
// Copyright (C) 2014-2018 Joonas Javanainen <joonas.javanainen@gmail.com>
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
extern crate mooneye_gb;

use std::path::PathBuf;
use std::time::{Duration, Instant};

use mooneye_gb::config::{Bootrom, Cartridge, HardwareConfig, Model};
use mooneye_gb::config::Model::*;
use mooneye_gb::config::DEFAULT_MODEL_PRIORITY;
use mooneye_gb::emulation::{EmuTime, EmuEvents};
use mooneye_gb::machine::Machine;

#[test]
fn add_sp_e_timing() { run_test("acceptance/add_sp_e_timing") }

#[test]
#[ignore]
fn boot_hwio_dmg_0() { run_test_with_model("acceptance/boot_hwio-dmg0", Dmg0) }

#[test]
#[ignore]
fn boot_hwio_dmg_abc_mgb() { run_test_with_models("acceptance/boot_hwio-dmgABCmgb", &[Dmg, Mgb]) }

#[test]
fn boot_hwio_s() { run_test_with_models("acceptance/boot_hwio-S", &[Sgb, Sgb2]) }

#[test]
fn boot_regs_dmg0() { run_test_with_model("acceptance/boot_regs-dmg0", Dmg0) }

#[test]
fn boot_regs_dmg_abc() { run_test_with_model("acceptance/boot_regs-dmgABC", Dmg) }

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
fn ei_sequence() { run_test("acceptance/ei_sequence") }

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
fn interrupts_ie_push() { run_test("acceptance/interrupts/ie_push") }

#[test]
fn oam_dma_basic() { run_test("acceptance/oam_dma/basic") }

#[test]
fn oam_dma_reg_read() { run_test("acceptance/oam_dma/reg_read") }

#[test]
fn oam_dma_sources_dmg_abc_mgb_s() { run_test("acceptance/oam_dma/sources-dmgABCmgbS") }

#[test]
fn ppu_hblank_ly_scx_timing_gs() { run_test("acceptance/ppu/hblank_ly_scx_timing-GS") }

#[test]
fn ppu_intr_1_2_timing_gs() { run_test("acceptance/ppu/intr_1_2_timing-GS") }

#[test]
fn ppu_intr_2_0_timing() { run_test("acceptance/ppu/intr_2_0_timing") }

#[test]
fn ppu_intr_2_mode0_timing() { run_test("acceptance/ppu/intr_2_mode0_timing") }

#[test]
#[ignore]
fn ppu_stat_lyc_onoff() { run_test("acceptance/ppu/stat_lyc_onoff") }

#[test]
#[ignore]
fn ppu_intr_2_mode0_timing_sprites() { run_test("acceptance/ppu/intr_2_mode0_timing_sprites") }

#[test]
fn ppu_intr_2_mode3_timing() { run_test("acceptance/ppu/intr_2_mode3_timing") }

#[test]
fn ppu_intr_2_oam_ok_timing() { run_test("acceptance/ppu/intr_2_oam_ok_timing") }

#[test]
#[ignore]
fn ppu_lcdon_timing_dmg_abc_mgb_s() { run_test("acceptance/ppu/lcdon_timing-dmgABCmgbS") }

#[test]
#[ignore]
fn ppu_lcdon_write_timing_gs() { run_test("acceptance/ppu/lcdon_write_timing-GS") }

#[test]
#[ignore]
fn ppu_stat_irq_blocking() { run_test("acceptance/ppu/stat_irq_blocking") }

#[test]
fn ppu_vblank_stat_intr_gs() { run_test("acceptance/ppu/vblank_stat_intr-GS") }

#[test]
#[ignore]
fn serial_boot_sclk_align_dmg_abc_mgb() { run_test("acceptance/serial/boot_sclk_align-dmgABCmgb") }

#[test]
fn timer_div_write() { run_test("acceptance/timer/div_write") }

#[test]
fn timer_rapid_toggle() { run_test("acceptance/timer/rapid_toggle") }

#[test]
fn timer_tim00() { run_test("acceptance/timer/tim00") }

#[test]
fn timer_tim00_div_trigger() { run_test("acceptance/timer/tim00_div_trigger") }

#[test]
fn timer_tim01() { run_test("acceptance/timer/tim01") }

#[test]
fn timer_tim01_div_trigger() { run_test("acceptance/timer/tim01_div_trigger") }

#[test]
fn timer_tim10() { run_test("acceptance/timer/tim10") }

#[test]
fn timer_tim10_div_trigger() { run_test("acceptance/timer/tim10_div_trigger") }

#[test]
fn timer_tim11() { run_test("acceptance/timer/tim11") }

#[test]
fn timer_tim11_div_trigger() { run_test("acceptance/timer/tim11_div_trigger") }

#[test]
fn timer_tima_reload() { run_test("acceptance/timer/tima_reload") }

#[test]
fn timer_tima_write_reloading() { run_test("acceptance/timer/tima_write_reloading") }

#[test]
fn timer_tma_write_reloading() { run_test("acceptance/timer/tma_write_reloading") }

#[test]
fn mbc1_bits_ram_en() { run_test("emulator-only/mbc1/bits_ram_en") }

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

fn run_test_with_model(name: &str, model: Model) {
  let bootrom = Bootrom::lookup(&[model])
    .unwrap_or_else(|| panic!("No boot ROM found ({:?})", model));

  let test_name = format!("../tests/build/{}.gb", name);
  let cartridge_path = PathBuf::from(&test_name);
  let cartridge = Cartridge::from_path(&cartridge_path).unwrap();

  let hardware_config = HardwareConfig {
    model: model,
    bootrom: Some(bootrom.data),
    cartridge: cartridge
  };

  let max_duration = Duration::from_secs(120);
  let start_time = Instant::now();
  let pulse_duration = EmuTime::from_machine_cycles(1_000_000);

  let mut machine = Machine::new(hardware_config);
  let mut registers = None;
  let mut emu_time = EmuTime::zero();
  loop {
    let time = Instant::now();
    if time - start_time > max_duration {
      break;
    }
    let (events, end_time) = machine.emulate(emu_time + pulse_duration);
    emu_time = end_time;
    if events.contains(EmuEvents::DEBUG_OP) {
      registers = Some(machine.regs());
      break;
    }
  }
  match registers {
    None => panic!("Test did not finish ({:?})", model),
    Some(regs) => {
      if regs.a != 0 {
        panic!("{} assertion failures in hardware test ({:?})", regs.a, model);
      }
      if regs.b != 3  || regs.c != 5  ||
         regs.d != 8  || regs.e != 13 ||
         regs.h != 21 || regs.l != 34 {
        panic!("Hardware test failed ({:?})", model);
      }
    }
  }
}

fn run_test_with_models(name: &str, models: &[Model]) {
  for &model in models {
    run_test_with_model(name, model);
  }
}

fn run_test(name: &str) {
  run_test_with_models(name, &DEFAULT_MODEL_PRIORITY);
}
