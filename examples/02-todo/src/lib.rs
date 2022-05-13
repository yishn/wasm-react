use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use wasm_react::{
  c,
  callback::Callable,
  export_component, h,
  hooks::{use_callback, use_state, Deps},
  Component, VNode,
};
use web_sys::{Event, HtmlInputElement};

pub struct App;

impl TryFrom<JsValue> for App {
  type Error = JsValue;

  fn try_from(_: JsValue) -> Result<Self, Self::Error> {
    Ok(App)
  }
}

impl Component for App {
  fn render(&self) -> VNode {
    let tasks = use_state(|| vec![]);
    let text = use_state(|| "".to_string());

    let handle_submit = use_callback(
      {
        let mut tasks = tasks.clone();
        let mut text = text.clone();

        move |evt: Event| {
          evt.prevent_default();

          if !text.value().is_empty() {
            tasks.update(|tasks| tasks.push((false, text.value().clone())));
            text.set(|_| "".to_string());
          }
        }
      },
      Deps::none(),
    );

    let handle_input = use_callback(
      {
        let mut text = text.clone();

        move |evt: Event| {
          text.set(|_| {
            evt
              .current_target()
              .unwrap_throw()
              .dyn_into::<HtmlInputElement>()
              .unwrap_throw()
              .value()
          })
        }
      },
      Deps::none(),
    );

    h!(div[."app"]).build(c![
      h!(h1).build(c!["Todo"]),
      TaskList {
        tasks: tasks.owned(),
        on_change: Some({
          let mut tasks = tasks.clone();

          move |(id, done)| {
            tasks.update(|tasks| {
              tasks
                .get_mut(id)
                .map(|task: &mut (bool, String)| task.0 = done);
            })
          }
        }),
      },
      h!(form).on_submit(&handle_submit).build(c![
        h!(input)
          .placeholder("Add new item...")
          .value(&*text.value())
          .on_change(&handle_input)
          .build(c![]),
        " ",
        h!(button).html_type("submit").build(c!["Add"])
      ])
    ])
  }
}

export_component!(App);

struct TaskList<F: FnMut((usize, bool)) + Clone> {
  tasks: Rc<RefCell<Vec<(bool, String)>>>,
  on_change: Option<F>,
}

impl<F: FnMut((usize, bool)) + Clone + 'static> Component for TaskList<F> {
  fn render(&self) -> VNode {
    h!(div[."task-list"]).build(c![
      //
      h!(ul).build(c![
        ..self
          .tasks
          .borrow()
          .iter()
          .enumerate()
          .map(|(i, (done, description))| TaskItem {
            id: i,
            description: description.clone(),
            done: *done,
            on_change: self.on_change.clone(),
          })
          .map(VNode::from)
      ])
    ])
  }
}

struct TaskItem<F: FnMut((usize, bool)) + Clone> {
  id: usize,
  description: String,
  done: bool,
  on_change: Option<F>,
}

impl<F: FnMut((usize, bool)) + Clone + 'static> Component for TaskItem<F> {
  fn render(&self) -> VNode {
    let handle_change = use_callback(
      {
        let mut on_change = self.on_change.clone();
        let id = self.id;

        move |evt: Event| {
          on_change.call((
            id,
            evt
              .current_target()
              .unwrap_throw()
              .dyn_into::<HtmlInputElement>()
              .unwrap_throw()
              .checked(),
          ));
        }
      },
      Deps::all(),
    );

    h!(li[."task-item"]).build(c![
      //
      h!(label).build(c![
        h!(input)
          .html_type("checkbox")
          .checked(self.done)
          .on_change(&handle_change)
          .build(c![]),
        " ",
        if self.done {
          h!(del).build(c![&self.description])
        } else {
          (&self.description).into()
        },
      ])
    ])
  }
}
