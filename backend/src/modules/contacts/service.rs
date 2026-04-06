use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use super::entity::Contact;

pub struct ContactService {
    pool: PgPool,
}

impl ContactService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Contact>> {
        let contact = sqlx::query_as::<_, Contact>("SELECT * FROM contacts WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(contact)
    }

    pub async fn upsert(
        &self,
        phone_number: &str,
        first_name: Option<&str>,
        last_name: Option<&str>,
    ) -> Result<Contact> {
        let contact = sqlx::query_as::<_, Contact>(
            r#"
            INSERT INTO contacts (id, phone_number, first_name, last_name)
            VALUES (gen_random_uuid(), $1, $2, $3)
            ON CONFLICT (phone_number) DO UPDATE
              SET first_name = COALESCE(EXCLUDED.first_name, contacts.first_name),
                  last_name  = COALESCE(EXCLUDED.last_name,  contacts.last_name)
            RETURNING *
            "#,
        )
        .bind(phone_number)
        .bind(first_name)
        .bind(last_name)
        .fetch_one(&self.pool)
        .await?;
        Ok(contact)
    }
}
