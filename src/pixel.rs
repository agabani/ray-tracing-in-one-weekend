pub struct Pixel {
    i: usize,
    j: usize,
}

impl Pixel {
    pub fn new(i: usize, j: usize) -> Self {
        Self { i, j }
    }

    pub fn i(&self) -> usize {
        self.i
    }

    pub fn j(&self) -> usize {
        self.j
    }
}
