use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
struct Student {
    name: String,
    email: String,
    phone: String,
    city: String,
    address: String,
    marks: Vec<u16>,
    percentage: Option<f32>,
    grade: Option<char>,
}
impl Student {
    fn calculate_percentage(&mut self) {
        let mut total_marks = 0.0;
        for marks in &self.marks {
            total_marks += *marks as f32;
        }
       self.percentage = Some(total_marks / self.marks.len() as f32);
    }
    fn calculate_grade(&mut self) {
       let grade =  match self.percentage.unwrap() {
            p if p >= 90.0=> 'A',
            p if p >= 80.0 =>'B',
            p if p >= 70.0 =>'C',
            p if p >= 60.0 =>'D',
            _ =>'F',
        };
        self.grade = Some(grade);

        
    }
}

fn main() {
    let path = "D:/Work/Rust/students/src/StudentData.json";
    let file_data = fs::read_to_string(path);

    match file_data {
        Ok(data) => {
            let mut struct_data: Vec<Student> = serde_json::from_str(&data).unwrap();
            for student_data in &mut struct_data {
                student_data.calculate_percentage();
                student_data.calculate_grade();
            }
            let updated_json =
                serde_json::to_string_pretty(&struct_data).expect("Failed to serialize data");
            fs::write("./src/updated_data.json", &updated_json)
                .expect("failed to write file");
        }
        Err(_) => {
            println!("failed to read");
        }
    }
}
