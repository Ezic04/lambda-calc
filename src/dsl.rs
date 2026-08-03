use std::fmt::Display;

use gluon::ThreadExt;

#[derive(Debug, Clone)]
pub enum ExprValue {
    Int(i32),
    Str(String),
}

pub async fn eval_expr(vm: gluon::RootedThread, expr: String) -> Result<ExprValue, String> {
    vm.run_expr_async::<i32>("example", expr.as_str())
        .await
        .map(|(val, _)| ExprValue::Int(val))
        .map_err(|err| err.to_string())
}

impl Display for ExprValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprValue::Int(i) => write!(f, "{i}"),
            ExprValue::Str(s) => write!(f, "{s}"),
        }
    }
}
