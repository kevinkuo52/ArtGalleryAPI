use serde::{Serialize, Deserialize};

pub type Token = String;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct RegistrationReq {
    pub email: String,
    pub password: String,
}

pub struct RegistrationRes {
    pub success: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct LoginReq {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct LoginRes {
    pub access_token: String,
}


#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Credential {
    pub email: String,
    pub auth_type: String,
    pub hashed_password: String,
}
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct CredentialES {

    pub _source: Credential
}


