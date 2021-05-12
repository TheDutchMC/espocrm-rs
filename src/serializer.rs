use crate::espocrm_types::{Params, Value};
use std::collections::HashMap;

pub fn serialize(input: Params) -> Result<String, &str> {

}

fn build_url_from_map(base_params: Vec<String>, map: HashMap<String, Value>) -> Result<String, &str> {
    let mut builder = String::new();

    for (k, v) in map {
        if builder.len() > 0 {
            builder.push('&');
        }

        if v == Value::Map {
            let mut base_param_2 = Vec::default();
            concatenate_vec(&mut base_param_2, &base_params);
            base_param_2.push(k);
            let str = build_url_from_map(base_param_2, v.into())?;
            builder.push_str(&str);

        } else if v == Value::Array {
            let mut base_param_2 = Vec::default();
            concatenate_vec(&mut base_param_2, &base_params);
            base_param_2.push(k);
            let str = build_url_from_vec(base_params_2, v.into())?;
            builder.push_str(&str);

        } else {
            let token = get_base_param_string(&base_params);
        }
    }

    Ok(builder)
}

fn build_url_from_vec(base_params: Vec<String>, vec: &Vec<Value>) -> Result<String, &str> {
    let mut builder = String::new();

    let mut index = 0;
    for v in vec {
        if i > 0 {
            builder.push('&');
        }

        if v == Value::Map {

        } else if v == Value::Array {

        } else {

        }

    }

}

fn get_base_param_string(base_params: &Vec<String>) -> String {
    let mut builder = String::new();
    for i in 0..base_params.len() {
        let s = base_params.get(i).unwrap();
        if i == 0 {
            builder.push_str(s);
        } else {
            builder.push_str(&format!("[{}]", s))
        }
    }

    builder
}

fn concatenate_vec<T>(vec1: &mut Vec<T>, vec2: &Vec<T>) {
    for i in vec2 {
        vec1.push(i);
    }
}