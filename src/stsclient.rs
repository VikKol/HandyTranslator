extern crate url;
extern crate serde_json;
extern crate hyper;
use self::url::percent_encoding::{FORM_URLENCODED_ENCODE_SET,percent_encode};
use self::hyper::{Client};
use self::hyper::status::StatusCode;

use std::io::prelude::*;
use std::collections::BTreeMap;

pub struct StsClient {
	base_url: &'static str,
	request_details: String,
	http_client: Client
}

pub struct StsToken {
	pub access_token: String,
	pub token_type: String,
	pub expires_in: String,
	pub scope: String
}

pub type StsResponse = Result<StsToken, String>;

impl StsClient {
	pub fn new(base_url: &'static str, client_id: &'static str, client_secret: &'static str, scope: &'static str) -> Self {
		StsClient {
			base_url: base_url,
			request_details: format!("grant_type=client_credentials&client_id={0}&client_secret={1}&scope={2}",
						 			 percent_encode(client_id.to_string().as_bytes(), FORM_URLENCODED_ENCODE_SET), 
									 percent_encode(client_secret.to_string().as_bytes(), FORM_URLENCODED_ENCODE_SET),
									 scope),
			http_client: Client::new()
		}
	}
	
	pub fn get_access_token(&self) -> StsResponse {		
		let mut response = self.http_client
			.post(self.base_url)
			.body(&self.request_details)	
			.send()
			.unwrap();
		
		if response.status == StatusCode::Ok {
			let mut content = String::new();
			response.read_to_string(&mut content);
			let token = self.deserialize_content(&content); 
			Ok(token)			
		} else {
			Err(format!("{:?}", response.status_raw()))
		}
	}
	
	fn deserialize_content(&self, content: &String) -> StsToken {
		let deserialized_map: BTreeMap<String, String> = serde_json::from_str(&content).unwrap();		
		StsToken {
			scope: deserialized_map[&"scope".to_owned()].clone(),	
			token_type: deserialized_map[&"token_type".to_owned()].clone(),
			expires_in: deserialized_map[&"expires_in".to_owned()].clone(),
			access_token: deserialized_map[&"access_token".to_owned()].clone()
		}		
	}
}