use crate::{
    auth::auth_service::generate_token,
    errors::auth_error::UserError,
    models::{LoginRequest, RegisterRequest},
    repositories::user_repository::UserRepository,
    utils::password::{hash_password, verify_password},
};

pub struct UserRegisterService {
    repository: UserRepository,
}

impl UserRegisterService {
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }

    pub async fn get_user_data(&self, user_id: i32) -> Result<(String, String, String), UserError> {
        let data = self.repository.user_data(user_id).await?;

        let name = data.0;

        let email = data.1;
        let formatted_date = data.2.format("%d %b %Y").to_string();

        Ok((name, email, formatted_date))
    }

    pub async fn create_user(&self, user: RegisterRequest) -> Result<(), UserError> {
        if user.username.trim().is_empty() {
            return Err(UserError::Validation("Invalid User!".to_string()));
        }

        if user.email.trim().is_empty() {
            return Err(UserError::Validation("Invalid User!".to_string()));
        }

        if user.password.trim().is_empty() {
            return Err(UserError::Validation("Invalid User!".to_string()));
        }
        let password_hash = hash_password(&user.password)
            .map_err(|_| UserError::Validation("Password hashing failed".to_string()))?;

        self.repository
            .create_user_repo(&user.username, &user.email, &password_hash)
            .await?;

        Ok(())
    }

    pub async fn login_user(&self, login_req: LoginRequest) -> Result<(String, String), UserError> {
        if login_req.email.trim().is_empty() || login_req.password.trim().is_empty() {
            return Err(UserError::Validation(
                "Invalid email or password.".to_string(),
            ));
        } else {
            let hash_pw = self.repository.get_hash_pw(login_req.email.trim()).await?;

            match hash_pw {
                Some(hash_pw) => {
                    let verify =
                        verify_password(&login_req.password, &hash_pw.1).map_err(|_| {
                            UserError::Validation("Invalid email or password.".to_string())
                        })?;

                    if !verify {
                        return Err(UserError::Validation(
                            "Invalid email or password.".to_string(),
                        ));
                    }

                    let jwt_token = generate_token(hash_pw.0, hash_pw.2.clone()).map_err(|_| {
                        return UserError::Validation(
                            "Invalid email or password. Code[0001]".to_string(),
                        );
                    });

                    if let Ok(jwt_token) = jwt_token {
                        return Ok((jwt_token, hash_pw.2));
                    } else {
                        return Err(UserError::Validation(
                            "Invalid email or password. Code[0001]".to_string(),
                        ));
                    }
                }

                None => {
                    return Err(UserError::Validation(
                        "Invalid email or password.".to_string(),
                    ));
                }
            }
        }
    }
}
