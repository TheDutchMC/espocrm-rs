use reqwest::Method;
use crypto::mac::Mac;
use std::future::Future;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EspoApiClient {
    pub(crate) url:                    String,
    pub(crate) username:               Option<String>,
    pub(crate) password:               Option<String>,
    pub(crate) api_key:                Option<String>,
    pub(crate) secret_key:             Option<String>,
    pub(crate) url_path:               String,
}

impl EspoApiClient {

    /// Create an instance of EspoApiClient.
    pub fn new(url: &str) -> EspoApiClient {
        EspoApiClient {
            url: url.to_string(),
            username: None,
            password: None,
            api_key: None,
            secret_key: None,
            url_path: "/api/v1/".to_string()
        }
    }

    pub fn build(&self) -> Self {
        self.clone()
    }

    /// Set the URL where EspoCRM is located.
    pub fn set_url<'a>(&'a mut self, url: &'a str) -> &'a mut EspoApiClient {
        let url = if url.ends_with("/") {
            let mut url = url.to_string();
            url.pop();
            url
        } else {
            url.to_string()
        };

        self.url = url;
        self
    }

    /// Set the username to use for authentication.
    /// If you use this you must also call [`Self::set_password()`]
    /// It is not recommended that you use this. Instead you should use API Key authorization or HMAC
    pub fn set_username<'a>(&'a mut self, username: &'a str) -> &'a mut EspoApiClient {
        self.username = Some(username.to_string());
        self
    }

    /// Set the password to use for authentication
    /// If you use this you must also call [`Self::set_username()`]
    /// It is not recommended that you use this. Instead you should use API Key authorization or HMAC authorization
    pub fn set_password<'a>(&'a mut self, password: &'a str) -> &'a mut EspoApiClient {
        self.password = Some(password.to_string());
        self
    }

    /// Set the API Key to use for authorization
    /// If you only provide the API key, and not the secret_key, API Key authorization will be used.
    /// If you wish to use HMAC authorization, you must also call [`Self::set_secret_key()`]
    pub fn set_api_key<'a>(&'a mut self, api_key: &'a str) -> &'a mut EspoApiClient {
        self.api_key = Some(api_key.to_string());
        self
    }

    /// Set the Secret Key to use for HMAC authorization
    /// If you use this you must also call [`Self::set_api_key()`]
    pub fn set_secret_key<'a>(&'a mut self, secret_key: &'a str) -> &'a mut EspoApiClient {
        self.secret_key = Some(secret_key.to_string());
        self
    }

    pub(crate) fn normalize_url(&self, action: String) -> String {
        format!("{}{}{}", self.url, self.url_path, action)
    }

    /// Make a request to EspoCRM
    /// For more information, see the [EspoCRM API Documentation](https://docs.espocrm.com/development/)
    ///
    /// * method: The HTTP method to be used. E.g GET or POST
    /// * action: On what EspoCRM Object should the action be performed on. E.g "Contact" or "Contact/ID". Essentially this is everything after "/api/v1/" in the URL.
    /// * data: The data to send. If the request method is GET, this will automatically be serialized to a query parameter String. If the request method is POST, it will be serialized to JSON and send as the request body.
    pub fn request<T: serde::Serialize + Clone>(&self, method: reqwest::Method, action: String, data: Option<T>) -> impl Future<Output = reqwest::Result<reqwest::Response>> {
        let mut url = self.normalize_url(action.clone());

        url = if data.is_some() && method == Method::GET {
            let data = data.clone().unwrap();

            format!("{}?{}", url, String::from_utf8(serde_php::to_vec(&data).unwrap()).unwrap())
        } else {
            url
        };

        println!("{}", &url);

        let client = reqwest::Client::new();
        let mut request_builder = client.request(method.clone(), url);

        //Basic authentication
        if self.username.is_some() && self.password.is_some() {
            request_builder = request_builder.basic_auth(self.username.clone().unwrap(), self.password.clone());

        //HMAC authentication
        } else if self.api_key.is_some() && self.secret_key.is_some() {
            let str = format!("{} /{}", method.clone().to_string(), action.clone());

            let mut hmac = crypto::hmac::Hmac::new(crypto::sha2::Sha256::new(), self.secret_key.clone().unwrap().as_bytes());
            hmac.input(str.as_bytes());
            let result = hmac.result();

            let auth_part = format!("{}{}{}", base64::encode(self.api_key.clone().unwrap()), base64::encode(":".as_bytes()), base64::encode(result.code()));
            request_builder = request_builder.header("X-Hmac-Authorization", auth_part);

        //Basic api key authentication
        } else if self.api_key.is_some() {
            request_builder = request_builder.header("X-Api-Key", self.api_key.clone().unwrap());
        }

        if data.is_some() {
            if method != Method::GET {
                request_builder = request_builder.json(&data.clone().unwrap());
                request_builder = request_builder.header("Content-Type", "application/json");
            }
        }

        let response = request_builder.send();
        response
    }
}