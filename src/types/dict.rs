use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::borrow::Borrow;

use itertools::Itertools;

use crate::error::Error;
use crate::vm::Frame;

use super::name::Name;
use super::savable::{Saved, Unwrap, HasNew, PENDING};

pub struct Dict {
    parent: Weak<RefCell<Saved>>,
}

impl PartialEq for Dict {
    fn eq(&self, other: &Self) -> bool {
        match (self.parent.upgrade(), other.parent.upgrade()) {
            (Some(ref parent), Some(ref other)) => Rc::ptr_eq(parent, other),
            (None, None) => true,
            _ => false,
        }
    }
}

impl HasNew for Dict {
    fn new(parent: &Rc<RefCell<Saved>>) -> Self {
        Self {parent: Rc::<_>::downgrade(parent)}
    }
    
    fn weak_parent(&self) -> Weak<RefCell<Saved>> {
        self.parent.clone()
    }
}

impl Dict {
    pub fn len(&self) -> Result<usize, Error> {
        let parent = self.get_parent()?;
        let saved = &(*parent).borrow();
        let dict: &HashMap<Name, Frame> = saved.unwrap();
        Ok(dict.len())
    }

    pub fn cap(&self) -> Result<usize, Error> {
        let parent = self.get_parent()?;
        let saved = &(*parent).borrow();
        let dict: &HashMap<Name, Frame> = saved.unwrap();
        Ok(dict.capacity())
    }

    pub fn find(&self, name: &Name) -> Result<Option<Frame>, Error> {
        let parent = self.get_parent()?;
        let saved = &(*parent).borrow();
        let dict: &HashMap<Name, Frame> = saved.unwrap();
        if let Some(frame) = dict.get(name) {
            Ok(Some(frame.clone()))
        } else {
            Ok(None)
        }
    }    

    pub fn get(&self, name: Name) -> Result<Frame, Error> {
        if let Some(frame) = self.find(&name)? {
            Ok(frame.clone())
        } else {
            let s: &String = name.borrow();
            Err(Error::MissingKey(s.clone()))
        }
    }

    pub fn put(&mut self, name: Name, frame: Frame) -> Option<Error> {
        let parent = match self.get_parent() {
            Err(err) => return Some(err),
            Ok(parent) => parent,
        };

        let saved = &mut *parent.borrow_mut();
        let dict: &mut HashMap<Name, Frame> = saved.unwrap_mut();
        dict.insert(name, frame);
        None
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DictFmt {
    parent: usize,
}

impl DictFmt {
    pub fn new(parent: &Rc<RefCell<Saved>>) -> Self {
        Self {parent: parent.as_ptr() as usize}
    }
}

impl Hash for DictFmt {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write_usize(self.parent);
    }
}

impl fmt::Display for Dict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.parent.upgrade() {
            None => write!(f, "-- Dropped --"),
            Some(parent) => {
                let dictfmt = DictFmt::new(&parent).into();
                if PENDING.lock().unwrap().contains(&dictfmt) {
                    return write!(f, "...")
                };

                PENDING.lock().unwrap().insert(dictfmt);
                let Saved::Dict(ref dict) = *(*parent).borrow() else {
                    panic!("Impossible object as dict");
                };

                let r = f.write_str(dict.iter()
                                    .map(|(name, frame)| format!("/{name} {frame}"))
                                    .join(" ")
                                    .as_str());
                PENDING.lock().unwrap().remove(&dictfmt);
                r
            },
        }
    }
}
