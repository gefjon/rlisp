/*
Honestly, I recommend that you not look in this module. It's scary,
it's evil, it's very `unsafe`, and it's a mess.
*/

use std::heap::Heap;
use alloc::allocator::{Alloc, Layout};
use types::*;
use lisp;
use std::ptr::Unique;
use std::{mem, ptr};
pub trait AllocObject {
    fn alloc<T>(&mut self, to_alloc: T) -> Object
    where
        Object: ::std::convert::From<*mut T>,
    {
        // Allocate a T, where T is one of `ConsCell`, `RlispError`,
        // or `RlispFunc`. `RlispString` and `Symbol` should use the
        // methods `alloc_string` and `alloc_sym`
        let pointer = Heap.alloc_one().unwrap().as_ptr();
        unsafe {
            ptr::write(pointer, to_alloc);
        }
        let obj = Object::from(pointer);
        self.objects_mut().push(obj);
        obj
    }
    unsafe fn dealloc(&mut self, to_dealloc: Object) {
        // deallocate an object
        match to_dealloc {
            Object::Num(_) | Object::Bool(_) => debug!("deallocating {} is a no-op", to_dealloc),
            Object::Cons(c) => self.low_level_dealloc(c),
            Object::Sym(s) => self.dealloc_sym(s as _),
            Object::String(s) => self.dealloc_string(s as _),
            Object::Function(f) => self.low_level_dealloc(f),
            Object::Error(e) => self.low_level_dealloc(e),
        }
    }
    unsafe fn low_level_dealloc<T>(&mut self, to_dealloc: *const T) {
        // This is called for types created by `alloc`, but the
        // dynamically sized `Symbol` and `RlispString` have their own
        // dealloc methods, `dealloc_sym` and `dealloc_string`.
        let ptr = Unique::new(to_dealloc as *mut T).unwrap();
        Heap.dealloc_one(ptr);
    }
    fn alloc_sym(&mut self, to_alloc: &str) -> Object {
        // allocate a block of memory large enough to store the
        // headers of a symbol plus all of the bytes of `to_alloc` and
        // then initialize a symbol there
        use gc::GcMark;
        use types::symbol::Binding;
        let layout = Layout::from_size_align(
            mem::size_of::<GcMark>() + mem::size_of::<Binding>() + mem::size_of::<usize>()
                + (mem::size_of::<u8>() * to_alloc.len()),
            mem::align_of::<Symbol>(),
        ).unwrap();
        let pointer = (unsafe { Heap.alloc(layout) }.unwrap()) as *mut Symbol;

        unsafe {
            ptr::write(pointer as *mut GcMark, 0);
            ptr::write(
                (pointer as usize + mem::size_of::<GcMark>()) as _,
                Binding::default(),
            );
            ptr::write(
                (pointer as usize + mem::size_of::<GcMark>() + mem::size_of::<Binding>()) as _,
                to_alloc.len(),
            );
            let string_head = pointer as usize + mem::size_of::<GcMark>()
                + mem::size_of::<Binding>() + mem::size_of::<usize>();
            for (offset, byte) in to_alloc.bytes().enumerate() {
                ptr::write(
                    (string_head + (offset * mem::size_of::<u8>())) as *mut u8,
                    byte,
                );
            }
        }

        Object::from(pointer)
    }

    unsafe fn dealloc_sym(&mut self, to_dealloc: *mut Symbol) {
        // build a `Layout` for `to_dealloc` including its name
        // contents, then deallocate it
        use gc::GcMark;
        use types::symbol::Binding;
        let len = *((to_dealloc as usize + mem::size_of::<GcMark>() + mem::size_of::<Binding>())
            as *const usize);
        let layout = Layout::from_size_align(
            mem::size_of::<GcMark>() + mem::size_of::<Binding>() + mem::size_of::<usize>()
                + (mem::size_of::<u8>() * len),
            mem::align_of::<Symbol>(),
        ).unwrap();
        Heap.dealloc(to_dealloc as _, layout);
    }
    fn alloc_string(&mut self, to_alloc: &str) -> Object {
        // allocate a block of memory large enough to store the
        // headers of a string plus all of the bytes of `to_alloc` and
        // then initialize a string there
        use gc::GcMark;
        let layout = Layout::from_size_align(
            mem::size_of::<GcMark>() + mem::size_of::<usize>()
                + (mem::size_of::<u8>() * to_alloc.len()),
            mem::align_of::<RlispString>(),
        ).unwrap();
        let pointer = (unsafe { Heap.alloc(layout) }.unwrap()) as *mut RlispString;

        unsafe {
            ptr::write(pointer as *mut GcMark, 0);
            ptr::write(
                (pointer as usize + mem::size_of::<GcMark>()) as _,
                to_alloc.len(),
            );

            let string_head: usize =
                pointer as usize + mem::size_of::<GcMark>() + mem::size_of::<usize>();
            for (offset, byte) in to_alloc.bytes().enumerate() {
                ptr::write(
                    (string_head + (offset * mem::size_of::<u8>())) as *mut u8,
                    byte,
                );
            }
        }
        Object::from(pointer)
    }

    unsafe fn dealloc_string(&mut self, to_dealloc: *mut RlispString) {
        // build a `Layout` for `to_dealloc` including its contents,
        // then deallocate it
        use gc::GcMark;
        let len = *((to_dealloc as usize + mem::size_of::<GcMark>()) as *const usize);
        let layout = Layout::from_size_align(
            mem::size_of::<GcMark>() + mem::size_of::<usize>() + (mem::size_of::<u8>() * len),
            mem::align_of::<RlispString>(),
        ).unwrap();
        Heap.dealloc(to_dealloc as _, layout);
    }

    fn objects(&self) -> &Vec<Object>;
    fn objects_mut(&mut self) -> &mut Vec<Object>;
}
impl AllocObject for lisp::Lisp {
    fn objects(&self) -> &Vec<Object> {
        &self.alloced_objects
    }
    fn objects_mut(&mut self) -> &mut Vec<Object> {
        &mut self.alloced_objects
    }
}
