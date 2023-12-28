use serde::Serialize;

#[derive(Debug, Serialize, Clone, Default)]
pub struct Dataset<'a> {
    pub label: &'a str,
    pub data: &'a [i64], // make generic
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct Data<'a> {
    pub labels: &'a [i64], // make generic
    pub datasets: &'a [&'a Dataset<'a>],
}
#[derive(Debug, Serialize, Clone, Default)]
pub struct Config<'a> {
    #[serde(rename = "type")]
    pub chart_type: Option<&'a str>,
    pub data: Option<&'a Data<'a>>,
    pub options: Option<()>, // not yet implemented
}
