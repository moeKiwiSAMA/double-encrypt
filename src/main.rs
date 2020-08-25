use openssl::symm::{encrypt, decrypt, Cipher};
use std::cell::RefCell;
use crate::util::KtStd;
use std::io::{stdin, Write};
use std::str::from_utf8;
use web_view::*;
mod util;
use regex::Regex;
use std::fs::File;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CIPHER: Cipher = Cipher::aes_256_cbc();
}

const SALT: &[u8; 32] = b"=);|:]>$&+=(@`-4^*K6'.,#=f~v*<}9";
const IV: &[u8; 16] = b"~Z@`j^]8&,'%*!2/";

fn main() {
    let indexPath = format!("file:///{}/resources/index.html", getCurrentDir());
    println!("{:?}", getCurrentDir());
    web_view::builder()
        .title("Encrypter")
        .content(Content::Url(indexPath))
        .size(900, 520)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg| {
            if Regex::new(r"^test:").unwrap().is_match(arg) {
                enc(arg.split(":").collect::<Vec<&str>>()[1]);
            }
            // match arg {
            //     "test" => enc(),
            //     _ => (),
            // }
            Ok(())
        })
        .run()
        .unwrap();
}


fn test(s: &str) {
    println!("{:?}", s)
}
fn make_key(input: &mut String) -> &[u8] {
    if input.len() >= 32 {
        input[0..32].as_bytes()
    } else {
        input.push_str(from_utf8(&SALT[0..32 - input.len()]).unwrap());
        input.as_bytes()
    }
}

fn getCurrentDir() -> std::string::String {
    String::from(std::env::current_dir().unwrap().to_str().unwrap())
}

fn enc(input: &str) -> std::io::Result<()> {
    //println!("{:?}", encrypt(*CIPHER, SALT, Some(IV), input.as_bytes()).unwrap())
    File::create("encrypted")?
        .write_all(&encrypt(*CIPHER, SALT, Some(IV), input.as_bytes()).unwrap())?;
    Ok(())
}