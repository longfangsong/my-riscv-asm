use std::ops::Index;
use core::fmt;
use std::fmt::{Formatter, Error};
use core::fmt::Debug;

pub struct Immediate {
    content: u32
}

impl fmt::Binary for Immediate {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        return self.content.fmt(f);
    }
}

impl<'a> Index<&'a usize> for Foo {
    type Output = u8;
    fn index(&self, i: &&'a usize) -> bool {
        self.content & (1 << i as i32)
    }
}

impl Immediate {}