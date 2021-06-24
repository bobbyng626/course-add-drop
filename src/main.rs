use std::io;
use std::io::Read;

use std::fs::File;

use std::error::Error;
use std::process;

// Global Constant
const MAX_COURSE: usize = 10;
// Declare student struct
struct Student {
    name: String,
    id: String,
    itsc: String,
    school: String,
    year: i32,
    credit: i32,
}
// Declare course struct
struct Course {
    code: String,
    name: String,
    credit: String,
}
// Store course info

// Store Student info
fn main() {
    loop {
        println!("Please select 1 function:
            0 - Quit
            1 - Print All Students
            2 - Print Selected Student
            3 - Print All Courses
            4 - Print Selected Course
            * - Quit"
        );
        let mut user_char: char = read_user_input();
        let mut student_list = read_student_info();
        let mut course_list = read_course_info();
        match user_char {
            '0' => break,
            '1' => print_all_student_info(&student_list),
            '2' => print_student_info(&student_list),
            '3' => print_all_course_info(&course_list),
            '4' => print_single_course_info(&course_list),
            '*' => break,
            _ => println!("Error"),
        }
    }
}

fn read_user_input()->char {
    let mut input = String::new();                  // init variable for storing input
    match io::stdin().read_line(&mut input) {       // io::stdin().read_line ===> read user input
        Ok(_) => {                                  // OK input => stor the single char to input
            match input.chars().next() {
                Some(c) => { return c; }
                None => { return '*'; }
            }
        }
        Err(e) => { return '*' }
    }
}

fn read_student_info() -> Vec<Student> {
    let mut students: Vec<Student> = Vec::new();
    let mut file = match csv::Reader::from_path("data/Student.csv"){
        Ok(file) => file,
        Err(e) => panic!("Could not open Student.csv: {}", e.to_string()),
    };
    for line in file.records().into_iter() { // loop into all records
        let mut record = line.unwrap(); // take out Ok()
        students.push(match_csv(&record));
    }
    return students;
}

fn read_course_info() -> Vec<Course> {
    let mut courses: Vec<Course> = Vec::new();
    let mut contents: String = String::new();

    let mut file = match File::open("data/text.txt"){
        Err(e) => panic!("Could not open text.txt: {}", e.to_string()),
        Ok(file) => file,
    };
    file.read_to_string(&mut contents)
        .expect("Cannot read the file");
    
    for line in contents.lines() {
        let mut course_detail_a: Vec<&str> = line.split(" - ").collect();
        let mut course_detail_b: Vec<&str> = course_detail_a[1].split(" (").collect();
        let mut course_detail_c: Vec<&str> = course_detail_b[1].split(" units").collect();
        let mut course_code: &str = course_detail_a[0].trim();
        let mut course_name: &str = course_detail_b[0].trim();
        let mut course_credit = course_detail_c[0];

        let mut course: Course = {Course {
            code: course_code.to_string(),
            name: course_name.to_string(),
            credit: course_credit.to_string(),
        }};
        courses.push(course);
    }

    return courses;
}

// 0

// 1

fn print_all_student_info(students: &Vec<Student>) {
    for student in students {
        println!("Name:   {}", student.name);
        println!("Id:     {}", student.id);
        println!("ITSC:   {}", student.itsc);
        println!("School: {}", student.school);
        println!("Year:   {}", student.year);
        println!("Credit: {}", student.credit);
        println!("=========================");
    }
}
// 2
fn print_student_info(students: &Vec<Student>) {
    let mut student_string: String = String::new();
    let mut stdin = io::stdin();
    println!("Enter a student ID: ");
    stdin.read_line(&mut student_string);
    for student in students {
        if student_string.trim() == student.id.trim() {
            println!("Name:   {}", student.name);
            println!("Id:     {}", student.id);
            println!("ITSC:   {}", student.itsc);
            println!("School: {}", student.school);
            println!("Year:   {}", student.year);
            println!("Credit: {}", student.credit);
            println!("=========================");
            break;
        }
    }
}

fn match_csv(row: &csv::StringRecord) -> Student{
    return Student {
        name: row[0].to_string(),
        id: row[1].to_string(),
        itsc: row[2].to_string(),
        school: row[3].to_string(),
        year: row[4].to_string().parse::<i32>().unwrap(),
        credit: row[5].to_string().parse::<i32>().unwrap(),
    }
}

// 3
fn print_all_course_info(course_list: &Vec<Course>) {
    println!("Printing all courses' detail");
    for course in course_list {
        println!("Code:    {}", course.code);
        println!("Name:    {}", course.name);
        println!("Credit:  {}", course.credit);
        // print_course_info(course_list, &course.code);
    }
}
// 4
fn print_single_course_info(course_list: &Vec<Course>) {
    let mut user_string: String = String::new();
    let mut stdin = io::stdin();
    println!("Enter a course code: ");
    stdin.read_line(&mut user_string);
    for course in course_list {
        if user_string.trim() == course.code.trim() {
            println!("Code:    {}", course.code);
            println!("Name:    {}", course.name);
            println!("Credit:  {}", course.credit);
            break;
        }
    }
}
 
fn print_course_info(course_list: &Vec<Course>, course_code: &String) {
    for course in course_list {
        if &course.code == course_code {
            println!("Code:    {}", course.code);
            println!("Name:    {}", course.name);
            println!("Credit:  {}", course.credit);
            break;
        }
    }
}