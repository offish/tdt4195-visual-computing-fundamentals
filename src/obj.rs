use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct ObjParser {
    file_path: String,
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
}

impl ObjParser {
    pub fn new(file_path: &str) -> ObjParser {
        ObjParser {
            file_path: file_path.to_string(),
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        let path = Path::new(&self.file_path);
        let file = File::open(&path).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split_whitespace().collect();

            // vertex coordinates
            if line.starts_with("v ") {
                let x: f32 = parts[1].parse().unwrap();
                let y: f32 = parts[2].parse().unwrap();
                let z: f32 = parts[3].parse().unwrap();
                // scale down to fit in the window
                self.vertices.push(x / 2.0);
                self.vertices.push(y / 2.0);
                self.vertices.push(z / 2.0);
            }
            // face indices
            else if line.starts_with("f ") {
                for i in 1..4 {
                    let index_str = parts[i].split('/').next().unwrap();
                    let index: u32 = index_str.parse::<u32>().unwrap() - 1;
                    self.indices.push(index);
                }
            }
        }
    }

    // Method to get the vertices
    pub fn get_vertices(&self) -> Vec<f32> {
        self.vertices.clone()
    }

    // Method to get the indices
    pub fn get_indices(&self) -> Vec<u32> {
        self.indices.clone()
    }
}
