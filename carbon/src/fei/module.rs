use super::{Metadata, Core, Feature};

/* Module struct */
pub struct Module {
    pub core: Box<dyn Core>,
    pub metadata: Metadata,
    pub extensions: Vec<&'static str>,
    pub features: Option<Vec<Feature>>
}