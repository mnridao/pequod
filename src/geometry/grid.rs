pub struct Grid {
    nx: usize,
    ny: usize,
    nz: usize,
    dx: f64,
    dy: f64
}

impl Grid {
    pub fn new(nx: usize, ny: usize, nz: usize, dx: f64, dy: f64) -> Self {
        Self {nx, ny, nz, dx, dy}
    }

    pub fn nx(&self) -> usize {
        self.nx
    }

    pub fn ny(&self) -> usize {
        self.ny
    }

    pub fn nz(&self) -> usize {
        self.nz
    }

    pub fn dx(&self) -> f64 {
        self.dx
    }

    pub fn dy(&self) -> f64 {
        self.dy
    }
}