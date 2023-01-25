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
//! # Making a GET request
//! To make a request, you need to know a couple things:
//! - The request method to use
//! - On what to perform the request
//! - Optionally, any data needed for the request
//!
//! Most of these things are laid out pretty well in the EspoCRM API documentation [here](https://docs.espocrm.com/development/api/)
//! ```rust
//! use espocrm_rs::{EspoApiClient, Params, Where, FilterType, Value, NoGeneric, Method};
//!
//! let params = Params::default()
//!     .set_offset(0)
//!     .set_where(vec![
//!         Where {
//!             r#type: FilterType::IsTrue,
//!             attribute: "exampleField".to_string(),
//!             value: None
//!         },
//!         Where {
//!             r#type: FilterType::ArrayAnyOf,
//!             attribute: "exampleField2".to_string(),
//!             value: Some(Value::array(vec![
//!                 Value::str("a"),
//!                 Value::str("b"),
//!                 Value::str("c")
//!             ]))
//!         }
//!     ])
//!     .build();
//!
//! let client = EspoApiClient::new("https://espocrm.example.com")
//!     .set_secret_key("Your Secret Key")
//!     .set_api_key("Your api key")
//!     .build();
//!
//! let result = client.request::<NoGeneric, &str>(Method::Get, "Contact", Some(params), None);
//! ```
//!
//! # Making a POST, PUT or DELETE request
//! These are all similar in working. They'll serialize your data into json using Serde's serialize trait
//!
//! ```rust
//! use espocrm_rs::{EspoApiClient, Method};
//! use serde::Serialize;
//!
//! #[derive(Serialize, Clone)]
//! struct MyData {
//!     some_value:         String,
//!     some_other_value:   i64
//! }
//!
//! let client = EspoApiClient::new("https://espocrm.example.com")
//!     .set_secret_key("Your Secret Key")
//!     .set_api_key("Your api key")
//!     .build();
//!
//! let data = MyData {
//!     some_value: "value".to_string(),
//!     some_other_value: 10
//! };
//!
//! let result = client.request(Method::Post, "Contact", None, Some(data));
//!```
//!

extern crate core;

mod espocrm_api_client;
mod espocrm_types;
mod serializer;
mod tracing_if;

pub use espocrm_api_client::*;
pub use espocrm_types::*;

#[cfg(test)]
mod tests {
    use crate::espocrm_api_client::EspoApiClient;
    use crate::espocrm_types::{FilterType, Order, Params, Value, Where};
    use crate::serializer::serialize;
    use std::collections::HashSet;
    use std::hash::Hash;

    const URL: &str = "foo";

    fn assert_eq_unsorted_vec<T: Eq + Hash>(a: &Vec<T>, b: &Vec<T>) -> bool {
        let a: HashSet<_> = a.iter().collect();
        let b: HashSet<_> = b.iter().collect();

        a == b
    }

    #[test]
    fn url() {
        let client = EspoApiClient::new(URL);
        assert_eq!(client.url, "foo".to_string());
    }

    #[test]
    fn username() {
        let client = EspoApiClient::new(URL).set_username("bar").build();

        assert_eq!(Some("bar".to_string()), client.username);
    }

    #[test]
    fn password() {
        let client = EspoApiClient::new(URL).set_password("bar").build();

        assert_eq!(Some("bar".to_string()), client.password);
    }

    #[test]
    fn api_key() {
        let client = EspoApiClient::new(URL).set_api_key("bar").build();

        assert_eq!(Some("bar".to_string()), client.api_key);
    }

    #[test]
    fn secret_key() {
        let client = EspoApiClient::new(URL).set_secret_key("bar").build();

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
        let client = EspoApiClient::new(URL).set_url("bar").build();

        assert_eq!("bar".to_string(), client.url)
    }

    #[test]
    fn normalize_url() {
        let client = EspoApiClient::new(URL);
        let normalized_url = client.normalize_url("Contact".to_string());

        assert_eq!(format!("{}{}Contact", URL, client.url_path), normalized_url)
    }

    #[test]
    fn serialize_basic() {
        let params = Params::new().set_offset(0).set_order(Order::Desc).build();

        let serialized = serialize(params).unwrap();
        let serialized_split: Vec<_> = serialized.split("&").collect();

        let correct = vec!["order=desc", "offset=0"];
        assert!(assert_eq_unsorted_vec(&serialized_split, &correct))
    }

    #[test]
    fn serialize_without_where_value() {
        let params = Params::new()
            .set_offset(0)
            .set_where(vec![Where {
                r#type: FilterType::IsTrue,
                attribute: "exampleBoolean".to_string(),
                value: None,
            }])
            .build();

        let serialized = serialize(params).unwrap();

        assert_eq!(
            "offset=0&where%5B0%5D%5Btype%5D=isTrue&where%5B0%5D%5Battribute%5D=exampleBoolean"
                .to_string(),
            serialized
        );
    }

    #[test]
    fn serialize_with_where_string_value() {
        let params = Params::new()
            .set_offset(0)
            .set_where(vec![Where {
                r#type: FilterType::IsTrue,
                attribute: "exampleBoolean".to_string(),
                value: Some(Value::str("a")),
            }])
            .build();

        let serialized = serialize(params).unwrap();

        assert_eq!("offset=0&where%5B0%5D%5Btype%5D=isTrue&where%5B0%5D%5Battribute%5D=exampleBoolean&where%5B0%5D%5Bvalue%5D=a".to_string(), serialized);
    }

    #[test]
    fn serialize_with_where_array_value() {
        let params = Params::new()
            .set_offset(0)
            .set_where(vec![Where {
                r#type: FilterType::IsTrue,
                attribute: "exampleBoolean".to_string(),
                value: Some(Value::Array(Some(vec![
                    Value::str("a"),
                    Value::str("b"),
                    Value::str("c"),
                ]))),
            }])
            .build();

        let serialized = serialize(params).unwrap();

        /*
           The left hand side has been created with the following PHP code:

           $where = [
               [
                   'type' => 'isTrue',
                   'attribute' => 'exampleBoolean',
                   'value' => ['a', 'b', 'c']
               ],
           ];

           $params = [
               'offset' => 0,
               'where' => $where
           ];

           echo http_build_query($params);
        */
        assert_eq!("offset=0&where%5B0%5D%5Btype%5D=isTrue&where%5B0%5D%5Battribute%5D=exampleBoolean&where%5B0%5D%5Bvalue%5D%5B0%5D=a&where%5B0%5D%5Bvalue%5D%5B1%5D=b&where%5B0%5D%5Bvalue%5D%5B2%5D=c".to_string(), serialized);
    }
}
