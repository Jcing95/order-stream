use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tower_sessions::{
    session::{Id, Record},
    session_store, SessionStore,
};
use crate::backend::db::DB;
use crate::common::errors::Error;

pub fn hash_password(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| Error::InternalError(format!("Password hashing failed: {}", e)))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, Error> {
    let parsed_hash =
        PasswordHash::new(hash).map_err(|e| Error::InternalError(format!("Invalid password hash: {}", e)))?;

    let argon2 = Argon2::default();
    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SessionRecord {
    id: String,
    data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct SurrealSessionStore;

impl SurrealSessionStore {
    pub fn new() -> Self {
        Self
    }
}

const SESSIONS_TABLE: &str = "sessions";

#[async_trait]
impl SessionStore for SurrealSessionStore {
    async fn create(&self, record: &mut Record) -> session_store::Result<()> {
        let now = OffsetDateTime::now_utc();
        let session_record = SessionRecord {
            id: record.id.to_string(),
            data: record.data.clone(),
            expiry_date: record.expiry_date.map(|d| d.format(&time::format_description::well_known::Rfc3339).unwrap()),
            created_at: now.format(&time::format_description::well_known::Rfc3339).unwrap(),
        };

        DB.create((SESSIONS_TABLE, &record.id.to_string()))
            .content(session_record)
            .await
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;

        Ok(())
    }

    async fn save(&self, record: &Record) -> session_store::Result<()> {
        let session_record = SessionRecord {
            id: record.id.to_string(),
            data: record.data.clone(),
            expiry_date: record.expiry_date.map(|d| d.format(&time::format_description::well_known::Rfc3339).unwrap()),
            created_at: OffsetDateTime::now_utc().format(&time::format_description::well_known::Rfc3339).unwrap(),
        };

        DB.update((SESSIONS_TABLE, &record.id.to_string()))
            .content(session_record)
            .await
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;

        Ok(())
    }

    async fn load(&self, session_id: &Id) -> session_store::Result<Option<Record>> {
        let session_record: Option<SessionRecord> = DB
            .select((SESSIONS_TABLE, &session_id.to_string()))
            .await
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;

        match session_record {
            Some(record) => {
                let expiry_date = record
                    .expiry_date
                    .as_ref()
                    .and_then(|d| OffsetDateTime::parse(d, &time::format_description::well_known::Rfc3339).ok());

                let session_record = Record {
                    id: session_id.clone(),
                    data: record.data,
                    expiry_date,
                };

                Ok(Some(session_record))
            }
            None => Ok(None),
        }
    }

    async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
        let _deleted: Option<SessionRecord> = DB
            .delete((SESSIONS_TABLE, &session_id.to_string()))
            .await
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;

        Ok(())
    }
}
