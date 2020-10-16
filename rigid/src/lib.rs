// Exposed so that the library is convenient to use for the end-user
pub use rigid_derive::FromJSON;
pub use rigid_runtime::{Error, Result};

// Exposed so that the generated code can find the runtime helpers
pub use rigid_runtime as runtime;
