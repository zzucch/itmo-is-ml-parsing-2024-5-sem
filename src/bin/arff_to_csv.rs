use std::{fs, path::Path};

use ml_parser::parse::arff::ARFFData;

fn main() {
    let input_path = "./data/data.arff";
    let output_path = "./data/data.csv";

    if !Path::new(input_path).exists() {
        eprintln!("Error: Input file {} not found.", input_path);
        return;
    }

    let content = fs::read_to_string(input_path).expect("Failed to read the ARFF file");

    let arff_data = ARFFData::from_arff(&content);

    arff_data.to_csv_normalized(output_path);

    println!(
        "Normalization complete. CSV file written to {}",
        output_path
    );
}
