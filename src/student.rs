use crate::course::Course;
use crate::main;
// Students is composed of Student, a struct that contains a few things.
// 0. name
// 1. His date of birth in a unsigned 32 bit integer
// 2. Float that contains his average total grade
// 3. A type Courses that contains his courses
pub struct Student {
    name: &'static str,
    birth: u32,
    average: f32,
    courses: Vec<Course>
}

impl New for Student {
    fn new(name: &str, birthday: u32) -> Student {
        Student(name, birthday, 0, Vec::new())
    }
}

impl CourseSelection for Student {
    fn enroll(&mut self, code: String) {
        match COURSE_CATALOG.get(code) {
            Some(Course) => self.courses.push(Course),
            None => Err("Course not available")
        }
    }

    fn drop(&self, code: String) {
       if COURSE_CATALOG.contains_key(code) {
           self.courses.remove()
           println!((format!("Removed {code} from {self}'s courses")));
       }
    }
}



// Courses is an alias for an array of Course which contains
// 1. Course code, must be 4 letters and 4 numbers
// 2. Float that contains average grade
// 3. Grades, an alias for an array of unsigned 8 bit integers no greater than 100


pub trait New {
    fn new(name: &str, birthday: u32) -> Result<Student, std::io::Error>;
}

pub trait CourseSelection {
    fn enroll(&self, code: String) -> Result<bool, std::io::Error>;
    fn drop(&self, code: String) -> Result<bool, std::io::Error>;
}

pub trait Graded {
    fn add_grade(&self, grade:u8) -> Result<u8, std::io::Error>;
    fn remove_grade(&self, grade:u8) -> Result<u8, std::io::Error>;
}

pub trait ToString {
    fn to_string(&self) -> String;
}

