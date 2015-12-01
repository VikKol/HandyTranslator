extern crate hyper;
use self::hyper::{Client};
use self::hyper::header::{Headers,Connection};
use self::hyper::status::StatusCode;

use std::io::prelude::*;
use std::error::Error;
use bing_auth_client::BingAuthClient;

pub struct Translator {
	url: &'static str,
	token_provider: BingAuthClient
}

impl Translator {
	pub fn new(sts_url: &'static str, client_id: &'static str, client_secret: &'static str, translator_url: &'static str) -> Self {
		Translator {
			url: translator_url,
			token_provider: BingAuthClient::new(sts_url, client_id, client_secret)
		}
	}
	
	pub fn translate(&self, text: String, from: &'static str, to: &'static str) -> String {
		let auth_token = format!("Bearer {0}", self.get_token());
		auth_token
		/*
		let requiest_url = format!("{0}?text={1}&from={2}&to={3}", self.url, text, from, to);
				
		let client = Client::new();
		
		let mut headers = Headers::new();
		headers.set(Connection::close());
		headers.set_raw("Authorization", vec![auth_token.into_bytes()]);
		
		let mut response = client
			.get(&*requiest_url)			
			.headers(headers)
			.send()
			.unwrap();
				
		let mut buf = String::new();
		match response.read_to_string(&mut buf) {
			Err(why) => Error::description(&why).to_string(),
			Ok(_) => if response.status == StatusCode::Ok {
				buf
			} else {
				format!("StatusCode: {0}; Response: {1}", response.status, buf)
			}
		}
		*/
	}
	
	fn get_token(&self) -> String {
		self.token_provider.get_access_token()
	}
}