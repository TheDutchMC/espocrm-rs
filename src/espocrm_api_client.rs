use crate::espocrm_types::Params;
use crate::{debug_if, trace_if};
use hmac::{Hmac, Mac};
use serde::Serialize;
use sha2::Sha256;
use std::fmt::Debug;
use tap::TapFallible;

type HmacSha256 = Hmac<Sha256>;

/// Used to indicate the required GenericType is not needed
/// Used when calling [request()](EspoApiClient::request) with the GET method
pub type NoGeneric = ();

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

impl From<Method> for reqwest::Method {
    fn from(a: Method) -> reqwest::Method {
        match a {
            Method::Get => reqwest::Method::GET,
            Method::Post => reqwest::Method::POST,
            Method::Put => reqwest::Method::PUT,
            Method::Delete => reqwest::Method::DELETE,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EspoApiClient {
    pub(crate) url: String,
    pub(crate) username: Option<String>,
    pub(crate) password: Option<String>,
    pub(crate) api_key: Option<String>,
    pub(crate) secret_key: Option<String>,
    pub(crate) url_path: String,
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
            url_path: "/api/v1/".to_string(),
        }
    }

    pub fn build(&self) -> Self {
        self.clone()
    }

    /// Set the URL where EspoCRM is located.
    pub fn set_url<S: AsRef<str>>(&mut self, url: S) -> &mut EspoApiClient {
        let url = url.as_ref();

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
    pub fn set_username<S: AsRef<str>>(&mut self, username: S) -> &mut EspoApiClient {
        self.username = Some(username.as_ref().to_string());
        self
    }

    /// Set the password to use for authentication
    /// If you use this you must also call [`Self::set_username()`]
    /// It is not recommended that you use this. Instead you should use API Key authorization or HMAC authorization
    pub fn set_password<S: AsRef<str>>(&mut self, password: S) -> &mut EspoApiClient {
        self.password = Some(password.as_ref().to_string());
        self
    }

    /// Set the API Key to use for authorization
    /// If you only provide the API key, and not the secret_key, API Key authorization will be used.
    /// If you wish to use HMAC authorization, you must also call [`Self::set_secret_key()`]
    pub fn set_api_key<S: AsRef<str>>(&mut self, api_key: S) -> &mut EspoApiClient {
        self.api_key = Some(api_key.as_ref().to_string());
        self
    }

    /// Set the Secret Key to use for HMAC authorization
    /// If you use this you must also call [`Self::set_api_key()`]
    pub fn set_secret_key<S: AsRef<str>>(&mut self, secret_key: S) -> &mut EspoApiClient {
        self.secret_key = Some(secret_key.as_ref().to_string());
        self
    }

    pub(crate) fn normalize_url<S: AsRef<str>>(&self, action: S) -> String {
        format!("{}{}{}", self.url, self.url_path, action.as_ref())
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
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(data_get, data_post)))]
    pub async fn request<T, S>(
        &self,
        method: Method,
        action: S,
        data_get: Option<Params>,
        data_post: Option<T>,
    ) -> reqwest::Result<reqwest::Response>
    where
        T: Serialize + Clone + Debug,
        S: AsRef<str> + Debug,
    {
        let mut url = self.normalize_url(&action.as_ref());
        debug_if!("Using URL {url} to request from EspoCRM");

        let reqwest_method = reqwest::Method::from(method);

        url = if data_get.is_some() && reqwest_method == reqwest::Method::GET {
            format!(
                "{}?{}",
                url,
                crate::serializer::serialize(data_get.unwrap()).unwrap()
            )
        } else {
            url
        };

        let client = reqwest::Client::new();
        let mut request_builder = client.request(reqwest_method.clone(), url);

        //Basic authentication
        if self.username.is_some() && self.password.is_some() {
            trace_if!("Using basic authentication");
            request_builder =
                request_builder.basic_auth(self.username.clone().unwrap(), self.password.clone());

        //HMAC authentication
        } else if self.api_key.is_some() && self.secret_key.is_some() {
            trace_if!("Using HMAC authentication.");

            let str = format!(
                "{} /{}",
                reqwest_method.clone().to_string(),
                action.as_ref()
            );

            let mut mac = HmacSha256::new_from_slice(self.secret_key.clone().unwrap().as_bytes())
                .expect("Unable to create Hmac instance. Is your key valid?");
            mac.update(str.as_bytes());
            let mac_result = mac.finalize().into_bytes();

            let auth_part = format!(
                "{}{}{}",
                base64::encode(self.api_key.clone().unwrap().as_bytes()),
                "6", //: in base64, for some reason this works, and turning ':' into base64 does not.
                base64::encode(mac_result)
            );

            request_builder = request_builder.header("X-Hmac-Authorization", auth_part);

        //Basic api key authentication
        } else if self.api_key.is_some() {
            trace_if!("Authenticating with an API key");

            request_builder = request_builder.header("X-Api-Key", self.api_key.clone().unwrap());
        }

        if data_post.is_some() {
            if reqwest_method != reqwest::Method::GET {
                request_builder = request_builder.json(&data_post.clone().unwrap());
                request_builder = request_builder.header("Content-Type", "application/json");
            }
        }

        trace_if!("Sending request to EspoCRM");
        #[allow(unused)]
        request_builder
            .send()
            .await
            .tap_err(|x| debug_if!("Got an error from EspoCRM: {x}"))
            .tap_ok(|x| debug_if!("Got response from EspoCRM with status code: {}", x.status()))
    }
}
