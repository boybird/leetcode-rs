use std::marker::PhantomData;
use std::mem;

struct Slice<'a, T: 'a> {
    start: *const T,
    end: *const T,
    phantom: PhantomData<&'a T>,
}

fn borrow_vec<'a, T>(vec: &'a Vec<T>) -> Slice<'a, T> {
    let ptr = vec.as_ptr();
    Slice {
        start: ptr,
        end: unsafe { ptr.add(vec.len()) },
        phantom: PhantomData,
    }
}

struct ExternalResource<R> {
    resource_handle: *mut (),
    resource_type: PhantomData<R>,
}

fn main() {}
