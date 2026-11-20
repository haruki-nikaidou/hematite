use sqlx::types::Json;
use time::PrimitiveDateTime;

#[derive(Debug, Clone)]
pub struct ModuleConfigEntity {
    pub key: String,
    pub value: Json<serde_json::Value>,
    pub last_updated_at: PrimitiveDateTime,
}

pub trait ModuleConfig: serde::Serialize + for<'de> serde::Deserialize<'de> + Default {
    const CONFIG_PREFIX: &'static str;
    const CONFIG_KEY: &'static str;
}
