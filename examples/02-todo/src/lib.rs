use std::rc::Rc;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use wasm_react::{
  callback::Callback,
  export_components, h,
  hooks::{use_callback, use_state, Deps},
  Component, VNode, ValueContainer,
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
    let text = use_state(|| Rc::<str>::from(""));

    let handle_submit = use_callback(
      {
        let mut tasks = tasks.clone();
        let mut text = text.clone();

        move |evt: Event| {
          evt.prevent_default();

          if !text.value().is_empty() {
            tasks.set(|mut tasks| {
              tasks.push((false, text.value().clone()));
              tasks
            });
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
              .into_boxed_str()
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
          tasks.set(|mut tasks| {
            tasks.get_mut(id).map(|task: &mut (bool, _)| task.0 = done);
            tasks
          })
        }
      },
      Deps::none(),
    );

    let result = h!(div[#"app"]).build((
      h!(h1).build("Todo"),
      //
      TaskList {
        tasks: tasks.clone().into(),
        on_change: Some(handle_task_change.into()),
      }
      .build(),
      //
      h!(form).on_submit(&handle_submit).build((
        h!(input)
          .placeholder("Add new item…")
          .value(&**text.value())
          .on_change(&handle_input)
          .build(()),
        " ",
        h!(button).html_type("submit").build(("Add",)),
      )),
    ));

    result
  }
}

export_components! {
  /// This is the entry component for our Todo application
  App
}

struct TaskList {
  tasks: ValueContainer<Vec<(bool, Rc<str>)>>,
  on_change: Option<Callback<(usize, bool)>>,
}

impl Component for TaskList {
  fn render(&self) -> VNode {
    h!(div[."task-list"]).build((
      //
      h!(ul).build(
        self
          .tasks
          .value()
          .iter()
          .enumerate()
          .map(|(i, (done, description))| {
            TaskItem {
              id: i,
              description: description.clone(),
              done: *done,
              on_change: self.on_change.clone(),
            }
            .memoized()
            .key(Some(i))
            .build()
          })
          .collect::<VNode>(),
      ),
    ))
  }
}

#[derive(Debug, PartialEq)]
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
      Deps::some((self.id, self.on_change.clone())),
    );

    h!(li[."task-item"]).build((
      //
      h!(label).build((
        h!(input)
          .html_type("checkbox")
          .checked(self.done)
          .on_change(&handle_change)
          .build(()),
        " ",
        if self.done {
          h!(del).build((&*self.description,))
        } else {
          (*self.description).into()
        },
      )),
    ))
  }
}
