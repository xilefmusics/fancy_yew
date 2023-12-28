use serde::Serialize;
use serde_json;

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

pub struct ConfigBuilder<'a> {
    config: Config<'a>,
}

impl<'a> ConfigBuilder<'a> {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    pub fn build(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&Config {
            chart_type: Some("line"),
            data: Some(&Data {
                labels: &vec![1, 2, 3, 4, 5, 6],
                datasets: &vec![&Dataset {
                    label: "Widget Data",
                    data: &vec![10, 35, 30, 20, 100, 15],
                }],
            }),
            options: None,
        })
    }
}
