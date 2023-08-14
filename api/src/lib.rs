mod handler;
pub use handler::*;

mod features;
pub use features::*;

mod metadata;
pub use metadata::*;

pub struct Module {
    pub core: Box<dyn Core>,
    pub metadata: Metadata,
    pub features: Option<Vec<Feature>>
}

macro_rules! console_method {
    ($obj: expr, $method: ident) => {
        $obj.$method()
    };
}