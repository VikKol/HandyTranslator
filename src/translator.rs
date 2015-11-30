extern crate hyper;

#[cfg(feature = "serde-serialization")]
extern crate serde;

use std::env;
use std::io;

use self::hyper::Url;
use self::hyper::Client;
use self::hyper::header::Connection;

static FROM: &'static str = "en";
static TO: &'static str = "ua";

header! { 
	(Authorization, "Authorization") => [String] 
}

struct Translator {
	url: &'static str
}

impl Translator {
	pub fn new(url: &'static str) -> Self {
		Translator { url: url }
	}
	
	pub fn translate(&self, text: String) -> String {
		let requiestUrl = self.url.to_string() + text + "&from=" + FROM + "&to=" + TO; 		
		let authToken = "Bearer".to_string() + " " + "token";//admToken.access_token;

		let client = Client::new();
	
		let mut response = client
			.get(&*self.url)			
			.header(Connection::close())
			.header(Authorization(authToken))
			.send()
			.unwrap();
		
		if response.status == 200 {
			res
		} else {
			"".to_owned()
		}
	}
	
	fn get_token(&self) -> &'static str {
		""
	}
}