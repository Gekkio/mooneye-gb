use glium::Surface;
use glium::backend::Facade;
use imgui::{ImGui, ImGuiSetCond_Always};
use imgui::glium_renderer::{Renderer, RendererError};
use sdl2::mouse::{MouseUtil};

use super::{FrontendError, FrontendResult};

impl From<RendererError> for FrontendError {
  fn from(e: RendererError) -> FrontendError {
    FrontendError::Renderer(format!("{}", e))
  }
}

pub struct Gui {
  imgui: ImGui,
  renderer: Renderer,
  show_perf_overlay: bool
}

impl Gui {
  pub fn init<F: Facade>(ctx: &F) -> FrontendResult<Gui> {
    let mut imgui = ImGui::init();
    imgui.draw_mouse_cursor(false);
    let renderer = try!(Renderer::init(&mut imgui, ctx));
    Ok(Gui {
      imgui: imgui,
      renderer: renderer,
      show_perf_overlay: false
    })
  }
  pub fn render<S: Surface>(&mut self, surface: &mut S, delta_time: f32, mouse: &MouseUtil,
                            fps: f64, perf: f64) -> FrontendResult<()> {
    let (width, height) = surface.get_dimensions();
    self.imgui.update_mouse(mouse);

    {
      let frame = self.imgui.frame(width, height, delta_time);

      if self.show_perf_overlay {
        frame.window()
          .name(im_str!("Performance overlay"))
          .bg_alpha(0.4)
          .title_bar(false)
          .resizable(false)
          .movable(false)
          .always_auto_resize(true)
          .position((0.0, 0.0), ImGuiSetCond_Always)
          .build(|| {
            frame.text(im_str!("FPS: {:.0}, speed: {:.0} %", fps, perf));
          });
      }
      try!(self.renderer.render(surface, frame));
    }

    Ok(())
  }
  pub fn toggle_perf_overlay(&mut self) {
    self.show_perf_overlay = !self.show_perf_overlay;
  }
}
