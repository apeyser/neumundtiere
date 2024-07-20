use std::rc::Rc;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};
//use std::borrow::Borrow;

use super::vm::Frame;
use super::list::List;

#[derive(Clone)]
pub enum Saved {
    List(Vec<Frame>),
}

#[derive(Clone)]
struct RcSaved(Rc<RefCell<Saved>>);

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

#[derive(Clone)]
pub struct Save {
    savebox: Rc<RefCell<Vec<RcSaved>>>,
}

impl Save {
    pub fn new() -> Self {
        Save {savebox: Rc::new(RefCell::new(Vec::<_>::new()))}
    }

    pub fn intern_list(&mut self, list: Vec<Frame>) -> List {
        let saved = RcSaved::new(Saved::List(list));
        self.insert(&saved);

        List::new(&saved.0)
    }

    fn insert(&mut self, saved: &RcSaved) {
        self.savebox.borrow_mut().push(saved.clone());
    }
}
