use super::Attr;
use crate::Callback;
use web_sys::MouseEvent;

impl Attr {
  pub fn on_click(self, f: impl Fn(MouseEvent) + 'static) -> Self {
    self.insert("onClick", Callback::new(f))
  }
}
