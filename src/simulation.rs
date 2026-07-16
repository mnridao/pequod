use crate::geometry::domain::Domain;

pub struct Simulation {
    // domain: Domain,

    // // Solver settings 
    // nt: usize,
    // dt: f64,

    // TODO: 
    // Domain type (owns fixed geometry, e.g. grid, boundaries, layers)
    // QGModel type (should describe the physical equations. Should own physical model parameters, maybe not the stencils themselves)
    // CabaretStepper type (should describe the numerical time integration) 
    // ModelStates (mutable)
    // Clock (keeps track of the simulation time)
    // OutputManager

    // Logger (observer pattern, AOP, proxy, decorator)
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