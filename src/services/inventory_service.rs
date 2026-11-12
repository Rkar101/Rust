use crate::{
    auth::extractor::AuthRole,
    errors::product_error::ProductError,
    models::{InventoryData, InventoryItemView},
    repositories::inventory_repository::InventoryRepo,
    services::storage_service::SupabaseStorage,
};

pub struct InventoryService {
    repository: InventoryRepo,
    storage: SupabaseStorage,
}

impl InventoryService {
    pub fn new(repo: InventoryRepo, storage: SupabaseStorage) -> Self {
        Self {
            repository: repo,
            storage: storage,
        }
    }

    pub async fn get_inventory_data_service(
        &self,
        role: AuthRole,
        shop_id: Option<i32>,
    ) -> Result<Vec<InventoryItemView>, ProductError> {
        if let Some(shop_id) = shop_id {
            if shop_id <= 0 {
                return Err(ProductError::Validation("Invalid Client Request Data.".to_string()));
            }
        } else {
            // Handle the case where shop_id is None
            return Err(ProductError::NotFound);
        }
        match role {
            // AuthRole::Admin => {
            //     let inventory_data = self.repository.get_inventory_data(shop_id).await?;
            //     Ok(inventory_data)
            // }
            // AuthRole::Owner => {
            //     let inventory_data = self.repository.get_inventory_data(shop_id).await?;
            //     Ok(inventory_data)
            // }
            AuthRole::Admin | AuthRole::Owner => {
                let mut inventory_data = self.repository.get_inventory_data(shop_id).await?;

                for item in &mut inventory_data {
                    if let Some(path) = &item.image_path {
                        item.image_url = Some(self.storage.get_image_url(path));
                    }
                }

                Ok(inventory_data)
            }
            AuthRole::Employee => {
                let mut inventory_data = self.repository.get_inventory_data(shop_id).await?;

                for item in &mut inventory_data {
                    if let Some(path) = &item.image_path {
                        item.image_url = Some(self.storage.get_image_url(path));
                    }
                }

                Ok(inventory_data)
            }
        }
    }

    pub async fn add_inventory_product_service(
        &self,
        role: AuthRole,
        inv_pdata: InventoryData,
    ) -> Result<(), ProductError> {
        match role {
            AuthRole::Admin => {
                self.repository.add_inventory_product(inv_pdata).await?;
                Ok(())
            }
            AuthRole::Owner => {
                self.repository.add_inventory_product(inv_pdata).await?;
                Ok(())
            }
            AuthRole::Employee => todo!(),
        }
    }

    pub async fn edit_inventory_product_service(
        &self,
        role: AuthRole,
        inv_pdata: InventoryData,
    ) -> Result<(), ProductError> {
        match role {
            AuthRole::Admin => {
                self.repository.edit_inventory_product(inv_pdata).await?;
                Ok(())
            }
            AuthRole::Owner => {
                self.repository.edit_inventory_product(inv_pdata).await?;
                Ok(())
            }
            AuthRole::Employee => todo!(),
        }
    }
}
