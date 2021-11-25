#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use base64;
use chrono::*;
use encoding_rs::*;
use regex::Regex;
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
/**
 * Export emails
 */
fn run_email_export(host: &str, port: u16, login: &str, password: &str, out_path: &str) {
  println!(
    "Debug: {host} {port} {login} {password} {out_path}",
    host = host,
    port = port,
    login = login,
    password = password,
    out_path = out_path
  );

  let tls = native_tls::TlsConnector::builder().build().unwrap();
  let client = imap::connect((host, port), host, &tls).unwrap();
  let mut imap_session = client.login(login, password).unwrap();

  // Getting mailbox list
  let mailboxes = imap_session.lsub(None, Some("*")).unwrap();
  for mailbox in mailboxes.iter() {
    let mailbox_name = mailbox.name();
    imap_session.select(mailbox_name).unwrap();

    let decoded_mailbox_name = decode_utf7_imap(String::from(mailbox_name));

    // Create mailbox directory
    let mailbox_dir = format!("{}/{}/", out_path, decoded_mailbox_name);
    fs::create_dir_all(mailbox_dir);

    // Getting all messages from current mailbox
    let messages = imap_session.fetch("1:3", "(RFC822 ENVELOPE)").unwrap();
    for message in messages.iter() {
      let envelope = message.envelope().unwrap();
      let date = get_date(std::str::from_utf8(envelope.date.unwrap()).unwrap());
      let subject = rfc2047_decoder::decode(envelope.subject.unwrap()).expect("No subject");

      if let Some(body) = message.body() {
        // Saving email as /output_path/mailbox/subject.eml
        let filepath = format!("{}/{}/{}.eml", out_path, decoded_mailbox_name, subject);

        println!("{}", filepath);
        fs::write(filepath.clone(), body).expect("Unable to write file");
        set_file_times(filepath, date.timestamp(), date.timestamp()).unwrap();
      } else {
        println!("Message didn't have a body!");
      }
    }
  }
}

/**
 * Decode UTF-7 IMAP mailbox name
 *
 * https://datatracker.ietf.org/doc/html/rfc3501#section-5.1.3
 */
fn decode_utf7_imap(text: String) -> String {
  let re = Regex::new(r"&[^&-]*-").unwrap();
  let mut result = text.clone();

  for cap in re.captures_iter(&text) {
    let encoded_text = cap.get(0).map_or("", |m| m.as_str());
    let decoded_text = decode_utf7_part(String::from(encoded_text));

    result = text.replace(&encoded_text, &decoded_text);
  }

  return result;
}

/**
 * Decode UTF-7 IMAP part of string
 */
fn decode_utf7_part(text: String) -> String {
  if text == "&-" {
    return String::from("&");
  }

  let text_mb64 = &text[1..text.len() - 1];
  let mut text_b64 = text_mb64.replace(",", "/");

  while (text_b64.len() % 4) != 0 {
    text_b64 += "=";
  }

  let text_u16 = base64::decode(text_b64).unwrap();
  let (cow, _encoding_used, _had_errors) = UTF_16BE.decode(&text_u16);
  let result = cow.as_ref();

  return String::from(result);
}

/**
 * Parse date from RFC822 or RFC822 + (Zone)
 */
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
