use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt;

use crate::error::Error;

use super::savable::{Saved, Unwrap, HasNew, Intern, RcSaved};

#[derive(Clone, Debug)]
pub struct Save {
    savebox: Vec<RcSaved>,
}

impl Save {
    pub fn new() -> Self {
        Save {savebox: Vec::<_>::new()}
    }

    pub fn insert(&mut self, saved: &RcSaved) {
        self.savebox.push(saved.clone());
    }
}

#[derive(Debug, Clone)]
pub struct SaveBox {
    parent: Weak<RefCell<Saved>>,
    _pinned: Option<Rc<RefCell<Saved>>>,
}

impl PartialEq for SaveBox {
    fn eq(&self, other: &Self) -> bool {
        match (self.parent.upgrade(), other.parent.upgrade()) {
            (Some(ref parent), Some(ref other)) => Rc::ptr_eq(parent, other),
            (None, None) => true,
            _ => false,
        }
    }
}

impl HasNew for SaveBox {
    fn new(parent: &Rc<RefCell<Saved>>) -> Self {
        Self {parent: Rc::<_>::downgrade(parent), _pinned: None}
    }

    fn weak_parent(&self) -> Weak<RefCell<Saved>> {
        self.parent.clone()
    }
}

impl SaveBox {
    pub fn base() -> Self {
        let parent = Rc::new(RefCell::new(Saved::Save(Save::new())));
        Self {_pinned: Some(parent.clone()), ..Self::new(&parent)}
    }

    pub fn len(&self) -> Result<usize, Error> {
        let parent = self.get_parent()?;
        let saved = &*parent.borrow();
        let save: &Save = saved.unwrap();
        Ok(save.savebox.len())
    }

    pub fn put<T: Intern>(&mut self, obj: T) -> Result<T::Interned, Error> {
        let parent = self.get_parent()?;
        let parent = &mut *parent.borrow_mut();
        let save: &mut Save = parent.unwrap_mut();
        Ok(obj.intern(save))
    }
}

impl fmt::Display for SaveBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.parent.upgrade() {
            None => write!(f, "-- Dropped --"),
            Some(parent) => {
                let Saved::Save(ref save) = *parent.borrow() else {
                    panic!("Impossible object as list");
                };

                write!(f, "len={}", save.savebox.len())
            },
        }
    }
}
