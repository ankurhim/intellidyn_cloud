use serde::Deserialize;

#[derive(Debug, Default, Deserialize, Clone)]
pub struct CompanyCode {
    pub company_code: String,
    pub company_name: String,
    pub city: String,
    pub country: String,
    pub language: String,
    pub currency: String,
    pub title: String,
    pub search_terms: Vec<String>,
    pub street_house_no: String,
    pub postal_code: String,
    pub region: String,
    pub telephone: Option<String>,
    pub tel_ext: Option<String>,
    pub fax: Option<String>,
    pub fax_ext: Option<String>,
    pub mobile_phone: Option<String>,
    pub email: String,
    pub comments: Option<String>,
    pub request: Option<String>,
    pub short_desc: Option<String>
}