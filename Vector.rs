struct Student {
    name: String,
    email: String,
    phone: String,
    id: u32,
}

fn main() {
    let mut students: Vec<Student> = Vec::new();

    // Adding 5 student details to the vector
    students.push(Student {
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
        phone: String::from("123-456-7890"),
        id: 1,
    });

    students.push(Student {
        name: String::from("Bob"),
        email: String::from("bob@example.com"),
        phone: String::from("987-654-3210"),
        id: 2,
    });

    students.push(Student {
        name: String::from("Charlie"),
        email: String::from("charlie@example.com"),
        phone: String::from("555-555-5555"),
        id: 3,
    });

    students.push(Student {
        name: String::from("David"),
        email: String::from("david@example.com"),
        phone: String::from("111-222-3333"),
        id: 4,
    });

    students.push(Student {
        name: String::from("Eve"),
        email: String::from("eve@example.com"),
        phone: String::from("999-888-7777"),
        id: 5,
    });

    // Accessing student details with error handling
    let index = 3; // Change this to access different students
    match students.get(index) {
        Some(student) => {
            println!("Student ID: {}", student.id);
            println!("Name: {}", student.name);
            println!("Email: {}", student.email);
            println!("Phone: {}", student.phone);
        }
        None => {
            println!("Student not found at index {}", index);
        }
    }
}
