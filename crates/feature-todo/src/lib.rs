//! Todo feature: an in-memory list. Depends only on `essential-core`.
//!
//! It shows the shape of a feature crate: its own types, its own tests, errors from core.

use essential_core::{Error, Result};

/// One todo entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Todo {
    /// Stable id, starting at 1.
    pub id: u32,
    /// What needs doing.
    pub title: String,
    /// Whether it is finished.
    pub done: bool,
}

/// An in-memory todo list.
#[derive(Debug, Default)]
pub struct TodoList {
    items: Vec<Todo>,
    next_id: u32,
}

impl TodoList {
    /// Create an empty list.
    #[must_use]
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            next_id: 1,
        }
    }

    /// Add a todo and return its id.
    pub fn add(&mut self, title: &str) -> Result<u32> {
        let title = title.trim();
        if title.is_empty() {
            return Err(Error::Invalid("title must not be empty".into()));
        }
        let id = self.next_id;
        self.next_id += 1;
        self.items.push(Todo {
            id,
            title: title.to_owned(),
            done: false,
        });
        Ok(id)
    }

    /// Mark a todo as done.
    pub fn complete(&mut self, id: u32) -> Result<()> {
        self.items
            .iter_mut()
            .find(|todo| todo.id == id)
            .map(|todo| todo.done = true)
            .ok_or_else(|| Error::NotFound(format!("todo {id}")))
    }

    /// All todos in insertion order.
    #[must_use]
    pub fn items(&self) -> &[Todo] {
        &self.items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_and_completes() {
        let mut list = TodoList::new();
        let id = list.add("write docs").unwrap();
        list.complete(id).unwrap();
        assert!(list.items()[0].done);
    }

    #[test]
    fn ids_increase() {
        let mut list = TodoList::new();
        assert_eq!(list.add("a").unwrap(), 1);
        assert_eq!(list.add("b").unwrap(), 2);
    }

    #[test]
    fn rejects_empty_titles() {
        assert!(matches!(TodoList::new().add(" "), Err(Error::Invalid(_))));
    }

    #[test]
    fn completing_unknown_id_fails() {
        assert_eq!(
            TodoList::new().complete(9),
            Err(Error::NotFound("todo 9".into()))
        );
    }
}
