use types::*;
use lisp;
use std::mem;

pub type GcMark = usize;

pub trait GarbageCollector
    : lisp::stack_storage::Stack + lisp::allocate::AllocObject {
    fn current_marking(&self) -> GcMark;
    fn mark_all(&mut self) {
        for obj in self.stack_vec() {
            self.mark(*obj);
        }
    }
    fn mark(&self, obj: Object) {
        obj.gc_mark(self.current_marking());
    }
    fn clear(&mut self) {
        let mut old_objs = mem::replace(self.objects_mut(), Vec::new());
        for obj in old_objs.drain(..) {
            if obj.should_dealloc(self.current_marking()) {
                unsafe { obj.deallocate() }
            } else {
                self.objects_mut().push(obj);
            }
        }
    }
    fn inc_gc_mark(&mut self);
    fn mark_symbols(&mut self);
    fn gc_pass(&mut self) {
        self.mark_all();
        self.mark_symbols();
        self.clear();
        self.inc_gc_mark();
    }
}

impl GarbageCollector for lisp::Lisp {
    fn current_marking(&self) -> GcMark {
        self.current_gc_mark
    }
    fn inc_gc_mark(&mut self) {
        self.current_gc_mark += 1
    }
    fn mark_symbols(&mut self) {
        for sym in self.symbols.map.values() {
            unsafe {
                (*(*sym as *mut Symbol)).gc_mark(self.current_marking());
            }
        }
    }
}

pub trait GarbageCollected {
    fn my_marking(&self) -> &GcMark;
    fn my_marking_mut(&mut self) -> &mut GcMark;
    fn gc_mark_children(&mut self, mark: GcMark);
    fn gc_mark(&mut self, mark: GcMark) {
        if *(self.my_marking()) != mark {
            *(self.my_marking_mut()) = mark;
            self.gc_mark_children(mark);
        }
    }
    fn should_dealloc(&self, current_marking: GcMark) -> bool {
        *(self.my_marking()) != current_marking
    }
}
