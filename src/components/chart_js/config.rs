use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize, Clone, Default)]
struct Options {
    pub responsive: bool,
    #[serde(rename = "maintainAspectRatio")]
    pub maintain_aspect_ratio: bool,
}

#[derive(Debug, Serialize, Clone, Default)]
struct Dataset<'a> {
    label: &'a str,
    data: &'a [i64], // make generic
}

#[derive(Debug, Serialize, Clone, Default)]
struct Data<'a> {
    labels: &'a [i64], // make generic
    datasets: &'a [&'a Dataset<'a>],
}
#[derive(Debug, Serialize, Clone, Default)]
struct Config<'a> {
    #[serde(rename = "type")]
    chart_type: Option<&'a str>,
    data: Option<&'a Data<'a>>,
    options: Option<&'a Options>,
}

pub struct ConfigBuilder<'a> {
    _config: Config<'a>,
}

impl<'a> ConfigBuilder<'a> {
    pub fn new() -> Self {
        Self {
            _config: Config::default(),
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
            options: Some(&Options {
                responsive: true,
                maintain_aspect_ratio: false,
            }),
        })
    }
}
