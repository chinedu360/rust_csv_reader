use std::error::Error;
use csv;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>>{
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.records() {
        let record = result?;

        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(e) = read_from_file("./customers.csv"){
        eprintln!("{}", e)
    }
}


//Got an arror while reading the sample csv

// StringRecord(["2", "1.7 Cubic Foot Compact \"Cube\" Office Refrigerators", "Barry French", "293", "457.81", "208.16", "68.02", "Nunavut", "Appliances", "0.58"])
// CSV parse error: record 2 (line 2, field: 1, byte: 245): invalid utf-8: invalid UTF-8 in field 1 near byte index 16

//would be fixed in the coming days!