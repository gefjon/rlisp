use std::collections::HashMap;
use types::*;
use gc::{GarbageCollected, GcMark};
use std::{collections, convert, fmt};

#[derive(Default)]
pub struct Namespace {
    pub gc_marking: GcMark,
    pub name: Option<Object>,
    table: HashMap<*const Symbol, Object>,
}

pub type Scope = Vec<*mut Namespace>;

impl convert::From<HashMap<*const Symbol, Object>> for Namespace {
    fn from(table: HashMap<*const Symbol, Object>) -> Self {
        Self {
            gc_marking: 0,
            table,
            name: None,
        }
    }
}

impl Namespace {
    pub fn with_name(mut self, name: Object) -> Self {
        self.name = Some(name);
        self
    }
    pub fn with_maybe_name(mut self, name: Option<Object>) -> Self {
        self.name = name;
        self
    }
    pub fn iter(&self) -> collections::hash_map::Iter<*const Symbol, Object> {
        self.table.iter()
    }
    pub fn len(&self) -> usize {
        self.table.len()
    }
    pub fn get_mut(&mut self, key: &*const Symbol) -> Option<&mut Object> {
        self.table.get_mut(key)
    }
    pub fn sym_ref(&mut self, sym: *const Symbol) -> Place {
        let place: &mut Object = self.table.entry(sym).or_insert_with(Object::nil);
        Place::from(place as *mut Object)
    }
    pub fn contains_key(&self, key: &*const Symbol) -> bool {
        self.table.contains_key(key)
    }
    pub fn get(&self, key: &*const Symbol) -> Option<&Object> {
        self.table.get(key)
    }
    pub fn insert(&mut self, key: *const Symbol, val: Object) -> Option<Object> {
        self.table.insert(key, val)
    }
    pub fn flatten_scope(scope: &[*mut Namespace]) -> Namespace {
        let count = {
            let mut ct = 0;
            for namespace in scope {
                let namespace = unsafe { &**namespace };
                ct += namespace.len();
            }
            ct
        };
        let mut new = HashMap::with_capacity(count);
        let mut iter = scope.iter();
        while let Some(namespace) = iter.next_back() {
            let namespace = unsafe { &**namespace };
            for (key, value) in namespace.iter() {
                if !new.contains_key(key) {
                    let _insert_res = new.insert(*key, *value);
                    debug_assert!(_insert_res.is_none());
                }
            }
        }
        new.shrink_to_fit();
        Namespace {
            gc_marking: 0,
            table: new,
            name: None,
        }
    }
}

impl fmt::Display for Namespace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(name) = self.name {
            write!(f, "<namespace {}>", name)
        } else {
            write!(f, "<anonymous namespace>")
        }
    }
}

impl fmt::Debug for Namespace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        writeln!(f, "{} {{", self)?;
        for (key, val) in self.iter() {
            writeln!(f, "({} . {})", Object::from(*key), val)?;
        }
        writeln!(f, "}}")
    }
}

impl GarbageCollected for Namespace {
    fn my_marking(&self) -> &GcMark {
        &self.gc_marking
    }
    fn my_marking_mut(&mut self) -> &mut GcMark {
        &mut self.gc_marking
    }
    fn gc_mark_children(&mut self, mark: GcMark) {
        if let Some(name) = self.name {
            name.gc_mark(mark);
        }
        for (sym, obj) in self.iter() {
            unsafe {
                (*((*sym) as *mut Symbol)).gc_mark(mark);
            }
            obj.gc_mark(mark);
        }
    }
}

impl FromUnchecked<Object> for *mut Namespace {
    unsafe fn from_unchecked(obj: Object) -> *mut Namespace {
        debug_assert!(obj.namespacep());
        ObjectTag::Namespace.untag(obj.0) as *mut Namespace
    }
}

impl FromObject for *mut Namespace {
    fn rlisp_type() -> RlispType {
        RlispType::Namespace
    }
}
