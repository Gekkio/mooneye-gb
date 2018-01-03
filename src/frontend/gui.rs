use imgui::{ImGuiCol, ImGuiCond, ImString, ImVec4, Ui};
use imgui_glium_renderer::RendererError;
use std::f32;

use config::HardwareConfig;
use super::FrontendError;

impl From<RendererError> for FrontendError {
  fn from(e: RendererError) -> FrontendError {
    FrontendError::Renderer(format!("{}", e))
  }
}

pub trait Screen {
  fn render(&mut self, ui: &Ui);
}

#[derive(Default)]
pub struct WaitBootromScreen {
  error: Option<ImString>
}

impl WaitBootromScreen {
  pub fn set_error(&mut self, text: String) {
    self.error = Some(ImString::new(text));
  }
}

impl Screen for WaitBootromScreen {
  fn render(&mut self, ui: &Ui) {
    ui.window(im_str!("Help overlay"))
      .title_bar(false)
      .resizable(false)
      .movable(false)
      .always_auto_resize(true)
      .position((f32::MIN, f32::MIN), ImGuiCond::Always)
      .build(|| {
        ui.text(im_str!("Mooneye GB requires a boot ROM to run"));
        ui.text(im_str!("Drag and drop here a boot rom of one of these types:"));
        ui.bullet_text(im_str!("Game Boy (usually called dmg_boot.bin)"));
        ui.bullet_text(im_str!("Game Boy Pocket (usually called mgb_boot.bin)"));

        if let Some(ref error) = self.error {
          ui.separator();
          ui.text_colored(ImVec4::new(1.0, 0.0, 0.0, 1.0), &error);
        }
      });
  }
}

pub struct InGameScreen {
  pub fps: f64,
  pub perf: f64,
  model: ImString,
  cartridge_title: ImString,
  show_info_overlay: bool
}

impl InGameScreen {
  pub fn new(config: &HardwareConfig) -> InGameScreen {
    InGameScreen {
      fps: 0.0,
      perf: 0.0,
      model: ImString::new(format!("{}", config.model)),
      cartridge_title: ImString::new(format!("{}", config.cartridge.title)),
      show_info_overlay: false
    }
  }
  pub fn toggle_info_overlay(&mut self) {
    self.show_info_overlay = !self.show_info_overlay;
  }
}

impl Screen for InGameScreen {
  fn render(&mut self, ui: &Ui) {
    if self.show_info_overlay {
      ui.with_color_var(ImGuiCol::WindowBg, (0.0, 0.0, 0.0, 0.4), || {
        ui.window(im_str!("Info overlay"))
          .title_bar(false)
          .resizable(false)
          .movable(false)
          .always_auto_resize(true)
          .position((0.0, 0.0), ImGuiCond::Always)
          .build(|| {
            ui.text(&self.model);
            ui.text(&self.cartridge_title);
            ui.text(im_str!("FPS: {:.0}, speed: {:.0} %", self.fps, self.perf));
          });
      });
    }
  }
}
