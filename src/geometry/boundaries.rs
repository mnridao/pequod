pub struct Boundaries {
    north: BoundaryType, 
    east: BoundaryType, 
    south: BoundaryType, 
    west: BoundaryType
}

// Each boundary gets its own BoundaryType
pub enum BoundaryType {
    Slip, 
    NoSlip, 
    PartialSlip, 
    Periodic // Maybe not implemented for now
}