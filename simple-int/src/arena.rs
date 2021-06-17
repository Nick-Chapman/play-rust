
use std::sync::Mutex;

pub struct Arena<T> {
    buffer: Mutex<Buffer<T>>,
}

impl<T:Copy> Arena<T> {
    pub fn new(t0: T) -> Arena<T> {
        Arena {
            buffer: Mutex::new(Buffer::new(t0)),
        }
    }
    pub fn alloc(&self, thing: T) -> &T {
        let buffer = &mut self.buffer.lock().unwrap();
        let next = buffer.next;
        buffer.next += 1;
        let hold = &mut buffer.array[next];
        *hold = thing;
        unsafe { & * (hold as *const T) }
    }
}

const BUFFER_SIZE : usize = 200;

struct Buffer<T> {
    next: usize,
    array: [T;BUFFER_SIZE],
}

impl<T:Copy> Buffer<T> {
    fn new(t0: T) -> Buffer<T> {
        Buffer { next: 0, array: [t0;BUFFER_SIZE] }
    }
}
