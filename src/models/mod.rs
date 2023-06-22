use serde_json::Value;

pub mod users;

pub trait IntoJson {
    fn into_json(self) -> Value;
}

pub fn vec_into_json<T: IntoJson>(vec: Vec<T>) -> Value {
    Value::Array(vec.into_iter().map(|x| x.into_json()).collect::<Vec<_>>())
}
