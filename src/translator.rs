extern crate hyper;

use std::io::prelude::*;
use std::error::Error;

use self::hyper::{Client};
use self::hyper::header::{Connection,Headers};
use self::hyper::status::StatusCode;

static FROM: &'static str = "en";
static TO: &'static str = "uk";

pub struct Translator {
	url: &'static str
}

impl Translator {
	pub fn new(url: &'static str) -> Self {
		Translator { url: url }
	}
	
	pub fn translate(&self, text: String) -> String {
		let requiest_url = format!("{0}?text={1}&from={2}&to={3}", self.url, text, FROM, TO);
		let auth_token = "Bearer".to_owned(); //.to_string() + " " + "token";
				
		let client = Client::new();
		
		let mut headers = Headers::new();
		headers.set(Connection::close());
		headers.set_raw("Authorization", vec![auth_token.into_bytes()]);
		
		let mut response = client
			.get(&*requiest_url)			
			.headers(headers)
			.send().unwrap();
				
		let mut buf = String::new();
		match response.read_to_string(&mut buf) {
			Err(why) => Error::description(&why).to_string(),
			Ok(_) => if response.status == StatusCode::Ok {
				buf
			} else {
				format!("StatusCode: {0}; Response: {1}", response.status, buf)
			}
		}
	}
	
	fn get_token(&self) -> &'static str {
		"token"
	}
}