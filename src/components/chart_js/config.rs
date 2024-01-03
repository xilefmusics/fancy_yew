use serde::Serialize;
use serde_json;
use std::collections::HashMap;

#[derive(Debug, Serialize, Clone, Default)]
struct Dataset<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stack: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "borderColor")]
    border_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "borderWidth")]
    border_width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "backgroundColor")]
    background_color: Option<String>,
    data: &'a [f64],
}

impl<'a> Dataset<'a> {
    fn new(data: &'a [f64]) -> Self {
        Self {
            label: None,
            stack: None,
            border_color: None,
            border_width: None,
            background_color: None,
            data,
        }
    }
}

#[derive(Debug, Serialize, Clone, Default)]
struct Data<'a> {
    labels: Vec<String>,
    datasets: Vec<Dataset<'a>>,
}

#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "lowercase")]
enum ChartType {
    #[default]
    Line,
    Bar,
    Bubble,
    Doughnut,
    Pie,
}

#[derive(Debug, Serialize, Clone, Default)]
struct Config<'a> {
    #[serde(rename = "type")]
    chart_type: ChartType,
    data: Data<'a>,
    options: HashMap<String, String>,
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

    pub fn line() -> Self {
        Self::new().chart_type(ChartType::Line)
    }

    pub fn bar() -> Self {
        Self::new().chart_type(ChartType::Bar)
    }

    pub fn bubble() -> Self {
        Self::new().chart_type(ChartType::Bubble)
    }

    pub fn doughnut() -> Self {
        Self::new().chart_type(ChartType::Doughnut)
    }

    pub fn pie() -> Self {
        Self::new().chart_type(ChartType::Pie)
    }

    fn chart_type(mut self, chart_type: ChartType) -> Self {
        self.config.chart_type = chart_type;
        self
    }

    pub fn dataset(mut self, data: &'a [f64]) -> Self {
        self.config.data.datasets.push(Dataset::new(data));
        self
    }

    pub fn dataset_label(mut self, label: &'a str) -> Self {
        if let Some(dataset) = self.config.data.datasets.last_mut() {
            (*dataset).label = Some(label);
        }
        self
    }

    pub fn dataset_stack(mut self, stack: usize) -> Self {
        if let Some(dataset) = self.config.data.datasets.last_mut() {
            (*dataset).stack = Some(stack);
        }
        self
    }

    pub fn dataset_border_color_rgba(mut self, r: u8, g: u8, b: u8, a: f64) -> Self {
        if let Some(dataset) = self.config.data.datasets.last_mut() {
            (*dataset).border_color = Some(format!("rgba({},{},{},{})", r, g, b, a));
        }
        self
    }

    pub fn dataset_border_width(mut self, width: usize) -> Self {
        if let Some(dataset) = self.config.data.datasets.last_mut() {
            (*dataset).border_width = Some(width);
        }
        self
    }

    pub fn dataset_background_color_rgba(mut self, r: u8, g: u8, b: u8, a: f64) -> Self {
        if let Some(dataset) = self.config.data.datasets.last_mut() {
            (*dataset).background_color = Some(format!("rgba({},{},{},{})", r, g, b, a));
        }
        self
    }

    pub fn labels<T: ToString>(mut self, labels: &'a [T]) -> Self {
        self.config.data.labels = labels.into_iter().map(|l| l.to_string()).collect();
        self
    }

    pub fn add_option<A: ToString, B: ToString>(mut self, key: A, value: B) -> Self {
        self.config
            .options
            .insert(key.to_string(), value.to_string());
        self
    }

    pub fn rotate(self) -> Self {
        self.add_option("indexAxis", "y")
    }

    pub fn build(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self.config)
    }
}
