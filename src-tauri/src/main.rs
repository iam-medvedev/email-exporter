#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use chrono::*;
use rfc2047_decoder;
use std::fs;
use std::str;
use utime::*;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![run_email_export])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn run_email_export(host: &str, port: u16, login: &str, password: &str, is_secured: bool) {
  println!(
    "Debug: {host} {port} {login} {password} {is_secured}",
    host = host,
    port = port,
    login = login,
    password = password,
    is_secured = is_secured
  );

  let out_path = "/Users/imedvedev/Desktop/mail-export";

  let tls = native_tls::TlsConnector::builder().build().unwrap();
  let client = imap::connect((host, port), host, &tls).unwrap();
  let mut imap_session = client.login(login, password).unwrap();

  // Getting mailbox list
  let mailboxes = imap_session.lsub(None, Some("*")).unwrap();
  for mailbox in mailboxes.iter() {
    let mailbox_name = mailbox.name();
    imap_session.select(mailbox_name).unwrap();

    let decoded_mailbox_name = mailbox_name.as_bytes();

    println!("{:?} {:?}", mailbox, decoded_mailbox_name);

    // Create mailbox directory
    let mailbox_dir = format!("{}/{}", out_path, mailbox_name);
    fs::create_dir_all(mailbox_dir);

    // Getting all messages from current mailbox
    let messages = imap_session.fetch("1:3", "(RFC822 ENVELOPE)").unwrap();
    for message in messages.iter() {
      let envelope = message.envelope().unwrap();
      let date = get_date(std::str::from_utf8(envelope.date.unwrap()).unwrap());
      let subject = rfc2047_decoder::decode(envelope.subject.unwrap()).expect("No subject");

      if let Some(body) = message.body() {
        // Saving email as /output_path/mailbox/subject.eml
        let filepath = format!("{}/{}/{}.eml", out_path, mailbox_name, subject);
        fs::write(filepath.clone(), body).expect("Unable to write file");
        set_file_times(filepath, date.timestamp(), date.timestamp()).unwrap();
      } else {
        println!("Message didn't have a body!");
      }
    }
  }
}

/** Parse date from RFC822 or RFC822 + (Zone) */
fn get_date(s: &str) -> DateTime<FixedOffset> {
  return match DateTime::parse_from_rfc2822(s) {
    Ok(res) => res,
    Err(_err) => {
      return match DateTime::parse_from_str(s, "%a, %e %b %Y %T %z (%Z)") {
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
