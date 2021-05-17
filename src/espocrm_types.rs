use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(unused)]
pub enum Value {
    String(Option<String>),
    Array(Option<Vec<Value>>),
    Integer(Option<i64>),
    Boolean(Option<bool>)
}

#[allow(unused)]
impl Value {
    pub fn eq(&self, b: &Value) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(b)
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

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(unused)]
pub struct Params {
    pub offset: Option<i64>,
    pub max_size: Option<i64>,
    pub select: Option<String>,
    pub r#where: Option<Vec<Where>>,
    pub primary_filter: Option<String>,
    pub bool_filter_list: Option<Vec<String>>,
    pub order: Option<Order>,
    pub order_by: Option<String>
}

impl Default for Params {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(unused)]
impl Params {
    pub fn new() -> Self {
        Self {
            offset: None,
            max_size: None,
            select: None,
            r#where: None,
            primary_filter: None,
            bool_filter_list: None,
            order_by: None,
            order: None
        }
    }

    pub fn set_offset(&mut self, offset: i64) -> &mut Self {
        self.offset = Some(offset);
        self
    }

    pub fn set_max_size(&mut self, max_size: i64) -> &mut Self {
        self.max_size = Some(max_size);
        self
    }

    pub fn set_select(&mut self, select: &str) -> &mut Self {
        self.select = Some(select.to_string());
        self
    }

    pub fn set_where(&mut self, r#where: Vec<Where>) -> &mut Self {
        self.r#where = Some(r#where);
        self
    }

    pub fn set_primary_filter(&mut self, primary_filter: &str) -> &mut Self {
        self.primary_filter = Some(primary_filter.to_string());
        self
    }

    pub fn set_bool_filter_list(&mut self, bool_filter_list: Vec<String>) ->&mut Self {
        self.bool_filter_list = Some(bool_filter_list);
        self
    }

    pub fn set_order(&mut self, order: Order) -> &mut Self {
        self.order = Some(order);
        self
    }

    pub fn set_order_by(&mut self, order_by: &str) -> &mut Self {
        self.order_by = Some(order_by.to_string());
        self
    }

    pub fn build(&self) -> Self {
        self.clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(unused)]
pub enum Order {
    Asc,
    Desc
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(unused)]
pub struct Where {
    pub r#type: FilterType,
    pub attribute: String,
    pub value: Option<Value>
}

#[allow(unused)]
impl Where {
    pub fn new(filter_type: FilterType, attribute: &str, value: Option<Value>) -> Self {
        Where {
            r#type: filter_type,
            attribute: attribute.to_string(),
            value
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(unused)]
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
    ArrayAllOf,
    ArrayIsEmpty,
    ArrayIsNotEmpty,
}

impl fmt::Display for FilterType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}