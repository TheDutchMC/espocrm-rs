use std::collections::HashMap;

pub enum Value {
    Map(HashMap<String, Value>),
    String(String),
    Array(Vec<Value>),
    Integer(i64),
    Boolean(bool)
}

pub struct Params {
    pub offset: Option<i64>,
    pub max_size: Option<i64>,
    pub select: Option<String>,
    pub r#where: Option<Where>,
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

pub enum OrderBy {
    Asc,
    Desc
}

pub struct Where {
    pub r#type: FilterType,
    pub attribute: String,
    pub value: Option<Value>
}

impl Default for Where {
    fn default() {

    }
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