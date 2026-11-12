use crate::{
    auth::extractor::AuthRole, errors::auth_error::UserError, models::{AdminDashboardStats, Category}, repositories::admin_repository::AdminRepository,
};

pub struct AdminService {
    repository: AdminRepository,
}

impl AdminService {
    pub fn new(repository: AdminRepository) -> Self {
        Self { repository }
    }

    pub async fn get_admin_dashboard(&self , emp_role : AuthRole , owner_id :i32 ) -> Result<AdminDashboardStats, UserError> {

        match emp_role {
            AuthRole::Admin => {
                 let data = self.repository.get_admin_dashboard_stats().await?;
        Ok(data)
            },
            AuthRole::Owner =>{ let data = self.repository.get_owner_dashboard_stats(owner_id).await?;
        Ok(data)},
            AuthRole::Employee => todo!(),
        }

       
    }

    pub async fn get_admin_categories(&self) -> Result<Vec<Category>, UserError> {
        let category = self.repository.get_categories().await?;
        Ok(category)
    }
}
