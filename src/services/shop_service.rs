use crate::{
    auth::extractor::AuthRole,
    errors::shop_error::ShopDataError,
    models::{Product, ResponseShopData, ShopData},
    repositories::shop_repository::ShopRepository,
};

pub struct ShopDataService {
    repository: ShopRepository,
}

impl ShopDataService {
    pub fn new(repository: ShopRepository) -> Self {
        Self { repository }
    }

    pub async fn create_shop_service(
        &self,
        shop_data: ShopData,
        emp_id: i32,
        emp_role: AuthRole,
    ) -> Result<(), ShopDataError> {
        match emp_role {
            AuthRole::Admin => {
                self.repository.create_shop(shop_data).await?;
                Ok(())
            }
            AuthRole::Owner => {
                self.repository
                    .create_shop_by_owner(shop_data, emp_id)
                    .await?;
                Ok(())
            }
            AuthRole::Employee => {
                return Err(ShopDataError::Validation(
                    "You don't have permission. [a_r : 26]".to_string(),
                ));
            }
        }
    }

    pub async fn edit_shop_service(
        &self,
        shop_data: ResponseShopData,
        emp_role: AuthRole,
    ) -> Result<(), ShopDataError> {
        match emp_role {
            AuthRole::Admin => {
                self.repository.edit_shop_by_owner(shop_data).await?;
                Ok(())
            }
            AuthRole::Owner => {
                self.repository.edit_shop_by_owner(shop_data).await?;
                Ok(())
            }
            AuthRole::Employee => {
                return Err(ShopDataError::Validation(
                    "You don't have permission. [a_r : 26]".to_string(),
                ));
            }
        }
    }

    pub async fn get_shop_byowner(
        &self,
        emp_id: i32,
        emp_role: AuthRole,
    ) -> Result<Vec<ResponseShopData>, ShopDataError> {
        match emp_role {
            AuthRole::Admin => {
                let shop_list = self.repository.get_shop_byowner(emp_id, emp_role).await?;
                Ok(shop_list)
            }
            AuthRole::Owner => {
                let shop_list = self.repository.get_shop_byowner(emp_id, emp_role).await?;
                Ok(shop_list)
            }
            AuthRole::Employee => {
                let shop_list = self.repository.get_shop_byowner(emp_id, emp_role).await?;
                Ok(shop_list)
            }
        }
    }

    pub async fn get_shop_products_service(
        &self,
        emp_role: AuthRole,
        shop_id: i32,
    ) -> Result<Vec<Product>, ShopDataError> {
        match emp_role {
            AuthRole::Admin => {
                let product_list = self.repository.get_shop_products(shop_id).await?;
                Ok(product_list)
            }
            AuthRole::Owner => {
                let product_list = self.repository.get_shop_products(shop_id).await?;
                Ok(product_list)
            }
            AuthRole::Employee => todo!(),
        }
    }
}
