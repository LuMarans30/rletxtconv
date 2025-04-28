#[derive(Debug, Clone)]
pub struct Universe {
    pub cells: Vec<bool>,
    pub width: usize,
    pub height: usize,
}

impl Universe {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cells: Vec::with_capacity(width * height),
            width,
            height,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<bool> {
        if row < self.height && col < self.width {
            self.cells.get(row * self.width + col).copied()
        } else {
            None
        }
    }
}
