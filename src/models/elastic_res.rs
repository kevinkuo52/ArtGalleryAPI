use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct DocES <T>{
    pub _index: String,
    pub _type: String,
    pub _id: String,
    pub _score: f32,
    pub _source: T, 
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct TotalRes {
    pub value: u32
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Hits <T>{
    pub total: TotalRes,
    pub hits: Vec<DocES<T>>
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SearchResES<T>{
    pub hits: Hits<T>,
}




