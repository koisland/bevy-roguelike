#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Vector2Int {
    pub x: i32,
    pub y: i32,
}

impl Vector2Int {
    pub fn new(x: i32, y: i32) -> Self {
        Vector2Int { x, y }
    }
}
