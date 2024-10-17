use csv::Writer;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ARFFData {
    _relation: String,
    attributes: Vec<Attribute>,
    data: Vec<DataRecord>,
}

#[derive(Debug)]
pub enum AttributeType {
    String,
    Numeric,
    Nominal(Vec<String>),
    BooleanNominal,
    Date,
}

#[derive(Debug)]
pub struct Attribute {
    name: String,
    attr_type: AttributeType,
}

#[derive(Debug)]
pub struct DataRecord {
    values: Vec<String>,
}

impl ARFFData {
    pub fn from_arff(content: &str) -> Self {
        let mut relation = String::new();
        let mut attributes = Vec::new();
        let mut data = Vec::new();
        let mut is_data_section = false;

        for line in content.lines() {
            let trimmed_line = line.trim();

            if trimmed_line.is_empty() || trimmed_line.starts_with('%') {
                continue;
            }

            if trimmed_line.starts_with("@relation") {
                relation = trimmed_line.split_whitespace().nth(1).unwrap().to_string();
            } else if trimmed_line.starts_with("@attribute") {
                let parts: Vec<&str> = trimmed_line.split_whitespace().collect();
                let name = parts[1].to_string();

                let attribute_type = match parts[2] {
                    "date" => AttributeType::Date,
                    "numeric" => AttributeType::Numeric,
                    "string" => AttributeType::String,
                    "{false," => AttributeType::BooleanNominal,
                    other if other.starts_with('{') => {
                        let mut values_section = String::new();
                        values_section.push_str(other);

                        for part in &parts[3..] {
                            values_section.push(' ');
                            values_section.push_str(part);
                            if part.ends_with('}') {
                                break;
                            }
                        }

                        let values_section =
                            values_section[1..values_section.len() - 1].to_string();

                        let mut values = Vec::new();
                        let mut current_value = String::new();
                        let mut in_quotes = false;

                        for ch in values_section.chars() {
                            if ch == '\'' {
                                in_quotes = !in_quotes;
                            } else if ch == ',' && !in_quotes {
                                values.push(current_value.trim().to_string());
                                current_value.clear();
                            } else {
                                current_value.push(ch);
                            }
                        }

                        if !current_value.is_empty() {
                            values.push(current_value.trim().to_string());
                        }

                        AttributeType::Nominal(values)
                    }
                    _ => panic!("Unknown attribute type: {}", parts[2]),
                };

                attributes.push(Attribute {
                    name,
                    attr_type: attribute_type,
                });
            } else if trimmed_line.starts_with("@data") {
                is_data_section = true;
            } else if is_data_section {
                let mut values = Vec::new();
                let mut in_quotes = false;
                let mut current_value = String::new();
                let mut prev_ch = ' ';

                for ch in trimmed_line.chars() {
                    if ch == '\'' && prev_ch != '\\' {
                        in_quotes = !in_quotes;
                    } else if ch == ',' && !in_quotes {
                        values.push(current_value.trim().to_string());
                        current_value.clear();
                    } else {
                        current_value.push(ch);
                    }

                    prev_ch = ch;
                }

                if !current_value.is_empty() {
                    values.push(current_value.trim().to_string());
                }

                assert_eq!(
                    values.len(),
                    attributes.len(),
                    "data length does not match attributes length"
                );

                data.push(DataRecord { values });
            }
        }

        ARFFData {
            _relation: relation,
            attributes,
            data,
        }
    }

    pub fn to_csv_normalized(&self, file_path: &str) {
        let mut writer = Writer::from_path(file_path).unwrap();

        let headers: Vec<String> = self
            .attributes
            .iter()
            .filter_map(|attr| match &attr.attr_type {
                AttributeType::String => None,
                AttributeType::Nominal(values) => {
                    if attr.name == "source" {
                        Some(vec![attr.name.clone()])
                    } else {
                        Some(
                            values
                                .iter()
                                .map(|v| format!("{}_{}", attr.name, v))
                                .collect::<Vec<_>>(),
                        )
                    }
                }
                AttributeType::BooleanNominal => Some(vec![attr.name.clone()]),
                _ => Some(vec![attr.name.clone()]),
            })
            .flatten()
            .collect();

        writer.write_record(&headers).unwrap();

        let mut means = HashMap::new();
        let mut standard_deviations = HashMap::new();
        let mut modes = HashMap::new();

        for (index, attribute) in self.attributes.iter().enumerate() {
            match &attribute.attr_type {
                AttributeType::Numeric | AttributeType::Date => {
                    let mut sum = 0.0;
                    let mut count = 0;
                    let mut values = Vec::new();

                    for record in &self.data {
                        let value = &record.values[index];

                        if value != "?" {
                            let num = value.parse::<f64>().unwrap();

                            values.push(num);
                            sum += num;
                            count += 1;
                        }
                    }

                    let mean = sum / count as f64;
                    means.insert(index, mean);

                    let standard_deviation =
                        (values.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / count as f64)
                            .sqrt();
                    standard_deviations.insert(index, standard_deviation);
                }
                AttributeType::Nominal(_) => {
                    let mut frequency = HashMap::new();

                    for record in &self.data {
                        let value = &record.values[index];

                        if value != "?" {
                            *frequency.entry(value.clone()).or_insert(0) += 1;
                        }
                    }

                    let mode = frequency
                        .into_iter()
                        .max_by_key(|&(_, count)| count)
                        .unwrap()
                        .0;

                    modes.insert(index, mode);
                }
                AttributeType::BooleanNominal => {
                    let mut count = 0;
                    let mut sum = 0;

                    for record in &self.data {
                        let value = &record.values[index];

                        sum += if value == "true" { 1 } else { 0 };
                        count += 1;
                    }

                    let mean = sum as f64 / count as f64;
                    means.insert(index, mean);

                    standard_deviations.insert(index, 0.0);
                }
                AttributeType::String => {}
            }
        }

        for record in &self.data {
            let mut normalized_record = Vec::new();

            for (index, value) in record.values.iter().enumerate() {
                let attribute = &self.attributes[index];

                match &attribute.attr_type {
                    AttributeType::Numeric | AttributeType::Date => {
                        let value = if value == "?" {
                            *means.get(&index).unwrap()
                        } else {
                            value.parse::<f64>().unwrap()
                        };

                        let mean = *means.get(&index).unwrap();
                        let std_dev = *standard_deviations.get(&index).unwrap();

                        let normalized_value = if std_dev > 0.0 {
                            (value - mean) / std_dev
                        } else {
                            0.0
                        };

                        normalized_record.push(normalized_value.to_string());
                    }
                    AttributeType::Nominal(nominal_values) => {
                        if attribute.name == "source" {
                            normalized_record.push(value.clone());
                        } else {
                            let mode = modes.get(&index).unwrap();
                            let actual_value = if value == "?" { mode } else { value };

                            for nominal_value in nominal_values {
                                if nominal_value == actual_value {
                                    normalized_record.push("1".to_string());
                                } else {
                                    normalized_record.push("0".to_string());
                                }
                            }
                        }
                    }
                    AttributeType::BooleanNominal => {
                        let normalized_value = if value == "true" { 1.0 } else { 0.0 };
                        normalized_record.push(normalized_value.to_string());
                    }
                    AttributeType::String => {}
                }
            }

            writer.write_record(&normalized_record).unwrap();
        }

        writer.flush().unwrap();
    }
}
