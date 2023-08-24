pub mod ui;
mod metadata;
mod module;
mod core;
mod handler;
mod driver;
mod status;
mod features;
pub use metadata::*;
pub use module::*;
pub use self::core::*;
pub use handler::*;
pub use driver::*;
pub use status::*;
pub use features::*;

#[macro_export]
macro_rules! box_method {
    ($module:expr, $method:ident) => {
        $module.$method()
    }
}