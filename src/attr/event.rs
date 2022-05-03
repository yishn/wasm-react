use super::Attr;
use web_sys::MouseEvent;

impl Attr {
  pub fn on_click(self, f: impl Fn(MouseEvent) + 'static) -> Self {
    self.insert_callback("onClick", f)
  }
}
