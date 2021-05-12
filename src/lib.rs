//! # espocrm-rs
//!
//! The `espocrm-rs` crate provides an API Client for EspoCRM. This client is based on the official PHP API client provided by the EspoCRM team.
//! You can find this client [here](https://docs.espocrm.com/development/api-client-php/).
//!
//! ## Getting started
//! To get started you'll have to provide the URL where EspoCRM is located at. You will also have to set the way you want to authenticate with EspoCRM.
//! This can be done in one of the following ways:
//! - Username+Password
//! - API Key
//! - HMAC (Recommended)
//!
//! The following example creates an EspoApiClient with HMAC authorization
//! ```rust
//! use espocrm_rs::EspoApiClient;
//!
//! let client = EspoApiClient::new("https://espocrm.example.com")
//!     .set_api_key("Your API Key here")
//!     .set_secret_key("Your API Secret")
//!     .build();
//! ```
//!
//! The following example creates an EspoApiClient with API Key authorization
//! ```rust
//! use espocrm_rs::EspoApiClient;
//! let client = EspoApiClient::new("https://espocrm.example.com")
//!     .set_api_key("Your API Key here")
//!     .build();
//! ```
//!
//! The following example creates an EspoApiClient with Username+Password authorization.
//! **This is highly discouraged!**
//! ```rust
//! use espocrm_rs::EspoApiClient;
//! let client = EspoApiClient::new("https://espocrm.example.com")
//!     .set_username("Your Username here")
//!     .set_password("Your Password here")
//!     .build();
//! ```
//!
//! # Making a request
//! To make a request, you need to know a couple things:
//! - The request method to use
//! - On what to perform the request
//! - Optionally, any data needed for the request
//!
//! Most of these things are laid out pretty well in the EspoCRM API documentation [here](https://docs.espocrm.com/development/api/)
//! ```rust
//! use espocrm_rs::EspoApiClient;
//! use reqwest::Method;
//! use serde::Serialize;
//!
//! #[derive(Serialize)]
//! struct RequestData {
//!     offset:     i64,
//!
//!     #[serde(rename(serialize = "where"))]
//!     filter:     Vec<RequestFilter>,
//!
//!     #[serde(rename(serialzie = "orderBy"))]
//!     order_by:   String,
//!     order:      String
//! }
//!
//! #[derive(Serialize)]
//! struct RequestFilter {
//!     #[serde(rename(serialize = "filterType"))]
//!     filter_type:    String,
//!     attribute:      String
//! }
//!
//! let client = EspoApiClient::new("https://espocrm.example.com")
//!     .set_api_key("Your API Key here")
//!     .set_secret_key("Your API Secret")
//!     .build();
//!
//! let request_filter = RequestFilter {
//!     filter_type: "isNotNull".to_string(),
//!     attribute: "firstName".to_string()
//! };
//!
//! let request_data = RequestData {
//!     offset: 0,
//!     filter: vec![request_filter],
//!     order_by: "createdAt".to_string(),
//!     order: "desc".to_string()
//! };
//!
//! let result = client.request(Method::GET, "Contact".to_string(), Some(request_data));
//! ```
//!
//! These structs weren't pulled out of thin air. Everything you need to know about this is described [here](https://docs.espocrm.com/development/api-search-params/)

mod espocrm;
mod espocrm_types;
mod serializer;

pub use espocrm::*;

#[cfg(test)]
mod tests {
    use crate::espocrm::EspoApiClient;
    const URL: &str = "foo";

    #[test]
    fn url() {
        let client = EspoApiClient::new(URL);
        assert_eq!(client.url, "foo".to_string());
    }

    #[test]
    fn username() {
        let client = EspoApiClient::new(URL)
            .set_username("bar")
            .build();

        assert_eq!(Some("bar".to_string()), client.username);
    }

    #[test]
    fn password() {
        let client = EspoApiClient::new(URL)
            .set_password("bar")
            .build();

        assert_eq!(Some("bar".to_string()), client.password);
    }

    #[test]
    fn api_key() {
        let client = EspoApiClient::new(URL)
            .set_api_key("bar")
            .build();

        assert_eq!(Some("bar".to_string()), client.api_key);
    }

    #[test]
    fn secret_key() {
        let client = EspoApiClient::new(URL)
            .set_secret_key("bar")
            .build();

        assert_eq!(Some("bar".to_string()), client.secret_key);
    }

    #[test]
    fn full() {
        let client = EspoApiClient::new(URL)
            .set_username("username")
            .set_password("password")
            .set_api_key("api_key")
            .set_secret_key("secret_key")
            .build();

        assert_eq!(Some("username".to_string()), client.username);
        assert_eq!(Some("password".to_string()), client.password);
        assert_eq!(Some("api_key".to_string()), client.api_key);
        assert_eq!(Some("secret_key".to_string()), client.secret_key);
        assert_eq!("foo".to_string(), client.url);
    }

    #[test]
    fn modify_url() {
        let client = EspoApiClient::new(URL)
            .set_url("bar")
            .build();

        assert_eq!("bar".to_string(), client.url)
    }

    #[test]
    fn normalize_url() {
        let client = EspoApiClient::new(URL);
        let normalized_url = client.normalize_url("Contact".to_string());

        assert_eq!(format!("{}{}Contact", URL, client.url_path), normalized_url)
    }
}
