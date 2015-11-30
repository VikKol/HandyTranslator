extern crate hyper;
use hyper::Client;
use hyper::header::{Headers,Connection};

static FROM = "en";
static TO = "ua";

header! { (Authorization, "Authorization") => [String] }

struct Translator {
	url: &'static str
}

impl Translator {
	pub fn new(url: &'static str) -> self {
		Translator { url: url }
	}
	
	pub fn translate(&self, String text) -> String {
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
}