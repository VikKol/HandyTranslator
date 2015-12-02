#[derive(Clone)]
pub struct AppSettings {
	pub sts_url: &'static str,
	pub scope: &'static str,
	pub client_id: &'static str,
	pub client_secret: &'static str, 
	pub translator_url: &'static str,
	pub source_lang: &'static str, 
	pub target_lang: &'static str
}