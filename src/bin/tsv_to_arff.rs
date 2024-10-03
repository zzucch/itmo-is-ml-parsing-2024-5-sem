use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Write},
};

use anyhow::Result;

fn main() -> Result<()> {
    const INPUT_PATH: &str = "./data/data.tsv";
    const OUTPUT_PATH: &str = "./data/data.arff";

    let input_file = File::open(INPUT_PATH)?;
    let reader = BufReader::new(input_file);

    let mut headers = Vec::new();
    let mut nominal_values: Vec<HashSet<String>> = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        let values: Vec<&str> = line.trim().split('\t').collect();

        if index == 0 {
            headers = values.iter().map(|&s| s.to_string()).collect();
            nominal_values = vec![HashSet::new(); headers.len()];
        } else {
            for (i, value) in values.iter().enumerate() {
                if headers[i].starts_with("is_")
                    || headers[i] == "format"
                    || headers[i] == "status"
                    || headers[i] == "source"
                    || headers[i].starts_with("company_")
                {
                    let value_to_insert = if headers[i].starts_with("company_") && value == &"0" {
                        "?".to_string()
                    } else if *value != "?" {
                        value.to_string()
                    } else {
                        continue;
                    };

                    nominal_values[i].insert(value_to_insert);
                }
            }
        }
    }

    let input_file = File::open(INPUT_PATH)?;
    let reader = BufReader::new(input_file);
    let mut output_file = File::create(OUTPUT_PATH)?;

    writeln!(output_file, "@relation data\n")?;

    for (index, header) in headers.iter().enumerate() {
        let attr_type = if header.starts_with("is_")
            || header == "format"
            || header == "status"
            || header == "source"
            || header.starts_with("company_")
        {
            let mut values: Vec<String> = nominal_values[index].iter().cloned().collect();
            if headers[index].starts_with("company_") {
                values.sort_by(|a, b| {
                    a.parse::<i32>()
                        .unwrap_or(0)
                        .cmp(&b.parse::<i32>().unwrap_or(0))
                });
            } else {
                values.sort();
            }
            format!(
                "{{{}}}",
                values
                    .iter()
                    .map(|v| quote_if_needed(v))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        } else if header.starts_with("name_") {
            "string".to_string()
        } else if header.ends_with("date") {
            "date 'S'".to_string()
        } else {
            "numeric".to_string()
        };
        writeln!(output_file, "@attribute {} {}", header, attr_type)?;
    }

    writeln!(output_file, "\n@data")?;

    for line in reader.lines().skip(1) {
        let line = line?;

        let values: Vec<&str> = line.trim().split('\t').collect();
        let quoted_values: Vec<String> = values.iter().map(|&v| quote_if_needed(v)).collect();

        writeln!(output_file, "{}", quoted_values.join(", "))?;
    }

    Ok(())
}

fn quote_if_needed(value: &str) -> String {
    if value.contains(' ') || value.contains(',') || value.contains('\'') {
        format!("'{}'", value.replace('\'', "''"))
    } else {
        value.to_string()
    }
}
