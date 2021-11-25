#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::fs;
use std::str;

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

    // Create mailbox directory
    let mailbox_dir = format!("{}/{}", out_path, mailbox_name);
    fs::create_dir_all(mailbox_dir);

    // Getting all messages from current mailbox
    let messages = imap_session.fetch("1:3", "(RFC822 ENVELOPE)").unwrap();
    for message in messages.iter() {
      let envelope = message.envelope().unwrap();

      let mut subject_string = String::new();
      if let Some(subject) = envelope.subject {
        subject_string.push_str(std::str::from_utf8(subject).unwrap())
      } else {
        println!("Message didn't have a subject!");
      }

      if let Some(body) = message.body() {
        let filepath = format!("{}/{}/{}.eml", out_path, mailbox_name, subject_string);
        println!("{}", filepath);
        fs::write(filepath, body).expect("Unable to write file");
      } else {
        println!("Message didn't have a body!");
      }
    }
  }
}
