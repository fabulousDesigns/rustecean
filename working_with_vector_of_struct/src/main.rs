// Exercise 3: Implement operations on a vector of custom structs
#[derive(Debug)]
struct Student {
    name: String,
    grade: u32,
    active: bool,
}

fn main() {
    // Create a vector of Student structs
    let mut students = vec![
        Student { name: String::from("Alice"), grade: 88, active: true },
        Student { name: String::from("Bob"), grade: 72, active: true },
        Student { name: String::from("Charlie"), grade: 95, active: false },
        Student { name: String::from("Dave"), grade: 65, active: true },
        Student { name: String::from("Eve"), grade: 91, active: true },
    ];

    // 1. Print all student names
    for student in &students {
        println!("The student is {}", student.name);
    }
    // 2. Calculate and print the average grade of active students
    let mut average_grade = 0;
    for student in &students {
        if student.active {
            average_grade += student.grade;
        }
    }
    println!("The average grade is {}", average_grade / students.len() as u32);
    // 3. Find and print the name of the student with the highest grade
    for student in &students {
        
    }
    // 4. Add a new student to the vector
    students.push(Student{name: "Bob".to_string(), grade: 88, active: true});

    // 5. Remove all inactive students from the vector

    // 6. Update all grades below 70 to exactly 70

    // 7. Print the final list of students
}