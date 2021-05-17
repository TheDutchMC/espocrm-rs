use sha2::Sha256;
use hmac::{Hmac, NewMac, Mac};
use serde::Serialize;
use crate::espocrm_types::Params;

type HmacSha256 = Hmac<Sha256>;

/// Used to indicate the required GenericType is not needed
/// Used when calling [request()](EspoApiClient::request) with the GET method
pub type NoGeneric = ();

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete
}

impl From<Method> for reqwest::Method {
    fn from(a: Method) -> reqwest::Method {
        match a {
            Method::Get => reqwest::Method::GET,
            Method::Post => reqwest::Method::POST,
            Method::Put => reqwest::Method::PUT,
            Method::Delete => reqwest::Method::DELETE
        }
    }
}

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
    /// If you are making a GET request, you will still need to provide a type declaration for T. You can use the type NoGeneric for this.
    ///
    /// * method: The HTTP method to be used. E.g GET or POST
    /// * action: On what EspoCRM Object should the action be performed on. E.g "Contact" or "Contact/ID". Essentially this is everything after "/api/v1/" in the URL.
    /// * data_get: The filter to use on a GET request. Will be serialized according to PHP's http_build_query function.
    /// * data_post: The data to send on everything that is not a GET request. It will be serialized to JSON and send as the request body.
    pub async fn request<T, S>(&self, method: Method, action: S, data_get: Option<Params>, data_post: Option<T>) -> reqwest::Result<reqwest::Response>
        where
            T: Serialize + Clone,
            S: AsRef<str>
    {

        let action = action.as_ref().to_string();
        let mut url = self.normalize_url(action.clone());
        let reqwest_method = reqwest::Method::from(method);

        url = if data_get.is_some() && reqwest_method == reqwest::Method::GET {
            format!("{}?{}", url, crate::serializer::serialize(data_get.unwrap()).unwrap())
        } else {
            url
        };

        let client = reqwest::Client::new();
        let mut request_builder = client.request(reqwest_method.clone(), url);

        //Basic authentication
        if self.username.is_some() && self.password.is_some() {
            request_builder = request_builder.basic_auth(self.username.clone().unwrap(), self.password.clone());

        //HMAC authentication
        } else if self.api_key.is_some() && self.secret_key.is_some() {
            let str = format!("{} /{}", reqwest_method.clone().to_string(), action.clone());

            let mut mac = HmacSha256::new_from_slice(self.secret_key.clone().unwrap().as_bytes()).expect("Unable to create Hmac instance. Is your key valid?");
            mac.update(str.as_bytes());
            let mac_result = mac.finalize().into_bytes();

            let auth_part = format!("{}{}{}",
                                    base64::encode(self.api_key.clone().unwrap().as_bytes()),
                                    "6", //: in base64, for some reason this works, and turning ':' into base64 does not.
                                    base64::encode(mac_result));

            request_builder = request_builder.header("X-Hmac-Authorization", auth_part);


        //Basic api key authentication
        } else if self.api_key.is_some() {
            request_builder = request_builder.header("X-Api-Key", self.api_key.clone().unwrap());
        }

        if data_post.is_some() {
            if reqwest_method != reqwest::Method::GET {
                request_builder = request_builder.json(&data_post.clone().unwrap());
                request_builder = request_builder.header("Content-Type", "application/json");
            }
        }

        let response = request_builder.send();
        response.await
    }
}