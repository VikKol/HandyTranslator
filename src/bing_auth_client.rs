/*
const REFRESH_TOKEN_IN_MIN: i32 = 9;
static DATAMARKET_ACCESS_URI = "https://datamarket.accesscontrol.windows.net/v2/OAuth2-13";

pub struct BingAuthClient {
	_client_id: &'static str,
	_client_secret: &'static str,
	_request_data: &'static str,
	_token: String
}

impl BingAuthClient {
	pub fn new(clientId: &'static str, clientSecret: &'static str) -> self {
		this.clientId = clientId;
		this.clientSecret = clientSecret;
		
		this.request = string.Format("grant_type=client_credentials&client_id={0}&client_secret={1}&scope=http://api.microsofttranslator.com", HttpUtility.UrlEncode(clientId), HttpUtility.UrlEncode(clientSecret));
		
		//this.token = HttpPost(DatamarketAccessUri, this.request);
	}
	
	public AdmAccessToken GetAccessToken()
        {
			return this.token;
        }
		private void RenewAccessToken()
        {
            AdmAccessToken newAccessToken = HttpPost(DatamarketAccessUri, this.request);
			//swap the new token with old one
			//Note: the swap is thread unsafe
			this.token = newAccessToken;
		}
		
		
		private AdmAccessToken HttpPost(string DatamarketAccessUri, string requestDetails)
        {
			//Prepare OAuth request 
            WebRequest webRequest = WebRequest.Create(DatamarketAccessUri);
            webRequest.ContentType = "application/x-www-form-urlencoded";
            webRequest.Method = "POST";
			byte[] bytes = Encoding.ASCII.GetBytes(requestDetails);
            webRequest.ContentLength = bytes.Length;
			using (Stream outputStream = webRequest.GetRequestStream())
            {
                outputStream.Write(bytes, 0, bytes.Length);
            }
			using (WebResponse webResponse = webRequest.GetResponse())
            {
                DataContractJsonSerializer serializer = new DataContractJsonSerializer(typeof(AdmAccessToken));
				//Get deserialized object from JSON stream
                AdmAccessToken token = (AdmAccessToken)serializer.ReadObject(webResponse.GetResponseStream());
				return token;
            }
        }
}
*/