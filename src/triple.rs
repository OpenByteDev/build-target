use std::env::{self, VarError};

pub fn target_triple() -> Result<String, VarError> {
    env::var("TARGET")
} 
