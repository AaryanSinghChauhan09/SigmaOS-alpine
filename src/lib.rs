// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-alpine - Multimedia/Office/Networking component
//! Part of SigmaOS multi-repository architecture


/// Component
pub struct Component {
    name: &'static str,
}

impl Component {
    pub fn new() -> Self {
        Self { name: "SigmaOS-alpine" }
    }
}

impl Default for Component {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_creation() {
        let c = super::Component::new();
        assert_eq!(c.name, "SigmaOS-alpine");
    }
}

pub mod config;
pub mod error;
pub mod service;

pub use config::Config;
pub use error::{Error, Result};
