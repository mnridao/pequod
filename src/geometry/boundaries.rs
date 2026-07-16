pub enum DomainTopology {
    ClosedBasin {
        north: WallCondition,
        south: WallCondition,
        east: WallCondition, 
        west: WallCondition,
    },
    ZonalChannel {
        north: WallCondition,
        south: WallCondition,
    }, 
    DoublyPeriodic,
}

// Each boundary gets its own BoundaryType
pub enum WallCondition {
    Slip, 
    NoSlip, 
    PartialSlip {slip_length: f64}, 
}