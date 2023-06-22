use handlebars::{Handlebars, Template};
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};
use serde_json::json;
use std::process;

use crate::{config::EmailConfig, errors::Error};

#[derive(Clone)]
pub struct Email {
    from: String,
    reply_to: String,
    transport: AsyncSmtpTransport<Tokio1Executor>,
}

impl Email {
    pub fn new(email_config: EmailConfig) -> Self {
        let credentials = Credentials::new(email_config.username, email_config.password);
        let transport = match AsyncSmtpTransport::<Tokio1Executor>::relay(&email_config.host) {
            Ok(builder) => builder
                .credentials(credentials)
                .port(email_config.port)
                .build(),
            Err(e) => {
                log::error!("Failed to create transport: {:?}", e);
                process::exit(1);
            }
        };

        Self {
            from: email_config.from,
            reply_to: email_config.reply_to,
            transport,
        }
    }

    //"/Users/headiron/codes/headiron-rust/target/release"
    //"/Users/headiron/codes/headiron-rust/target/release/headiron-rust"

    //"/Users/headiron/codes/headiron-rust"
    //"/Users/headiron/codes/headiron-rust/target/debug/headiron-rust"

    pub async fn send_registration_code(
        &self,
        to: String,
        subject: &'static str,
        header: &'static str,
        code: String,
        code_expire: i64,
    ) -> Result<(), Error> {
        let email = self.generate_email(to, subject, header, code, code_expire)?;

        self.transport.send(email).await?;

        Ok(())
    }

    fn generate_email(
        &self,
        to: String,
        subject: &'static str,
        header: &'static str,
        code: String,
        code_expire: i64,
    ) -> Result<Message, Error> {
        let mut handlebars = Handlebars::new();

        let template = match Template::compile(TEMPLATE) {
            Ok(template) => template,
            Err(e) => {
                return Err(Error::HandlebarsTemplateError(Box::new(e)));
            }
        };

        handlebars.register_template("template", template);

        let data = &json!({
            "title": subject,
            "header": header,
            "code": code,
            "code_expire": code_expire,
        });

        let html = handlebars.render("template", data)?;

        let message = Message::builder()
            .from(self.from.parse().unwrap())
            .reply_to(self.reply_to.parse().unwrap())
            .to(to.parse().unwrap())
            .header(ContentType::TEXT_HTML)
            .subject(subject)
            .date_now()
            .body(html)?;

        Ok(message)
    }
}

static TEMPLATE: &str = include_str!("../assets/email.html");
