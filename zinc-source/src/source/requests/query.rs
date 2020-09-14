//!
//! The contract resource query PUT request.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value as JsonValue;

///
/// The contract resource query PUT request query.
///
#[derive(Debug, Deserialize)]
pub struct Query {
    /// The contract account ID.
    pub contract_id: i64,
    /// The name of the queried method. If not specified, the storage is returned.
    pub method: Option<String>,
}

impl Query {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(contract_id: i64, method: Option<String>) -> Self {
        Self {
            contract_id,
            method,
        }
    }

    ///
    /// Converts the query into an iterable list of arguments.
    ///
    pub fn into_vec(self) -> Vec<(&'static str, String)> {
        let mut result = Vec::with_capacity(2);
        result.push(("contract_id", self.contract_id.to_string()));
        if let Some(method) = self.method {
            result.push(("method", method));
        }
        result
    }
}

///
/// The contract resource query PUT request body.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    /// The JSON method input. Required for querying methods.
    pub arguments: Option<JsonValue>,
}

impl Body {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(arguments: Option<JsonValue>) -> Self {
        Self { arguments }
    }
}
