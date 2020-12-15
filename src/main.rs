use std::future::Future;
use std::marker::PhantomData;

pub struct Task {
}

pub struct Executor<'a> {
    _marker: PhantomData<std::cell::UnsafeCell<&'a ()>>,
}

impl<'a> Executor<'a> {
    pub const fn new() -> Executor<'a> {
        Executor {
            _marker: PhantomData,
        }
    }

    pub fn spawn<T: 'a>(&self, future: impl Future<Output = T> + 'a) {
    }
}

unsafe impl Send for Executor<'_> {}
unsafe impl Sync for Executor<'_> {}

fn run<'a>(ex: &'a Executor<'a>) {
    let ex2 = ex.clone();
    ex2.spawn(async move {
        ex2.spawn(async {
        });
    });
    ex.spawn(async {});
}

fn main() {
    let e = Executor::new();
    run(&e);
}
