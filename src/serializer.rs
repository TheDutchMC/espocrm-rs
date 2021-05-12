use crate::espocrm_types::{Params, Value};
use std::collections::HashMap;
use urlencoding::encode;

pub fn serialize(input: Params) -> Result<String, &'static str> {
    let mut builder = String::new();

    if input.select.is_some() {
        if builder.len() > 0 {
            builder.push('&');
        }

        builder.push_str(&input.select.unwrap())
    }

    if input.order_by.is_some() {
        if builder.len() > 0 {
            builder.push('&');
        }

        builder.push_str(&input.order_by.unwrap().to_string())
    }

    if input.offset.is_some() {
        if builder.len() > 0 {
            builder.push('&');
        }

        builder.push_str(&input.offset.unwrap().to_string())
    }

    if input.bool_filter_list.is_some() {
        if builder.len() > 0 {
            builder.push('&');
        }

        let mut bool_filter_as_vals = Vec::new();
        for v in input.bool_filter_list.unwrap() {
            bool_filter_as_vals.push(Value::string(v));
        }

        let mut base_params = Vec::default();
        base_params.push("boolFilterList".to_string());
        let str = build_url_from_vec(base_params, &bool_filter_as_vals)?;

        builder.push_str(&str);
    }

    if input.max_size.is_some() {
        if builder.len() > 0 {
            builder.push('&');
        }

        builder.push_str(&input.max_size.unwrap().to_string())
    }

    if input.primary_filter.is_some() {
        if builder.len() > 0 {
            builder.push('&');
        }

        builder.push_str(&input.primary_filter.unwrap())
    }

    if input.r#where.is_some() {
        if builder.len() > 0 {
            builder.push('&');
        }

        for v in input.r#where.unwrap() {
            if v.value.is_some() {
                let v_val = v.value.unwrap();
                match v_val {
                    Value::Map(v_inner) => {
                        let mut base_params = Vec::default();
                        base_params.push("where".to_string());
                        let str = build_url_from_map(base_params, v_inner.as_ref().unwrap())?;
                        builder.push_str(&str);
                    }
                    Value::Array(v_inner) => {
                        let mut base_params = Vec::default();
                        base_params.push("where".to_string());
                        let str = build_url_from_vec(base_params, v_inner.as_ref().unwrap())?;
                        builder.push_str(&str)
                    }
                    _ => {
                        let v_str = match v_val {
                            Value::Boolean(v_inner) => {
                                v_inner.as_ref().unwrap().to_string()
                            }
                            Value::Integer(v_inner) => {
                                v_inner.as_ref().unwrap().to_string()
                            }
                            Value::String(v_inner) => {
                                v_inner.as_ref().unwrap().to_string()
                            }
                            _ => { "".to_string() }
                        };

                        builder.push_str(&format!("{}={}", encode("where"), encode(&v_str)));
                    }
                }
            }
        }
    }

    Ok(builder)
}

fn build_url_from_map(base_params: Vec<String>, map: &HashMap<String, Value>) -> Result<String, &'static str> {
    let mut builder = String::new();

    for (k, v) in map {
        if builder.len() > 0 {
            builder.push('&');
        }

        match v {
            Value::Map(v_inner) => {
                let mut base_param_2 = Vec::default();
                concatenate_vec(&mut base_param_2, &base_params);
                base_param_2.push(k.clone());
                let str = build_url_from_map(base_param_2, v_inner.as_ref().unwrap())?;
                builder.push_str(&str);
            }
            Value::Array(v_inner) => {
                let mut base_param_2 = Vec::default();
                concatenate_vec(&mut base_param_2, &base_params);
                base_param_2.push(k.clone());
                let str = build_url_from_vec(base_param_2, v_inner.as_ref().unwrap())?;
                builder.push_str(&str)
            },
            _ => {
                let v_str = match v {
                    Value::Boolean(v_inner) => {
                        v_inner.as_ref().unwrap().to_string()
                    }
                    Value::Integer(v_inner) => {
                        v_inner.as_ref().unwrap().to_string()
                    }
                    Value::String(v_inner) => {
                        v_inner.as_ref().unwrap().to_string()
                    }
                    _ => { "".to_string() }
                };

                let token = format!("{}[{}]={}", get_base_param_string(&base_params), k.clone(), encode(&v_str));
                builder.push_str(&token);
            }
        }
    }

    Ok(builder)
}

fn build_url_from_vec(base_params: Vec<String>, vec: &Vec<Value>) -> Result<String, &'static str> {
    let mut builder = String::new();

    let mut i = 0;
    for v in vec {
        i += 1;

        if i > 0 {
            builder.push('&');
        }

        match v {
            Value::Map(v_inner) => {
                let mut base_params_2 = Vec::new();
                concatenate_vec(&mut base_params_2, &base_params);
                base_params_2.push(i.to_string());

                let str = build_url_from_map(base_params_2, v_inner.as_ref().unwrap())?;
                builder.push_str(&str);
            }
            Value::Array(v_inner) => {
                let mut base_params_2 = Vec::new();
                concatenate_vec(&mut base_params_2, &base_params);
                base_params_2.push(i.to_string());
                let str = build_url_from_vec(base_params_2, v_inner.as_ref().unwrap())?;
                builder.push_str(&str);
            }
            _ => {
                let v_str = match v {
                    Value::Boolean(v_inner) => {
                        v_inner.as_ref().unwrap().to_string()
                    }
                    Value::Integer(v_inner) => {
                        v_inner.as_ref().unwrap().to_string()
                    }
                    Value::String(v_inner) => {
                        v_inner.as_ref().unwrap().to_string()
                    }
                    _ => { "".to_string() }
                };

                let token = format!("{}[{}]={}", get_base_param_string(&base_params), i.to_string(), encode(&v_str));
                builder.push_str(&token);
            }
        }
    }

    Ok(builder)
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

fn concatenate_vec<T: Clone>(vec1: &mut Vec<T>, vec2: &Vec<T>) {
    for i in vec2.clone() {
        let i_clone = i.clone();
        vec1.push(i_clone);
    }
}