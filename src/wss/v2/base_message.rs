use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseMessage<T>
where
    T: Serialize + Deserialize + Debug,
{
    pub method: String,
    pub params: T,
    pub req_id: i64,
}
