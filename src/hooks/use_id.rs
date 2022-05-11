use crate::react_bindings;

/// Returns a unique component ID which is stable across server and client.
pub fn use_id() -> String {
  react_bindings::use_id()
}
