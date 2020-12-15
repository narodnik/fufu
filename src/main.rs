use std::future::Future;
use std::marker::PhantomData;

pub struct Executor<'a> {
    _marker: PhantomData<std::cell::UnsafeCell<&'a ()>>,
}

impl<'a> Executor<'a> {
    pub const fn new() -> Executor<'a> {
        Executor {
            _marker: PhantomData,
        }
    }

    pub fn spawn<T: Send + 'a>(&self, future: impl Future<Output = T> + Send + 'a) {
    }
}

impl Drop for Executor<'_> {
    fn drop(&mut self) {
    }
}

fn run<'a>(ex: &'a Executor<'a>) {
    // ...
}

fn main() {
    let e = Executor::new();
    run(&e);
}
