use reqwest::Client;

#[derive(Debug)]
pub enum StorageError {
    Request(reqwest::Error),
    UploadFailed {
        status: u16,
        message: String,
    },
}

impl From<reqwest::Error> for StorageError {
    fn from(error: reqwest::Error) -> Self {
        Self::Request(error)
    }
}

#[derive(Clone)]
pub struct SupabaseStorage {
    client: Client,
    supabase_url: String,
    service_role_key: String,
    bucket: String,
}

impl SupabaseStorage {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            supabase_url: std::env::var("SUPABASE_URL")
                .expect("SUPABASE_URL is missing"),
            service_role_key: std::env::var("SUPABASE_SERVICE_ROLE_KEY")
                .expect("SUPABASE_SERVICE_ROLE_KEY is missing"),
            bucket: std::env::var("SUPABASE_STORAGE_BUCKET")
                .expect("SUPABASE_STORAGE_BUCKET is missing"),
        }
    }

    pub async fn upload_image(
        &self,
        inventory_id: i32,
        image_bytes: Vec<u8>,
        content_type: &str,
    ) -> Result<String, StorageError> {
        let path = format!("products/{inventory_id}/image.jpg");

        let url = format!(
            "{}/{}/{}",
            self.supabase_url,
            self.bucket,
            path
        );

        let response = self
            .client
            .post(&url)
            .header(
                "Authorization",
                format!("Bearer {}", self.service_role_key),
            )
            .header("apikey", &self.service_role_key)
            .header("Content-Type", content_type)
            .body(image_bytes)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let message = response.text().await.unwrap_or_default();

            return Err(StorageError::UploadFailed {
                status,
                message,
            });
        }

        Ok(path)
    }

    pub fn get_image_url(&self, image_path: &str) -> String {
        format!(
            "{}/{}/{}",
            self.supabase_url,
            self.bucket,
            image_path
        )
    }
}