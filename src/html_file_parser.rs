use crate::file_questionary;
use crate::file_questionary::BuildHandle;
use std::fmt::format;
use std::ops::Add;

const html_top: &str= "<html><head><div><canvas id=\"myChart\"></canvas></div><script src=\"https://cdn.jsdelivr.net/npm/chart.js\"></script><script>";
const html_bottom: &str = "const config = {data: data,};const myChart = new Chart(document.getElementById('myChart'),config);</script></head><body></body></html>";

fn append_lables(page: &mut String, times: &Vec<f64>) {
    page.push_str("const labels = ");
    add_array(page, times);
    page.push_str(";");
}

fn append_data(page: &mut String, attrs: Vec<&str>, data: &Vec<Vec<f64>>, handle: BuildHandle) {
    page.push_str("const data = {labels: labels,datasets: [");
    for (p, node) in handle.nodes.iter().enumerate() {
        page.push('{');
        add_type(page, "label", attrs[p]);
        let color: String = node.color.clone().into();
        add_type(page, "backgroundColor", &color);
        add_type(page, "borderColor", &color);
        let r#type: String = node.style.clone().into();
        add_type(page, "type", &format!("\"{}\"", &r#type));

        for conf in node.data_config.iter() {
            let parsed_conf: String = conf.clone().into();
            page.push_str(&parsed_conf);
            page.push(',')
        }
        page.push_str("data : ");
        add_array(page, &data[p + 1].clone());
        page.push_str("},");
    }
    page.push_str("]};");
}
fn add_type(page: &mut String, r#type: &str, data: &str) {
    page.push_str(&format!("{} : {},", r#type, data));
}
fn add_array(page: &mut String, data: &Vec<f64>) {
    page.push('[');
    for data_point in data {
        page.push_str(&format!("{},", data_point));
    }
    page.push(']');
}

pub fn buid_html_page_str(data: Vec<Vec<f64>>, attrs: Vec<&str>, handle: BuildHandle) -> String {
    let mut page: String = "".to_owned();
    page.push_str(html_top);
    append_lables(&mut page, &data[0]);
    append_data(&mut page, attrs, &data, handle);
    page.push_str(html_bottom);
    page
}
