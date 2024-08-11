#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
use cube_lib::{Cube};

#[napi(js_name = "CubeCore")]
pub struct CubeCore {
    inner: Cube,
}

#[napi]
impl CubeCore {
    #[napi(constructor)]
    pub fn new() -> Self {
        CubeCore {
            inner: Cube::new(),
        }
    }
    #[napi]
    pub fn rotate(&mut self, operations: String) {
        self.inner.rots(operations.as_str());
    }

    #[napi]
    pub fn get_start_time(&self) -> i64 {
        self.inner.start_time as i64
    }

    #[napi]
    pub fn reset(&mut self){
        self.inner.reset();
    }

    #[napi]
    pub fn get_cube(&self) -> [[[i8; 3]; 3];6]{
        [self.inner.front, self.inner.back, self.inner.left, self.inner.right, self.inner.up, self.inner.down]
    }

    #[napi]
    pub fn get_last_step(&self) -> String {
        self.inner.last_step.clone()
    }
}