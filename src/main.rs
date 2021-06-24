use std::io;
use std::io::Read;
use std::fs::File;


// Global Constant
// const stdin: Stdin = std::io::stdin();
// Declare student struct
struct Student {
    name: String,
    id: String,
    itsc: String,
    school: String,
    year: i32,
    credit: i32,
    course_list: Vec<String>,
}
// Declare course struct
struct Course {
    code: String,
    name: String,
    credit: String,
}
// Store course info

static mut STUDENT_LIST: Vec<Student> = Vec::new();
static mut COURSE_LIST: Vec<Course> = Vec::new();

// Store Student info
fn main() {
    unsafe {
        STUDENT_LIST = read_student_info();
        COURSE_LIST = read_course_info();
    }
    loop {
        println!("Please select 1 function:
            0 - Quit
            1 - Print All Students
            2 - Print Selected Student
            3 - Print All Courses
            4 - Print Selected Course
            5 - Add A Course
            * - Quit"
        );
        
        unsafe {
            let user_char: char = read_user_input();
            match user_char {
                '0' => break,
                '1' => print_all_student_info(&STUDENT_LIST),
                '2' => print_student_info(&STUDENT_LIST),
                '3' => print_all_course_info(&COURSE_LIST),
                '4' => print_single_course_info(&COURSE_LIST),
                '5' => add_a_course(&STUDENT_LIST, &COURSE_LIST),
                '*' => break,
                _ => println!("Error"),
            }
            for item in &mut STUDENT_LIST {
                for another in &item.course_list {
                    println!("{}", another);
                }
            }
        }

        // let student_list = read_student_info();
        // let course_list = read_course_info();
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
        Err(_e) => { return '*' }
    }
}

fn read_student_info() -> Vec<Student> {
    let mut students: Vec<Student> = Vec::new();
    let mut file = match csv::Reader::from_path("data/Student.csv"){
        Ok(file) => file,
        Err(e) => panic!("Could not open Student.csv: {}", e.to_string()),
    };
    for line in file.records().into_iter() { // loop into all records
        let record = line.unwrap(); // take out Ok()
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
        let course_detail_a: Vec<&str> = line.split(" - ").collect();
        let course_detail_b: Vec<&str> = course_detail_a[1].split(" (").collect();
        let course_detail_c: Vec<&str> = course_detail_b[1].split(" units").collect();
        let course_code: &str = course_detail_a[0].trim();
        let course_name: &str = course_detail_b[0].trim();
        let course_credit = course_detail_c[0];

        let course: Course = {Course {
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
    let stdin = io::stdin();
    println!("Enter a student ID: ");
    stdin.read_line(&mut student_string).unwrap();
    for student in students {
        if student_string.trim() == student.id.trim() {
            println!("Name:   {}", student.name);
            println!("Id:     {}", student.id);
            println!("ITSC:   {}", student.itsc);
            println!("School: {}", student.school);
            println!("Year:   {}", student.year);
            println!("Credit: {}", student.credit);
            println!("Course: {:?}", student.course_list);
            // for detail in student.course_list {

            // }
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
        course_list: Vec::new(),
    }
}

// 3
fn print_all_course_info(course_list: &Vec<Course>) {
    println!("Printing all courses' detail");
    for course in course_list {
        println!("Code:    {}", course.code);
        println!("Name:    {}", course.name);
        println!("Credit:  {}", course.credit);
    }
}
// 4
fn print_single_course_info(course_list: &Vec<Course>) {
    let mut user_string: String = String::new();
    let stdin = io::stdin();
    println!("Enter a course code: ");
    stdin.read_line(&mut user_string).unwrap();
    for course in course_list {
        if user_string.trim() == course.code.trim() {
            println!("Code:    {}", course.code);
            println!("Name:    {}", course.name);
            println!("Credit:  {}", course.credit);
            break;
        }
    }
}

// 5
fn add_a_course(students: &Vec<Student>, courses: &Vec<Course>) {
    // assume all data is fine and correct first
    let mut student = add_a_course_step_1(students);
    let mut course = add_a_course_step_2(courses);
    add_a_course_step_3(&mut student, &mut course);
}

fn add_a_course_step_1(students: &Vec<Student>) -> &Student { // choose a student
    println!("/////////////////////////////////");
    println!("/////////////////////////////////");
    println!("/////////////Step 1//////////////");
    println!("/////////////////////////////////");
    println!("/////////////////////////////////");
    println!("");
    println!("Choose a STUDENT:
    (id): input id for selecting a student
    999: print all student information");
    let mut student_string: String = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut student_string).unwrap();
    match student_string.trim() {
        "999" => {
            print_all_student_info(students);
            return add_a_course_step_1(students);
        },
        _ => {
            for student in students {
                if student_string.trim() == student.id.trim() {
                    println!("=========STUDENT==========");
                    println!("Name:   {}", student.name);
                    println!("Id:     {}", student.id);
                    println!("ITSC:   {}", student.itsc);
                    println!("School: {}", student.school);
                    println!("Year:   {}", student.year);
                    println!("Credit: {}", student.credit);
                    println!("=========================");
                    return student;
                }
            }
            return &students[0];
        }
    }
}

fn add_a_course_step_2(courses: &Vec<Course>) -> &Course { // choose a course
    println!("/////////////////////////////////");
    println!("/////////////////////////////////");
    println!("/////////////Step 2//////////////");
    println!("/////////////////////////////////");
    println!("/////////////////////////////////");
    println!("");
    println!("Choose a Course:
    (code): input course code 
    999: print all course information");
    let mut course_string: String = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut course_string).unwrap();
    match course_string.trim() {
        "999" => {
            print_all_course_info(courses);
            return add_a_course_step_2(courses);
        },
        _ => {
            for course in courses {
                if course_string.trim() == course.code.trim() {
                    println!("=========COURSE==========");
                    println!("Code:    {}", course.code);
                    println!("Name:    {}", course.name);
                    println!("Credit:  {}", course.credit);
                    println!("=========================");
                    return course;
                }
            }
            return &courses[0];
        }
    }
}

fn add_a_course_step_3(student: &Student, course: &Course) { // append course code to student
    println!("/////////////////////////////////");
    println!("/////////////////////////////////");
    println!("/////////////Step 3//////////////");
    println!("/////////////////////////////////");
    println!("/////////////////////////////////");
    println!("");
    println!("Adding {} to Student {} ({})", course.code, student.name, student.id);
    let course_code = &course.code;
    unsafe {
        for (pos, e) in &mut STUDENT_LIST.iter().enumerate() {
            // println!("{}", student.credit.)
            let mut total_credit: i32 = student.credit + course.credit.to_string().parse::<i32>().unwrap();
            if e.id == student.id {
                if total_credit < 18 {
                    STUDENT_LIST[pos].course_list.push(course_code.to_string());
                    STUDENT_LIST[pos].credit = total_credit;
                }
            }
        }
    }
}
