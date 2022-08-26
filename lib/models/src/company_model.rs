use serde::Deserialize;

#[derive(Debug, Default, Deserialize, Clone)]
pub struct Company {
    pub company_key: String,
    pub company_name: String,
    pub company_name_2: Option<String>,
    pub street: String,
    pub postal_code: String,
    pub city: String,
    pub country: String,
    pub language: String,
    pub currency: String,
    pub view_maintenance: Option<String>,
    pub request: Option<String>,
    pub short_desc: Option<String>
}