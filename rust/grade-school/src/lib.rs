use std::collections::HashMap;

pub struct School<'a> {
    grades : HashMap<u32, Vec<&'a str>>
}

impl<'a> School<'a> {
    pub fn new() -> School<'a> {
        School {
            grades: HashMap::new()
        }
    }

    pub fn add(&mut self, grade: u32, student: &'a str) {
        let mut students = match self.grades.get(&grade) {
                            Some(students) => students.clone(),
                            None => Vec::new(),
                        };

        students.push(student);

        self.grades.insert(grade, students); 
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades = self.grades.keys()
                                    .cloned()
                                    .collect::<Vec<u32>>();
        grades.sort();
        grades
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        let mut students_opt = self.grades.get(&grade)
                   .map_or(None, |students| 
                        Some(students.iter()
                                .map(|student| student.to_string())
                                .collect::<Vec<String>>()));

        if let Some(students) = students_opt.as_mut() {
            (*students).sort();
        }
        students_opt                
    }
}
