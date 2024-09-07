use schemars::JsonSchema;
use serde::Deserialize;
use spring::config::Configurable;

/// SMTP mailer configuration structure.
#[derive(Debug, Configurable, Clone, JsonSchema, Deserialize)]
#[config_prefix = "mail"]
pub struct MailerConfig {
    /// SMTP host. for example: localhost, smtp.gmail.com etc.
    pub host: String,
    /// SMTP port/
    pub port: u16,
    /// Enable TLS
    #[serde(default = "bool::default")]
    pub secure: bool,
    /// Auth SMTP server
    pub auth: Option<MailerAuth>,
}

/// Authentication details for the mailer
#[derive(Debug, Clone, JsonSchema, Deserialize)]
pub struct MailerAuth {
    /// User
    pub user: String,
    /// Password
    pub password: String,
}
