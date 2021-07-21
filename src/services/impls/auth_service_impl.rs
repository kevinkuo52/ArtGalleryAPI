use crate::{
    models::auth::*,
    models::error::ServiceError,
    repositories::credential_repo_i::CredentialRepo,
    repositories::user_repo_i::UserRepo,
    services::auth_service_i::AuthService,
    utils::constants::{NATIVE_AUTH_TYPE},
};
use async_trait::async_trait;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2
};
use rand_core::OsRng;

pub struct AuthServiceImpl <'a, C: CredentialRepo, U: UserRepo>{
    pub credential_repo: C,
    pub user_repo: U,
    pub argon2: Argon2<'a>
}

#[async_trait]
impl <'a, C, U>AuthService for AuthServiceImpl<'a, C, U> where C: CredentialRepo + Sync + Send, U: UserRepo + Sync + Send{
    
    async fn register(self: &Self, req: &RegistrationReq, auth_type: &str) -> Result<(), ServiceError> {
        let salt = SaltString::generate(&mut OsRng);
        //let password_hash = "".to_string();
        let password_hash = 
            if auth_type == NATIVE_AUTH_TYPE {
                self.argon2.hash_password_simple(req.password.as_bytes(), salt.as_ref()).unwrap().to_string()
            } 
            else {
                "".to_string()
            };
        let credential = Credential{
            email: req.email.clone(),
            hashed_password: password_hash,
            auth_type: auth_type.to_string(),
        };
        let _id = self.credential_repo.create_credential(&credential).await?;
        return self.user_repo.create_user(&_id, &req.username).await;
    }

    async fn verify_password(self: &Self, req: &LoginReq) -> Result<String, ServiceError> {
        let credential = self.credential_repo.get_credential(&req.email).await?;
        let parsed_hash = match PasswordHash::new(&credential._source.hashed_password){
            Ok(h) => h,
            Err(error) => return Err(ServiceError::UnauthorizedError(error.to_string()))
        };
        match self.argon2.verify_password(req.password.as_bytes(), &parsed_hash).is_ok(){
            true => Ok(credential._id),
            false => Err(ServiceError::UnauthorizedError("Incorrect username or password".to_string()))
        }
    }
}
