use crate::espocrm_types::{Params, Value};
use urlencoding::encode;

pub fn serialize(input: Params) -> Result<String, &'static str> {
    let mut builder = String::new();

    if input.select.is_some() {
        if builder.len() > 0 {
            builder.push('&');
        }

        builder.push_str(&format!("select={}", input.select.unwrap()))
    }

    if input.order_by.is_some() {
        if builder.len() > 0 {
            builder.push('&');
        }

        builder.push_str(&format!("orderBy={}", input.order_by.unwrap().to_string()))
    }

    if input.order.is_some() {
        if builder.len() > 0 {
            builder.push('&');
        }

        let lower_camel_case_type = {
            let mut builder = String::new();
            let mut is_first = true;
            for c in input.order.unwrap().to_string().chars() {
                if is_first {
                    let mut x = String::new();
                    x.push(c);
                    let x_upper = x.to_lowercase();

                    builder.push_str(&x_upper);
                    is_first = false;
                } else {
                    builder.push(c)
                }
            }

            builder
        };

        builder.push_str(&format!("order={}", lower_camel_case_type))
    }

    if input.offset.is_some() {
        if builder.len() > 0 {
            builder.push('&');
        }

        builder.push_str(&format!("offset={}", input.offset.unwrap().to_string()))
    }

    if input.bool_filter_list.is_some() {
        todo!();
    }

    if input.max_size.is_some() {
        if builder.len() > 0 {
            builder.push('&');
        }

        builder.push_str(&format!("maxSize={}", input.max_size.unwrap().to_string()));
    }

    if input.primary_filter.is_some() {
        if builder.len() > 0 {
            builder.push('&');
        }

        builder.push_str(&format!("primaryFilter={}", input.primary_filter.unwrap()));
    }

    if input.r#where.is_some() {
        if builder.len() > 0 {
            builder.push('&');
        }

        let mut i = 0;
        for v in input.r#where.unwrap() {
            if i > 0 {
                builder.push('&');
            }

            let lower_camel_case_type = {
                let mut builder = String::new();
                let mut is_first = true;
                for c in v.r#type.to_string().chars() {
                    if is_first {
                        let mut x = String::new();
                        x.push(c);
                        let x_upper = x.to_lowercase();

                        builder.push_str(&x_upper);
                        is_first = false;
                    } else {
                        builder.push(c)
                    }
                }

                builder
            };

            builder.push_str(&format!("{}={}", &encode(&format!("where[{}][type]", i)), lower_camel_case_type));
            builder.push('&');
            builder.push_str(&format!("{}={}", &encode(&format!("where[{}][attribute]", i)), v.attribute));

            if v.value.is_some() {
                let val_unwrapped = v.value.unwrap();
                match val_unwrapped {
                    Value::Array(arr) => {
                        let arr_1 = arr.unwrap();

                        let mut j = 0;
                        for elem in arr_1 {
                            let elem_v_inner = match elem {
                                Value::String(inner) => {
                                    inner.unwrap()
                                }
                                Value::Integer(inner) => {
                                    inner.unwrap().to_string()
                                }
                                Value::Boolean(inner) => {
                                    inner.unwrap().to_string()
                                }
                                Value::Array(_) => {
                                    unimplemented!();
                                }
                            };

                            builder.push('&');
                            builder.push_str(&format!("{}={}", &encode(&format!("where[{}][value][{}]", i, j)), elem_v_inner));

                            j+=1;
                        }
                    }
                    _ => {
                        let elem_v_inner = match val_unwrapped {
                            Value::String(inner) => {
                                inner.unwrap()
                            }
                            Value::Integer(inner) => {
                                inner.unwrap().to_string()
                            }
                            Value::Boolean(inner) => {
                                inner.unwrap().to_string()
                            }
                            _ => {panic!("Unreachable")}
                        };

                        builder.push('&');
                        builder.push_str(&format!("{}={}", &encode(&format!("where[{}][value]", i)), elem_v_inner));
                    }
                }
            }

            i+=1;
        }
    }

    Ok(builder)
}