use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::sync::Mutex;
use std::collections::{HashSet, HashMap};

use once_cell::sync::Lazy;

use crate::vm::Frame;
use crate::error::*;

use super::list::{List, ListFmt};
use super::dict::{Dict, DictFmt};
use super::save::{Save, SaveBox};
use super::name::Name;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AnyFmt {
    List(ListFmt),
    Dict(DictFmt),
}

impl From<ListFmt> for AnyFmt {
    fn from(item: ListFmt) -> Self {
        Self::List(item)
    }
}

impl From<DictFmt> for AnyFmt {
    fn from(item: DictFmt) -> Self {
        Self::Dict(item)
    }
}

impl Hash for AnyFmt {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        match self {
            AnyFmt::List(list) => list.hash(hasher),
            AnyFmt::Dict(dict) => dict.hash(hasher),
        }
    }
}

pub static PENDING: Lazy<Mutex<HashSet<AnyFmt>>> = Lazy::new(|| Mutex::new(HashSet::new()));

#[derive(Clone, Debug)]
pub enum Saved {
    List(Vec<Frame>),
    Dict(HashMap<Name, Frame>),
    Save(Save),
}

pub trait Unwrap<T> {
    fn unwrap(&self) -> &T;
    fn unwrap_mut(&mut self) -> &mut T;
}

pub trait Wrap {
    fn wrap(self) -> Saved;
}

pub trait HasNew {    
    fn new(parent: &Rc<RefCell<Saved>>) -> Self;

    fn get_parent(&self) -> Result<Rc<RefCell<Saved>>, Error> {
        let Some(parent) = self.weak_parent().upgrade() else {
            return Error::Dropped.into()
        };
        Ok(parent)
    }

    fn weak_parent(&self) -> Weak<RefCell<Saved>>;
}

#[derive(Clone, Debug)]
pub struct RcSaved(Rc<RefCell<Saved>>);

impl RcSaved {
    fn new(saved: Saved) -> Self {
        RcSaved(Rc::new(RefCell::new(saved)))
    }
}

impl PartialEq for RcSaved {
    fn eq(&self, other: &RcSaved) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Hash for RcSaved {
    fn hash<H: Hasher>(&self, hasher: &mut H) { 
        hasher.write_usize(Rc::as_ptr(&self.0) as usize);
    }
}

pub trait Intern: Sized {
    type Interned: HasNew;

    fn wrap(self) -> Saved;
    
    fn intern(self, save: &mut Save) -> Self::Interned {
        let saved = RcSaved::new(self.wrap());
        save.insert(&saved);
        Self::Interned::new(&saved.0)
    }
}

impl Unwrap<Vec<Frame>> for Saved {
    fn unwrap(&self) -> &Vec<Frame> {
        #[allow(unreachable_patterns)]
        match self {
            Saved::List(ref list) => list,
            _ => panic!("Unwrapping Vec<Frame> from a non-list"),
        }
    }
    fn unwrap_mut(&mut self) -> &mut Vec<Frame> {
        match self {
            Saved::List(ref mut list) => list,
            _ => panic!("Unwrapping Vec<Frame> from a non-list"),
        }
    }
}

impl Intern for Vec<Frame> {
    type Interned = List;

    fn wrap(self) -> Saved {
        Saved::List(self)
    }
}

impl Unwrap<HashMap<Name, Frame>> for Saved {
    fn unwrap(&self) -> &HashMap<Name, Frame> {
        match self {
            Saved::Dict(ref dict) => dict,
            _ => panic!("Unwrapping HashMap<Name, Frame> from a non-dict"),
        }
    }
    fn unwrap_mut(&mut self) -> &mut HashMap<Name, Frame> {
        match self {
            Saved::Dict(ref mut dict) => dict,
            _ => panic!("Unwrapping HashMap<Name, Frame> from a non-dict"),
        }
    }
}

impl Intern for HashMap<Name, Frame> {
    type Interned = Dict;

    fn wrap(self) -> Saved {
        Saved::Dict(self)
    }
}

impl Unwrap<Save> for Saved {
    fn unwrap(&self) -> &Save {
        match self {
            Saved::Save(ref save) => save,
            _ => panic!("Unwrapping Saver from a non-save"),
        }
    }
    
    fn unwrap_mut(&mut self) -> &mut Save {
        match self {
            Saved::Save(ref mut save) => save,
            _ => panic!("Unwrapping Save from a non-save"),
        }
    }
}

impl Intern for Save {
    type Interned = SaveBox;

    fn wrap(self) -> Saved {
        Saved::Save(self)
    }
}
