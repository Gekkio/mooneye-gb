use imgui::{ImGuiSetCond_Always, ImStr, ImVec4, Ui};
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
  error: Option<ImStr<'static>>
}

impl WaitBootromScreen {
  pub fn set_error(&mut self, text: String) {
    self.error = Some(text.into());
  }
}

impl Screen for WaitBootromScreen {
  fn render(&mut self, ui: &Ui) {
    ui.window(im_str!("Help overlay"))
      .title_bar(false)
      .resizable(false)
      .movable(false)
      .always_auto_resize(true)
      .position((f32::MIN, f32::MIN), ImGuiSetCond_Always)
      .build(|| {
        ui.text(im_str!("Mooneye GB requires a boot ROM to run"));
        ui.text(im_str!("Drag and drop here a boot rom of one of these types:"));
        ui.bullet_text(im_str!("Game Boy (usually called dmg_boot.bin)"));
        ui.bullet_text(im_str!("Game Boy Pocket (usually called mgb_boot.bin)"));

        if let Some(ref error) = self.error {
          ui.separator();
          ui.text_colored(ImVec4::new(1.0, 0.0, 0.0, 1.0), error.clone());
        }
      });
  }
}

pub struct WaitRomScreen {
  title: ImStr<'static>,
  error: Option<ImStr<'static>>
}

impl WaitRomScreen {
  pub fn new() -> WaitRomScreen {
    WaitRomScreen {
      title: im_str!("Mooneye GB v{}", ::VERSION),
      error: None
    }
  }
  pub fn set_error(&mut self, text: String) {
    self.error = Some(text.into());
  }
}

impl Screen for WaitRomScreen {
  fn render(&mut self, ui: &Ui) {
    ui.window(im_str!("Help overlay"))
      .title_bar(false)
      .resizable(false)
      .movable(false)
      .always_auto_resize(true)
      .position((f32::MIN, f32::MIN), ImGuiSetCond_Always)
      .build(|| {
        ui.text(self.title.clone());
        ui.separator();
        ui.text(im_str!("Drag and drop a Game Boy ROM file here to load it"));

        if let Some(ref error) = self.error {
          ui.separator();
          ui.text_colored(ImVec4::new(1.0, 0.0, 0.0, 1.0), error.clone());
        }
      });
  }
}

pub struct InGameScreen {
  pub fps: f64,
  pub perf: f64,
  model: ImStr<'static>,
  cartridge_title: ImStr<'static>,
  show_info_overlay: bool
}

impl InGameScreen {
  pub fn new(config: &HardwareConfig) -> InGameScreen {
    InGameScreen {
      fps: 0.0,
      perf: 0.0,
      model: im_str!("{}", config.model),
      cartridge_title: im_str!("{}", config.cartridge.title),
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
      ui.window(im_str!("Info overlay"))
        .bg_alpha(0.4)
        .title_bar(false)
        .resizable(false)
        .movable(false)
        .always_auto_resize(true)
        .position((0.0, 0.0), ImGuiSetCond_Always)
        .build(|| {
          ui.text(self.model.clone());
          ui.text(self.cartridge_title.clone());
          ui.text(im_str!("FPS: {:.0}, speed: {:.0} %", self.fps, self.perf));
        });
    }
  }
}
