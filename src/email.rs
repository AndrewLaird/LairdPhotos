use std::env;
use chrono;
use lettre::{
    message::{header::ContentType, Attachment, Mailbox, Message, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    transport::smtp::SmtpTransport,
    Transport,
};

pub async fn send(files: Vec<(String, Vec<u8>)>, destination: String) -> String {
    let from_name = env::var("FROM_NAME").unwrap_or_default();
    let from_email = env::var("FROM_EMAIL").expect("FROM_EMAIL not set");
    let to_name = env::var("TO_NAME").unwrap_or_default();

    let from: Mailbox = format!("{from_name} <{from_email}>")
        .parse()
        .expect("Invalid 'from' email address");
    let to: Mailbox = format!("{to_name} <{destination}>")
        .parse()
        .expect("Invalid 'to' email address");

    // Build the mixed multipart with a text body part
    let mut multipart = MultiPart::mixed().singlepart(
        SinglePart::builder()
            .header(ContentType::TEXT_PLAIN)
            .body("I've attached photos for the digital album".to_string()),
    );

    // Attach each file
    for (filename, data) in files {
        let attachment_part = Attachment::new(filename)
            .body(data, ContentType::parse(&mime::APPLICATION_OCTET_STREAM.to_string()).unwrap());
        multipart = multipart.singlepart(attachment_part);
    }

    // Build the final email
    let email = match Message::builder()
        .from(from)
        .to(to)
        .subject("Digital Photo Upload From {{date}}".replace("{{date}}", &chrono::Local::now().to_string()))
        .multipart(multipart)
    {
        Ok(msg) => msg,
        Err(e) => return format!("Failed to build email message: {e}"),
    };

    // SMTP configuration (e.g., Gmail or your provider)
    let smtp_server = env::var("SMTP_SERVER").unwrap_or_else(|_| "smtp.gmail.com".to_string());
    let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME not set");
    let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD not set");

    let creds = Credentials::new(smtp_username, smtp_password);
    let mailer = match SmtpTransport::relay(&smtp_server) {
        Ok(builder) => builder.credentials(creds).build(),
        Err(e) => return format!("Failed to create SMTP transport: {e}"),
    };

    match mailer.send(&email) {
        Ok(_) => format!("Email sent successfully! from {from_email} to {destination}"),
        Err(e) => format!("Could not send email: {e}"),
    }
}
