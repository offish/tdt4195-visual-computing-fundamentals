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
        let mut temp_x_vertices: Vec<f32> = Vec::new();
        let mut temp_y_vertices: Vec<f32> = Vec::new();
        let mut temp_z_vertices: Vec<f32> = Vec::new();
        let mut max_x_value: f32 = 0.0;
        let mut max_y_value: f32 = 0.0;
        let mut max_value: f32 = 0.0;

        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split_whitespace().collect();

            // vertex coordinates
            if line.starts_with("v ") {
                let x: f32 = parts[1].parse().unwrap();
                let y: f32 = parts[2].parse().unwrap();
                let z: f32 = parts[3].parse().unwrap();

                if x.abs() > max_x_value {
                    max_x_value = x.abs();
                }

                if y.abs() > max_y_value {
                    max_y_value = y.abs();
                }

                temp_x_vertices.push(x);
                temp_y_vertices.push(y);
                temp_z_vertices.push(z);
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

        if max_x_value > max_y_value {
            max_value = max_x_value;
        } else {
            max_value = max_y_value;
        }

        // scale down to fit the window
        for i in 0..temp_x_vertices.len() {
            // self.vertices.push(temp_x_vertices[i]);
            // self.vertices.push(temp_y_vertices[i]);
            self.vertices.push(temp_x_vertices[i] / max_value);
            self.vertices.push(temp_y_vertices[i] / max_value);
            // self.vertices.push(temp_z_vertices[i]);
            self.vertices.push(0.0);
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
