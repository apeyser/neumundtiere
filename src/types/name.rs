use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Clone)]
pub struct InternTable {
    table: Rc<RefCell<HashSet<Rc<String>>>>,
}

impl InternTable {
    pub fn new() -> Self {
        InternTable{table: Rc::new(RefCell::new(HashSet::new()))}
    }

    pub fn remove(&mut self, string: &String) {
        self.table.borrow_mut().remove(string);
    }

    pub fn intern(&mut self, string: String) -> Name {
        if let Some(string) = self.table.borrow_mut().get(&string) {
            return Name {string: string.clone(), table: self.clone()}
        }

        let name = Name {string: Rc::new(string), table: self.clone()};
        self.insert(&name.string);
        name
    }

    fn insert(&mut self, string: &Rc<String>) {
        self.table.borrow_mut().insert(string.clone());
    }
}


#[derive(Clone)]
pub struct Name {
    string: Rc<String>,
    table: InternTable,
}

impl Drop for Name {
    fn drop(&mut self) {
        let string = &self.string;
        if Rc::strong_count(string) == 2 {
            self.table.remove(&**string);
        }
    }
}

impl Into<String> for Name {
    fn into(self) -> String {
        String::clone(&*self.string)
    }
}

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.string)
    }
}

impl Borrow<String> for Name {
    fn borrow(&self) -> &String {
        &*self.string
    }
}

impl Hash for Name {
    fn hash<H>(&self, state: &mut H)
      where H: Hasher {
        self.string.hash(state)
    }
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.string == other.string
    }
}

impl PartialEq<String> for Name {
    fn eq(&self, other: &String) -> bool {
        &*self.string == other
    }
}

impl PartialEq<Name> for String {
    fn eq(&self, other: &Name) -> bool {
        self == &*other.string
    }
}

impl Eq for Name {}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &*self.string)
    }
}
