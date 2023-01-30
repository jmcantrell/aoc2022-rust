use super::Value;

pub enum Expression {
    Value(Value),
    Function(Box<dyn FnOnce(Value) -> Value>),
}
