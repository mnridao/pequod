use crate::geometry::grid::Grid;
use crate::geometry::boundaries::Boundaries;

pub struct Simulation {
    // grid: Grid,
    // boundaries: Boundaries,

    // // Solver settings 
    // nt: usize,
    // dt: f64,

    // TODO: 
    // Domain type (contains grid, boundaries, )
    // QGModel type (contains access to equations)
    // CabaretStepper type 
    // Fields (mutable)
    // Clock (keeps track of the simulation time)
}

impl Simulation {
    // pub fn new(grid: Grid, boundaries: Boundaries, nt: usize, dt: f64) -> Self {
    //     println!("Creating new Simulation type");
    //     Self {grid, boundaries, nt, dt}
    // }

    pub fn new() -> Self {
        println!("Creating new Simulation");
        Self {}
    }

    pub fn run(&self) {
        println!("Initialising model.");
        println!("Running Cabaret algorithm.");
        println!("Saving output.");
        println!("Model finished running.")
    }
}