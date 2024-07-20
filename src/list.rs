use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::sync::Mutex;

use once_cell::sync::Lazy;

use super::error::Error;
use super::vm::Frame;
use super::save::{Saved, Unwrap};

#[derive(Debug, Clone)]
pub struct List {
    parent: Weak<RefCell<Saved>>,
    start: usize,
    len: usize,
}

impl PartialEq for List {
    fn eq(&self, other: &List) -> bool {
        self.start == other.start && self.len == other.len && {
            match (self.parent.upgrade(), other.parent.upgrade()) {
                (Some(ref parent), Some(ref other)) => Rc::ptr_eq(parent, other),
                (None, None) => true,
                _ => false,
            }
        }
    }
}

impl List {
    pub fn new(parent: &Rc<RefCell<Saved>>) -> Self {
        let saved = &*parent.borrow();
        let list = saved.unwrap();
        List {parent: Rc::<_>::downgrade(parent), start: 0, len: list.len()}
    }

    fn get_parent(&self) -> Result<Rc<RefCell<Saved>>, Error> {
        let Some(parent) = self.parent.upgrade() else {
            return Error::Dropped.into()
        };
        Ok(parent)
    }

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
        let list = saved.unwrap();
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
        let list = saved.unwrap_mut();
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
struct ListFmt {
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
        hasher.write_usize(self.parent as usize);
    }
}

static PENDING: Lazy<Mutex<HashSet<ListFmt>>> = Lazy::new(|| Mutex::new(HashSet::new()));

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.parent.upgrade() {
            None => write!(f, "-- Dropped --"),
            Some(parent) => {
                let listfmt = ListFmt::new(&parent, self.start, self.len);
                if PENDING.lock().unwrap().contains(&listfmt) {
                    return write!(f, "...")
                };

                PENDING.lock().unwrap().insert(listfmt);
                #[allow(irrefutable_let_patterns)]
                let Saved::List(ref list) = *parent.borrow() else {
                    panic!("Impossible object as list");
                };
                
                let r = match self.len {
                    0   => write!(f, ""),
                    1   => write!(f, "{}", list[self.start]),
                    len => {
                        for frame in &list[self.start..self.start+len-1] {
                            write!(f, "{frame} ")?;
                        };
                        write!(f, "{}", &list[self.start+len-1])
                    },
                };
                PENDING.lock().unwrap().remove(&listfmt);
                r
            },
        }
    }
}
