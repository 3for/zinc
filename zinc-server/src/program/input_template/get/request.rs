//!
//! The program input template resource GET request.
//!

use serde_derive::Deserialize;

///
/// The program input template resource GET request.
///
#[derive(Debug, Deserialize)]
pub struct Request {
    /// The name of the requested program.
    pub name: String,
    /// The name of the requested entry within the program.
    pub entry: String,
}
