use crate::geometry::domain::Domain;
use crate::config::SimulationConfig;

pub struct Simulation {

    // simulation_config: SimulationConfig,
    // domain: Domain,

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
    pub fn new(config: SimulationConfig) -> Self {
        println!("Creating new Simulation");
        println!("{config:#?}");
        Self {}

    //     let domain = Domain{

    //     }

    }

    pub fn run(&self) -> () {
        println!("Pequod is not implemented yet.")
    }
}