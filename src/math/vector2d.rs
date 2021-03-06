#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Vector2d<T> {
    pub i: T,
    pub j: T
}

impl<T> Vector2d<T> {
    pub fn new(i: T, j: T) -> Vector2d<T> {
        Vector2d { i, j }
    }
}

impl<T: Default> Default for Vector2d<T> {
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}