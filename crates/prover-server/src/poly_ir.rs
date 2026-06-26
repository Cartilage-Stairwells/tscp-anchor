use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolyIR {
    pub version: String,
    pub constraints: Vec<Constraint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub name: String,
    pub expr: Expr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op")]
pub enum Expr {
    Const { value: i64 },
    Var { name: String },
    Add { left: Box<Expr>, right: Box<Expr> },
    Mul { left: Box<Expr>, right: Box<Expr> },
}

impl PolyIR {
    pub fn verify_schema(&self) -> Result<(), String> {
        if self.version.is_empty() {
            return Err("missing version".into());
        }
        Ok(())
    }
}
