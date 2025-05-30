// OrbitKit Component Library (now part of orbit)

pub mod components;
pub mod theme;
pub mod utils;

/// Version of the OrbitKit library (deprecated, use orbit version)
pub const VERSION: &str = env!("CARGO_PKG_VERSION"); // This will now refer to orbit's version

/// Re-export of common components for convenience
pub mod prelude {
    pub use crate::kit::components::button::Button;
    pub use crate::kit::components::card::Card;
    pub use crate::kit::components::input::Input;
    pub use crate::kit::components::layout::Layout;
    pub use crate::kit::theme::{Theme, ThemeProvider};
}
