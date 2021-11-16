use crate::file_questionary::ConfigOption::{RADIUS, RESPONSIVE};
use crate::file_questionary::DataSetOption::{FILL, ORDER, TENSION};
use rand::*;
use std::borrow::Borrow;
use std::io::{Read, Write};
use std::ops::{Add, Range};

pub fn start_questionary(attrs: Vec<&str>, file: &str) -> BuildHandle {
    let mut graphs = Vec::<Graph>::new();
    let mut file_config = FileConfig::default();
    for attr in attrs {
        println!("Settings for attribute {} in file {}", attr, file);
        if ask("Default settings ? (y/n)").eq("y") {
            graphs.push(Graph::default());
            continue;
        }
        let graph_style = GraphStyle::from(ask(&format!(
            "Graph style options are : {} .Everything must be given lowercase and underscores must be removed. (if you fuck up it will choose line)",
            GraphStyle::list_items()
        )));

        let color = Color::from(ask("Choose a color for your graph help https://www.google.com/search?client=firefox-b-d&q=Hex+color+picker . Format 'R,G,B'"));

        let configs = until_question(
            &format!(
                "Config options are {}. Format type lowercase => type=VALUE",
                ConfigOption::list_items()
            ),
            "Do you want to choose a config setting (y/n)",
        );
        let data_configs = until_question(
            &format!(
                "Data config options are {}. Format type lowercase => type=VALUE",
                DataSetOption::list_items()
            ),
            "Do you want to choose a data config setting (y/n)",
        );
        let mut eq = None;
        if ask("Equation (y/n)").eq("y") {
            let equation = ask("A equation is going to manipulate the data ex. =>  x*5'");
            eq = Some(equation);
        }

        graphs.push(Graph {
            config: configs
                .iter()
                .map(|raw| ConfigOption::from(raw.clone()))
                .collect(),
            style: graph_style,
            data_config: data_configs
                .iter()
                .map(|raw| DataSetOption::from(raw.clone()))
                .collect(),
            color,
            eq,
        });
        if ask("Range (y/n)").eq("y") {
            let range_data = ask("Give parameters => Format 'time_min,time_max'");
            let extracted_range_data = range_data.split_once(",").unwrap_or(("", ""));
            file_config.range = Some(
                extracted_range_data.0.parse::<i64>().unwrap_or(0)
                    ..extracted_range_data.1.parse::<i64>().unwrap_or(0),
            );
        }
        if ask("Usage Rate (y/n)").eq("y") {
            file_config.usage_rate = Some(
                ask("Usage Rate format n'th point")
                    .parse::<i64>()
                    .unwrap_or(0),
            );
        }
        println!("finished for attr {}", attr);
    }

    BuildHandle {
        nodes: graphs,
        config: file_config,
    }
}

fn until_question(question: &str, next_question: &str) -> Vec<String> {
    let mut res = Vec::<String>::new();
    loop {
        if ask(next_question).eq("n") {
            break;
        }
        res.push(ask(question));
    }
    res
}

pub fn ask(question: &str) -> String {
    let mut res = "".to_owned();
    std::io::stdout().flush();
    println!("{}", question);
    let _ = std::io::stdin().read_line(&mut res).unwrap();
    while res.as_bytes().last().unwrap_or(&255) < &31 {
        res.pop();
    }
    res
}
#[derive(Debug)]
pub struct BuildHandle {
    pub nodes: Vec<Graph>,
    pub config: FileConfig,
}
#[derive(Debug)]
pub struct Graph {
    pub eq: Option<String>,
    pub color: Color,
    pub config: Vec<ConfigOption>,
    pub data_config: Vec<DataSetOption>,
    pub style: GraphStyle,
}

impl Graph {
    pub fn data_adapter(&self, data: &Vec<f64>) -> Vec<f64> {
        let expr: meval::Expr = self.eq.clone().unwrap_or("x".to_owned()).parse().unwrap();
        let func = expr.bind("x").unwrap();
        data.iter().map(|data| func(*data)).collect()
    }
}

impl Default for Graph {
    fn default() -> Self {
        Graph {
            eq: None,
            color: Color::default(),
            config: [ConfigOption::RADIUS(0.0), ConfigOption::RESPONSIVE(true)].to_vec(),
            data_config: Vec::new(),
            style: GraphStyle::LINE,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Color(pub u8, pub u8, pub u8);

impl Into<String> for Color {
    fn into(self) -> String {
        format!("'rgb({},{},{})'", self.0, self.1, self.2)
    }
}
impl Default for Color {
    fn default() -> Self {
        Color(rand::random(), rand::random(), rand::random())
    }
}
impl From<String> for Color {
    fn from(str: String) -> Self {
        let mut res = Color(0, 0, 0);
        for x in str.split(',').enumerate() {
            match x.0 {
                0 => res.0 = x.1.parse::<u8>().unwrap_or(0),
                1 => res.1 = x.1.parse::<u8>().unwrap_or(0),
                2 => res.2 = x.1.parse::<u8>().unwrap_or(0),
                _ => {}
            }
        }
        res
    }
}
#[derive(Debug, Clone)]
pub enum GraphStyle {
    BAR,
    LINE,
    BUBBLE,
    DOUGHNUT,
    PIE,
    POLAR_AREA,
    RADAR,
}
impl SumEnum for GraphStyle {
    fn list_items() -> String {
        "BAR,
        LINE,
        BUBBLE,
        DOUGHNUT,
        PIE,
        POLAR_AREA,
        RADAR"
            .to_owned()
    }
}
impl Into<String> for GraphStyle {
    fn into(self) -> String {
        match self {
            GraphStyle::BAR => "bar",
            GraphStyle::LINE => "line",
            GraphStyle::BUBBLE => "bubble",
            GraphStyle::DOUGHNUT => "doughnut",
            GraphStyle::PIE => "pie",
            GraphStyle::POLAR_AREA => "polarArea",
            GraphStyle::RADAR => "radar",
        }
        .to_owned()
    }
}
impl From<String> for GraphStyle {
    fn from(string: String) -> Self {
        match string {
            str if str.eq_ignore_ascii_case("bar") => GraphStyle::BAR,
            str if str.eq_ignore_ascii_case("line") => GraphStyle::LINE,
            str if str.eq_ignore_ascii_case("bubble") => GraphStyle::BUBBLE,
            str if str.eq_ignore_ascii_case("doughnut") => GraphStyle::DOUGHNUT,
            str if str.eq_ignore_ascii_case("pie") => GraphStyle::PIE,
            str if str.eq_ignore_ascii_case("polar_area") => GraphStyle::POLAR_AREA,
            str if str.eq_ignore_ascii_case("radar") => GraphStyle::RADAR,
            _ => GraphStyle::LINE,
        }
    }
}

#[derive(Clone, Debug)]
pub enum DataSetOption {
    TENSION(f64),
    FILL(bool),
    ORDER(u64),
}
impl From<String> for DataSetOption {
    fn from(string: String) -> Self {
        match string {
            str if str.starts_with("tension") => {
                return TENSION(
                    str.split_once("=")
                        .unwrap_or(("", ""))
                        .1
                        .parse::<f64>()
                        .unwrap_or(0.0),
                )
            }

            str if str.starts_with("fill") => {
                return FILL(
                    str.split_once("=")
                        .unwrap_or(("", ""))
                        .1
                        .parse::<bool>()
                        .unwrap_or(false),
                )
            }

            str if str.starts_with("order") => {
                return ORDER(
                    str.split_once("=")
                        .unwrap_or(("", ""))
                        .1
                        .parse::<u64>()
                        .unwrap_or(0),
                )
            }

            _ => return FILL(false),
        }
    }
}
impl SumEnum for DataSetOption {
    fn list_items() -> String {
        "TENSION(Rational),
         FILL(Boolean),
        ORDER(Natural)"
            .to_owned()
    }
}
impl Into<String> for DataSetOption {
    fn into(self) -> String {
        match self {
            DataSetOption::TENSION(tension) => {
                format!("tension : {} ", tension)
            }
            DataSetOption::FILL(fill) => {
                format!("fill : {} ", fill.to_string())
            }
            DataSetOption::ORDER(order) => {
                format!("order : {} ", order)
            }
        }
    }
}
#[derive(Clone, Debug)]
pub enum ConfigOption {
    RADIUS(f64),
    RESPONSIVE(bool),
}

impl From<String> for ConfigOption {
    fn from(string: String) -> Self {
        match string {
            str if str.starts_with("radius") => RADIUS(
                str.split_once("=")
                    .unwrap_or(("", ""))
                    .1
                    .parse::<f64>()
                    .unwrap_or(0.0),
            ),
            str if str.starts_with("responsive") => RESPONSIVE(
                str.split_once("=")
                    .unwrap_or(("", ""))
                    .1
                    .parse::<bool>()
                    .unwrap_or(false),
            ),
            _ => RESPONSIVE(false),
        }
    }
}

impl SumEnum for ConfigOption {
    fn list_items() -> String {
        "RADIUS(Rational),
        RESPONSIVE(Boolean),"
            .to_owned()
    }
}

impl Into<String> for ConfigOption {
    fn into(self) -> String {
        match self {
            ConfigOption::RADIUS(rad) => {
                format!("radius : {}", rad)
            }
            ConfigOption::RESPONSIVE(res) => {
                format!("responsive : {}", res.to_string())
            }
        }
    }
}
#[derive(Debug, Clone)]
pub struct FileConfig {
    pub usage_rate: Option<i64>,
    pub range: Option<Range<i64>>,
}

impl FileConfig {
    pub fn data_adapter(&self, data: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        let mut points_to_remove: Vec<usize> = Vec::new();
        for (p, v) in data[0].iter().enumerate() {
            match &self.usage_rate {
                None => {
                    continue;
                }
                Some(x) => {
                    if p as i64 % x != 0 {
                        points_to_remove.push(p);
                        continue;
                    }
                }
            }
            match &self.range {
                None => {
                    continue;
                }
                Some(x) => {
                    if !x.contains(&(v.round() as i64)) {
                        points_to_remove.push(p);
                    }
                }
            }
        }
        let mut cp: Vec<Vec<f64>> = Vec::new();
        for _ in 0..data.len() {
            cp.push(Vec::new());
        }

        for x in data.iter().enumerate() {
            for y in x.1.iter().enumerate() {
                if points_to_remove.contains(&y.0) {
                    continue;
                }
                cp[x.0].push(*y.1);
            }
        }
        cp
    }
}

impl Default for FileConfig {
    fn default() -> Self {
        FileConfig {
            usage_rate: Some(100),
            range: None,
        }
    }
}

trait SumEnum {
    fn list_items() -> String;
}
