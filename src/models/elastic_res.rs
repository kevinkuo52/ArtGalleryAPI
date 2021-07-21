use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct DocES <T>{
    pub _id: String,
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

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct IndexResES{
    pub _id: String,
}




