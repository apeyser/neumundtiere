use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt;
use std::hash::{Hash, Hasher};

use itertools::Itertools;

use super::error::Error;
use super::vm::Frame;
use super::save::{self, Saved, Unwrap, HasNew};

#[derive(Debug, Clone)]
pub struct List {
    parent: Weak<RefCell<Saved>>,
    start: usize,
    len: usize,
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.len == other.len && {
            match (self.parent.upgrade(), other.parent.upgrade()) {
                (Some(ref parent), Some(ref other)) => Rc::ptr_eq(parent, other),
                (None, None) => true,
                _ => false,
            }
        }
    }
}

impl HasNew for List {
    fn new(parent: &Rc<RefCell<Saved>>) -> Self {
        let saved = &*parent.borrow();
        let list: &Vec<_> = saved.unwrap();
        Self {parent: Rc::<_>::downgrade(parent), start: 0, len: list.len()}
    }

    fn weak_parent(&self) -> Weak<RefCell<Saved>> {
        self.parent.clone()
    }
}

impl List {
    pub fn len(&self) -> Result<usize, Error> {
        let _ = self.get_parent()?;
        Ok(self.len)
    }

    pub fn get(&self, index: usize) -> Result<Frame, Error> {
        if index >= self.len {
            return Error::Range {len: self.len, index}.into();
        };

        let parent = self.get_parent()?;
        let saved = &*parent.borrow();
        let list: &Vec<_> = saved.unwrap();
        Ok(list[self.start+index].clone())
    }

    pub fn put(&mut self, index: usize, frame: Frame) -> Option<Error> {
        if index >= self.len {
            return Some(Error::Range {len: self.len, index})
        };

        let parent = match self.get_parent() {
            Err(err) => return Some(err),
            Ok(parent) => parent,
        };

        let saved = &mut *parent.borrow_mut();
        let list: &mut Vec<_> = saved.unwrap_mut();
        list[self.start+index] = frame.clone();
        None
    }

    pub fn range(&self, start: usize, len: usize) -> Result<Self, Error> {
        if start+len > self.len {
            return (Error::Range {len: self.len, index: start+len}).into()
        };

        let parent = self.get_parent()?;
        Ok(Self {
            parent: Rc::<_>::downgrade(&parent),
            start: self.start+start,
            len
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ListFmt {
    parent: usize,
    start: usize,
    len: usize,
}

impl ListFmt {
    pub fn new(parent: &Rc<RefCell<Saved>>, start: usize, len: usize) -> Self {
        Self {parent: parent.as_ptr() as usize, start, len}
    }
}

impl Hash for ListFmt {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.start.hash(hasher);
        self.len.hash(hasher);
        hasher.write_usize(self.parent);
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.parent.upgrade() {
            None => write!(f, "-- Dropped --"),
            Some(parent) => {
                let listfmt = ListFmt::new(&parent, self.start, self.len).into();
                if save::PENDING.lock().unwrap().contains(&listfmt) {
                    return write!(f, "...")
                };

                save::PENDING.lock().unwrap().insert(listfmt);
                let Saved::List(ref list) = *parent.borrow() else {
                    panic!("Impossible object as list");
                };

                let r = f.write_str(list.iter()
                                        .map(|frame| format!("{frame}"))
                                        .join(" ")
                                        .as_str());
                save::PENDING.lock().unwrap().remove(&listfmt);
                r
            },
        }
    }
}
