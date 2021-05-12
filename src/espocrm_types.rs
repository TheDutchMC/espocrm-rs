use std::collections::HashMap;
use std::fmt;

pub enum Value {
    Map(Option<HashMap<String, Value>>),
    String(Option<String>),
    Array(Option<Vec<Value>>),
    Integer(Option<i64>),
    Boolean(Option<bool>)
}

impl Value {
    pub fn eq(&self, b: &Value) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(b)
    }

    pub fn map(v: HashMap<String, Value>) -> Self {
        Value::Map(Some(v))
    }

    pub fn string(v: String) -> Self {
        Value::String(Some(v))
    }

    pub fn str(v: &str) -> Self {
        Value::String(Some(v.to_string()))
    }

    pub fn array(v: Vec<Value>) -> Self {
        Value::Array(Some(v))
    }

    pub fn int(v: i64) -> Self {
        Value::Integer(Some(v))
    }

    pub fn bool(v: bool) -> Self {
        Value::Boolean(Some(v))
    }
}

pub struct Params {
    pub offset: Option<i64>,
    pub max_size: Option<i64>,
    pub select: Option<String>,
    pub r#where: Option<Vec<Where>>,
    pub primary_filter: Option<String>,
    pub bool_filter_list: Option<Vec<String>>,
    pub order_by: Option<OrderBy>
}

impl Default for Params {
    fn default() -> Self {
        Self::new()
    }
}

impl Params {
    pub fn new() -> Self {
        Self {
            offset: None,
            max_size: None,
            select: None,
            r#where: None,
            primary_filter: None,
            bool_filter_list: None,
            order_by: None
        }
    }
}

#[derive(Debug)]
pub enum OrderBy {
    Asc,
    Desc
}

impl fmt::Display for OrderBy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Where {
    pub r#type: FilterType,
    pub attribute: String,
    pub value: Option<Value>
}

impl Where {
    pub fn new(filter_type: FilterType, attribute: &str, value: Option<Value>) -> Self {
        Where {
            r#type: filter_type,
            attribute: attribute.to_string(),
            value
        }
    }
}

#[derive(Debug)]
pub enum FilterType {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEquals,
    LessThanOrEquals,
    IsNull,
    IsNotNull,
    IsTrue,
    IsFalse,
    LinkedWith,
    NotLinkedWith,
    IsLinked,
    IsNotLinked,
    In,
    NotIn,
    Contains,
    NotContains,
    StartsWith,
    EndsWith,
    Like,
    NotLike,
    Or,
    AndToday,
    Past,
    Future,
    LastSevenDays,
    CurrentMonth,
    LastMonth,
    NextMonth,
    CurrentQuarter,
    LastQuarter,
    CurrentYear,
    LastYear,
    CurrentFiscalYear,
    LastFiscalYear,
    CurrentFiscalQuarter,
    LastFiscalQuarter,
    LastXDays,
    NextXDays,
    OlderThanXDays,
    AfterXDays,
    Between,
    ArrayAnyOf,
    ArrayNoneOf,
    ArrayAlOf,
    ArrayIsEmpty,
    ArrayIsNotEmpty,
}

impl fmt::Display for FilterType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}