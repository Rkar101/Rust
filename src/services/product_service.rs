use sqlx::PgPool;

use crate::{
    auth::extractor::AuthRole,
    models::{
        BulkProductRequest, CreateProduct, Pagination, PatchProduct, Product, ProductListResponse,
    },
    repositories::product_repository::ProductRepository,
};

use crate::errors::product_error::ProductError;

pub struct ProductService {
    repository: ProductRepository,
    service_pool: PgPool,
}

impl ProductService {
    pub fn new(repository: ProductRepository, service_pool: PgPool) -> Self {
        Self {
            repository,
            service_pool,
        }
    }

    pub async fn get_products(
        &self,
        page: i64,
        limit: i64,
        name: Option<String>,
        sort: Option<String>,
        order: Option<String>,
        min_price: Option<i64>,
        max_price: Option<i64>,
        user_role: AuthRole,
        user_id: i32,
    ) -> Result<ProductListResponse, ProductError> {
        if page <= 0 {
            return Err(ProductError::Validation(
                "Page must be greater than 0".to_string(),
            ));
        }

        if limit <= 0 || limit > 35 {
            return Err(ProductError::Validation(
                "Limit must be between 1 and 35".to_string(),
            ));
        }

        if let Some(sort) = &sort {
            match sort.as_str() {
                "id" | "name" | "price" | "stock" => {}
                _ => {
                    return Err(ProductError::Validation("Invalid sort field".to_string()));
                }
            }
        }

        if let Some(order) = &order {
            match order.as_str() {
                "asc" | "desc" => {}
                _ => {
                    return Err(ProductError::Validation("Invalid sort field".to_string()));
                }
            }
        }
        if let Some(min_price) = min_price {
            if min_price < 0 {
                return Err(ProductError::Validation(
                    "Minimum price cannot be negative".to_string(),
                ));
            }
        }

        if let Some(max_price) = max_price {
            if max_price < 0 {
                return Err(ProductError::Validation(
                    "Maximum price cannot be negative".to_string(),
                ));
            }
        }

        if let (Some(min_price), Some(max_price)) = (min_price, max_price) {
            if max_price < min_price {
                return Err(ProductError::Validation(
                    "Maximum price cannot be lower than minimum price".to_string(),
                ));
            }
        }

        if user_role == AuthRole::Employee  {
            let mut tx = self.service_pool.begin().await?;

            let total_product = self
                .repository
                .count_products_by_owner(&mut tx, user_id)
                .await
                .map_err(ProductError::Database)?;

            let total_pages = (total_product + limit - 1) / limit;

            let products = self
                .repository
                .get_products_by_owner(page, limit, &mut tx, user_id)
                .await
                .map_err(ProductError::Database)?;

            tx.commit().await?;

            Ok(ProductListResponse {
                data: products,
                pagination: Pagination {
                    limit,
                    page,
                    total_items: total_product,
                    total_pages,
                },
            })
        } else {
            let mut tx = self.service_pool.begin().await?;

            let total_product = self
                .repository
                .count_products(name.clone(), min_price, max_price, &mut tx)
                .await
                .map_err(ProductError::Database)?;

            let total_pages = (total_product + limit - 1) / limit;

            let products = self
                .repository
                .get_products(
                    page, limit, name, sort, order, min_price, max_price, &mut tx,
                )
                .await
                .map_err(ProductError::Database)?;

            tx.commit().await?;
            Ok(ProductListResponse {
                data: products,
                pagination: Pagination {
                    limit,
                    page,
                    total_items: total_product,
                    total_pages,
                },
            })
        }
    }

    pub async fn create_bulk_product_list(
        &self,
        bulk_product: BulkProductRequest,
    ) -> Result<(), ProductError> {
        self.repository
            .create_bulk_product(bulk_product.products)
            .await?;
        Ok(())
    }

    pub async fn get_product_by_id(&self, id: i32) -> Result<Product, ProductError> {
        if id <= 0 {
            return Err(ProductError::Validation("Invalid Id".to_string()));
        }

        let product = self.repository.get_product_by_id(id).await?;

        match product {
            Some(product) => Ok(product),

            None => Err(ProductError::NotFound),
        }
    }

    pub async fn patch_product_by_id(
        &self,
        id: i32,
        product: &PatchProduct,
    ) -> Result<(), ProductError> {
        if id <= 0 {
            return Err(ProductError::Validation("Invalid Id".to_string()));
        }
        self.repository.patch_product_by_id(id, product).await?;
        Ok(())
    }

    pub async fn delete_product_by_id(&self, id: i32) -> Result<(), ProductError> {
        if id <= 0 {
            return Err(ProductError::Validation("Invalid Id".to_string()));
        }
        self.repository.delete_product_by_id(id).await?;
        Ok(())
    }

    pub async fn create_product(
        &self,
        product: CreateProduct,
        auth_role: AuthRole,
    ) -> Result<(), ProductError> {
        match auth_role {
            AuthRole::Admin => {
                if product.name.trim().is_empty() {
                    println!("Validation Error");
                    return Err(ProductError::Validation(
                        "Invalid Product Name!".to_string(),
                    ));
                }
                // if product.price < 0 {
                //     println!("Validation Error Price");
                //     return Err(ProductError::Validation(
                //         "Price cannot be negative!".to_string(),
                //     ));
                // }
                // if product.stock < 0 {
                //     println!("Validation Error stock");
                //     return Err(ProductError::Validation(
                //         "Stock cannot be negative!".to_string(),
                //     ));
                // }

                self.repository.create_product(&product).await?;
                Ok(())
            }
            AuthRole::Owner => {
                if product.name.trim().is_empty() {
                    println!("Validation Error");
                    return Err(ProductError::Validation(
                        "Invalid Product Name!".to_string(),
                    ));
                }

                self.repository.create_product(&product).await?;
                Ok(())
            }
            AuthRole::Employee => todo!(),
        }
    }

    pub async fn create_product_list(
        &self,
        product_list: Vec<CreateProduct>,
    ) -> Result<(), ProductError> {
        for product in product_list {
            if product.name.trim().is_empty() {
                println!("Validation Error");
                return Err(ProductError::Validation(
                    "Invalid Product Name!".to_string(),
                ));
            }
            // if product.price < 0 {
            //     println!("Validation Error Price");
            //     return Err(ProductError::Validation(
            //         "Price cannot be negative!".to_string(),
            //     ));
            // }
            // if product.stock < 0 {
            //     println!("Validation Error stock");
            //     return Err(ProductError::Validation(
            //         "Stock cannot be negative!".to_string(),
            //     ));
            // }

            self.repository.create_product(&product).await?;
        }
        Ok(())
    }
}
