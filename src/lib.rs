use std::error::Error;

#[allow(dead_code)]
pub struct Shell<'a, 'b> {
    cmd: &'a str,
    args: &'b [&'a str],
}

impl<'a, 'b> Shell<'a, 'b> {
    pub fn new(p1: &'a str, p2: &'b [&'a str]) -> Self {
        Self { cmd: p1, args: p2 }
    }
}

pub type BoxedErr = Box<dyn Error + Send + Sync>;
pub trait Executor {
    fn run(&self) -> Result<Option<i32>, BoxedErr>;
}

impl<'a, 'b> Executor for Shell<'a, 'b> {
    fn run(&self) -> Result<Option<i32>, BoxedErr> {
        Ok(Some(32))
    }
}

pub fn execute_genrics(cmd: &impl Executor) -> Result<Option<i32>, BoxedErr> {
    cmd.run()
}

pub fn execute_trait_object(cmd: &dyn Executor) -> Result<Option<i32>, BoxedErr> {
    cmd.run()
}

pub fn execute_boxed_trait_object(cmd: Box<dyn Executor>) -> Result<Option<i32>, BoxedErr> {
    cmd.run()
}