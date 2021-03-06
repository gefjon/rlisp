/*
Every heap-allocated Object (currently ConsCell, Symbol, RlispString
and RlispFunc) has a GcMark. lisp::Lisp owns a GcMark which signals
the "correct" marking. Whenever the garbage collector runs, it
iterates through the stack and the symbols table and marks each object
accessible with the "correct" marking, iterates through the heap and
deallocs any object with the wrong marking, and then increments
lisp::Lisp.gc_marking.
*/

use types::*;
use lisp;
use std::mem;

pub type GcMark = usize;

pub trait GarbageCollector
    : lisp::stack_storage::Stack + lisp::allocate::AllocObject
// This trait is implemented by lisp::Lisp
// its methods amount to a simple mark+sweep garbage collector
{
    fn should_gc_run(&self) -> bool;
    fn update_gc_threshold(&mut self);
    fn current_marking(&self) -> GcMark;
    fn inc_gc_mark(&mut self);
    fn mark_scope(&mut self);
    fn mark_stack(&mut self) {
        for obj in self.stack_vec() {
            debug!("{} is accessible; marking it", obj);
            self.mark(*obj);
        }
    }
    fn mark(&self, obj: Object) {
        obj.gc_mark(self.current_marking());
    }
    fn sweep(&mut self) {
        let mut old_objs = mem::replace(self.objects_mut(), Vec::new());
        for obj in old_objs.drain(..) {
            if obj.should_dealloc(self.current_marking()) {
                debug!("{} is not marked, deallocating it", obj);
                unsafe { self.dealloc(obj) }
            } else {
                self.objects_mut().push(obj);
            }
        }
    }
    fn gc_pass(&mut self) {
        self.mark_stack();
        self.mark_scope();
        self.sweep();
        self.inc_gc_mark();
        self.update_gc_threshold();
    }
    fn gc_maybe_pass(&mut self) {
        if self.should_gc_run() {
            debug!("running the garbage collector");
            self.gc_pass();
        }
    }
}

impl GarbageCollector for lisp::Lisp {
    fn mark_scope(&mut self) {
        for namespace in &self.symbols {
            unsafe {
                (**namespace).gc_mark(self.current_marking());
            }
        }
    }
    fn should_gc_run(&self) -> bool {
        self.alloced_objects.len() > self.gc_threshold
    }
    fn update_gc_threshold(&mut self) {
        self.gc_threshold = 2 * self.alloced_objects.len();
    }
    fn current_marking(&self) -> GcMark {
        self.current_gc_mark
    }
    fn inc_gc_mark(&mut self) {
        self.current_gc_mark += 1
    }
}

pub trait GarbageCollected
// This trait is implemented by all Object subtypes which are heap-allocated
// Note that it does not actually include dealloc() ;
// that function is owned by Object
{
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
