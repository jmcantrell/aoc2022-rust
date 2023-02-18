pub type Value = isize;

pub enum Expression {
    Value(Value),
    Function(Box<dyn FnOnce(Value) -> Value>),
}
