pub struct Grid {
    nx: usize,
    ny: usize,
    basinscale: f64,
}

impl Grid {
    pub fn new(nx: usize, ny: usize, basinscale: f64) -> Self {
        Self {nx, ny, basinscale}
    }

    pub fn nx(&self) -> usize {
        self.nx
    }

    pub fn ny(&self) -> usize {
        self.ny
    }

    pub fn basinscale(&self) -> f64 {
        self.basinscale
    }
}