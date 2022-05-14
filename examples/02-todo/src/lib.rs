use std::rc::Rc;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use wasm_react::{
  c,
  callback::Callback,
  export_component, h,
  hooks::{use_callback, use_state, Deps},
  Component, ReadOrState, VNode,
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
    let text = use_state(|| -> Rc<str> { "".into() });

    let handle_submit = use_callback(
      {
        let mut tasks = tasks.clone();
        let mut text = text.clone();

        move |evt: Event| {
          evt.prevent_default();

          if !text.value().is_empty() {
            tasks.update(|tasks| tasks.push((false, text.value().clone())));
            text.set(|_| "".into());
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
              .into()
          })
        }
      },
      Deps::none(),
    );

    let handle_task_change = use_callback(
      {
        let mut tasks = tasks.clone();

        move |(id, done): (usize, bool)| {
          tasks.update(|tasks| {
            tasks.get_mut(id).map(|task: &mut (bool, _)| task.0 = done);
          })
        }
      },
      Deps::none(),
    );

    h!(div[."app"]).build(c![
      h!(h1).build(c!["Todo"]),
      //
      TaskList {
        tasks: tasks.clone().into(),
        on_change: Some(handle_task_change.into()),
      },
      //
      h!(form).on_submit(&handle_submit).build(c![
        h!(input)
          .placeholder("Add new item...")
          .value(&**text.value())
          .on_change(&handle_input)
          .build(c![]),
        " ",
        h!(button).html_type("submit").build(c!["Add"])
      ])
    ])
  }
}

export_component!(App);

struct TaskList {
  tasks: ReadOrState<Vec<(bool, Rc<str>)>>,
  on_change: Option<Callback<(usize, bool)>>,
}

impl Component for TaskList {
  fn render(&self) -> VNode {
    h!(div[."task-list"]).build(c![
      //
      h!(ul).build(c![
        ..self
          .tasks
          .value()
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

struct TaskItem {
  id: usize,
  description: Rc<str>,
  done: bool,
  on_change: Option<Callback<(usize, bool)>>,
}

impl Component for TaskItem {
  fn render(&self) -> VNode {
    let handle_change = use_callback(
      {
        let id = self.id;

        self
          .on_change
          .clone()
          .unwrap_or_default()
          .premap(move |evt: Event| {
            (
              id,
              evt
                .current_target()
                .unwrap_throw()
                .dyn_into::<HtmlInputElement>()
                .unwrap_throw()
                .checked(),
            )
          })
          .to_closure()
      },
      Deps::some(self.on_change.clone()),
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
          h!(del).build(c![*self.description])
        } else {
          (*self.description).into()
        },
      ])
    ])
  }
}
