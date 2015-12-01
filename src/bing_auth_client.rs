extern crate hyper;

use self::hyper::{Client};
use self::hyper::client::Body;
use self::hyper::header::{Headers,Connection};
use self::hyper::status::StatusCode;

use std::io::prelude::*;
use std::error::Error;

pub struct BingAuthClient {
	base_url: &'static str,
	request_details: String
}

impl BingAuthClient {
	pub fn new(base_url: &'static str, client_id: &'static str, client_secret: &'static str) -> Self {
		BingAuthClient {
			base_url: base_url,
			request_details: format!("grant_type=client_credentials&client_id={0}&client_secret={1}&scope=http://api.microsofttranslator.com", 
						 			 client_id, 
									 client_secret)
		}
	}
	
	pub fn get_access_token(&self) -> String {				
		let bytes = self.request_details.to_string().into_bytes();
		let client = Client::new();
		let mut headers = Headers::new();
		headers.set(Connection::close());
		headers.set_raw("ContentType", vec!["application/x-www-form-urlencoded".to_string().into_bytes()]);
		headers.set_raw("ContentLength", vec![format!("{}", bytes.len()).into_bytes()]);
		
		let response = client
			.post(&*self.base_url)
			.headers(headers)
			.body(Body::BufBody(&bytes[..], bytes.len()))	
			.send();
		
		match response {
			Err(why) => Error::description(&why).to_string(),
			Ok(mut resp) => {
				let mut buf = String::new();
				match resp.read_to_string(&mut buf) {
					Err(nested_why) => Error::description(&nested_why).to_string(),
					Ok(_) => if resp.status == StatusCode::Ok {
						buf
					} else {
						format!("StatusCode: {0}; Response: {1}", resp.status, buf)
					}
				}
			}
		}
	}
}