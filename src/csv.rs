use std::fs::{self, File};

// This module handles extracting the data from the CSV to JSON objects
use csv::Reader;

use crate::PatchData;
pub struct CsvReader {
    path: String,
    reader: Reader<File>,
}

impl CsvReader {
    pub fn new(path: String) -> CsvReader {
        CsvReader {
            reader: Reader::from_path(&path).expect("CSV file not found"),
            path,
        }
    }

    fn count_lines(&self) -> usize {
        let file = fs::read_to_string(&self.path).expect("Error reading CSV file");
        file.lines().count()
    }

    pub fn extract(&mut self) -> Result<Vec<PatchData>, Box<dyn std::error::Error>> {
        let mut data: Vec<PatchData> = Vec::with_capacity(self.count_lines());
        for result in self.reader.deserialize() {
            data.push(result?)
        }
        Ok(data)
    }
}
