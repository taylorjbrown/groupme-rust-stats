
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub type:String,
    pub project_id:String,
    pub private_key_id:String,
    pub private_key:String,
    pub client_email:String,
    pub client_id:String,
    pub auth_uri:String,
    pub token_uri:String,
    pub auth_provider_x509_cert_url:String,
    pub client_x509_cert_url:String
}