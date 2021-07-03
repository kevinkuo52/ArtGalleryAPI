use crate::{
    models::user::*,
    models::auth::*,
    models::error::ServiceError,
    repositories::credential_repo_i::*,
    services::auth_service_i::*,
    utils::constants::{NATIVE_AUTH_TYPE},
};
use async_trait::async_trait;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2
};
use std::sync::Arc;
use rand_core::OsRng;

pub struct AuthServiceImpl <'a, C: CredentialRepo>{
    pub credential_repo: C,
    pub argon2: Argon2<'a>
}

#[async_trait]
impl <'a, C>AuthService for AuthServiceImpl<'a, C> where C: CredentialRepo + Sync + Send{
    
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
        return self.credential_repo.save_credential(&credential).await;
    }

    async fn verify_password(self: &Self, req: &LoginReq) -> Result<bool, ServiceError> {
        let credential = self.credential_repo.get_credential(&req.email).await?;
        let parsed_hash = match PasswordHash::new(&credential.hashed_password){
            Ok(h) => h,
            Err(error) => return Err(ServiceError::UnauthorizedError(error.to_string()))
        };
        return Ok(self.argon2.verify_password(req.password.as_bytes(), &parsed_hash).is_ok());
    }

    async fn authenticate(self: &Self, token: &Token) -> Option<String> {
        // self.token_repo.get_username_by_token(token).await
        return None;
    }

   

}
