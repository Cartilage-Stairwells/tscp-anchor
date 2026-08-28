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



impl Expr {
    /// Maximum nesting depth of this expression tree.
    /// Used to prevent stack overflow during evaluation.
    pub fn depth(&self) -> usize {
        match self {
            Expr::Const { .. } | Expr::Var { .. } => 1,
            Expr::Add { left, right } | Expr::Mul { left, right } => {
                1 + left.depth().max(right.depth())
            }
        }
    }
}

impl PolyIR {
    pub fn verify_schema(&self) -> Result<(), String> {
        if self.version.is_empty() {
            return Err("missing version".into());
        }
        // Validate each constraint's expression for depth and content
        for constraint in &self.constraints {
            if constraint.name.is_empty() {
                return Err("constraint with empty name".into());
            }
            let depth = constraint.expr.depth();
            if depth > 100 {
                return Err(format!(
                    "constraint '{}' expression depth {} exceeds limit 100",
                    constraint.name, depth
                ));
            }
        }
        Ok(())
    }
}
