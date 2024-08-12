use pyo3::prelude::*;
use cube_lib::Cube;

#[pyclass]
pub struct CubeCore {
    inner: Cube,
}

#[pymethods]
impl CubeCore {
    #[new]
    pub fn new() -> Self {
        CubeCore {
            inner: Cube::new(),
        }
    }
    pub fn rotate(&mut self, operations: String) {
        self.inner.rots(operations.as_str());
    }

    pub fn get_start_time(&self) -> i64 {
        self.inner.start_time as i64
    }

    pub fn reset(&mut self){
        self.inner.reset();
    }

    pub fn get_cube(&self) -> [[[i8; 3]; 3];6]{
        [self.inner.front, self.inner.back, self.inner.left, self.inner.right, self.inner.up, self.inner.down]
    }

    pub fn get_last_step(&self) -> String {
        self.inner.last_step.clone()
    }

    pub fn scramble(&mut self, steps: u32) {
        self.inner.scramble(steps);
    }

    pub fn is_solved(&self) -> bool {
        self.inner.is_solved()
    }
}

#[pymodule]
fn cube_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<CubeCore>()?;
    Ok(())
}