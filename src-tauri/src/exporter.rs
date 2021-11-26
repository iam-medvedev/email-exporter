use chrono::{DateTime, FixedOffset, Local};
use std::fs;
use tauri::Manager;

/// Export emails
#[tauri::command(async)]
pub fn run_email_export(
  app_handle: tauri::AppHandle,
  host: &str,
  port: u16,
  login: &str,
  password: &str,
  out_path: &str,
) {
  let tls = native_tls::TlsConnector::builder().build().unwrap();
  let client = imap::connect((host, port), host, &tls).unwrap();
  let mut imap_session = client.login(login, password).unwrap();

  // Getting mailbox list
  let mailboxes = imap_session.lsub(None, Some("*")).unwrap();
  let mailboxes_count = mailboxes.len();

  app_handle.emit_all("progress", 0).unwrap();

  let mut i: f32 = 1.0;
  for mailbox in mailboxes.iter() {
    app_handle
      .emit_all("progress", i / mailboxes_count as f32)
      .unwrap();

    let mailbox_name = mailbox.name();
    imap_session.select(mailbox_name).unwrap();

    // Decoding mailbox name from UTF7 IMAP
    let decoded_mailbox_name = utf7_imap::decode_utf7_imap(String::from(mailbox_name));

    // Create mailbox directory
    let mailbox_dir = format!("{}/{}/", out_path, decoded_mailbox_name);
    fs::create_dir_all(mailbox_dir).unwrap();

    // Getting all messages from current mailbox
    let messages = imap_session.fetch("1:*", "(RFC822 ENVELOPE)").unwrap();
    for message in messages.iter() {
      let envelope = message.envelope().unwrap();
      let subject = rfc2047_decoder::decode(envelope.subject.unwrap()).expect("No subject");
      let date = get_date(String::from(
        std::str::from_utf8(envelope.date.unwrap()).unwrap(),
      ));

      if let Some(body) = message.body() {
        // Saving email as /output_path/mailbox/subject.eml
        let filepath = format!("{}/{}/{}.eml", out_path, decoded_mailbox_name, subject);
        fs::write(filepath.clone(), body).expect("Unable to write file");
        utime::set_file_times(filepath, date.timestamp(), date.timestamp()).unwrap();
      } else {
        println!("Message didn't have a body!");
      }
    }

    i += 1 as f32;
  }

  app_handle.emit_all("success", true).unwrap();
}

/// Parse date from RFC822 or RFC822 + (Zone)
fn get_date(s: String) -> DateTime<FixedOffset> {
  return match DateTime::parse_from_rfc2822(&s) {
    Ok(res) => res,
    Err(_err) => {
      return match DateTime::parse_from_str(&s, "%a, %e %b %Y %T %z (%Z)") {
        Ok(res) => res,
        Err(_err) => {
          let ndt = Local::now();
          let dt: DateTime<FixedOffset> = ndt.to_string().parse().unwrap();
          return dt;
        }
      }
    }
  };
}
