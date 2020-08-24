use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Student {
    name: String,
}

impl Student {
    fn new(name: impl Into<String>) -> Student {
        Student { name: name.into() }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Teacher {
    name: String,
}

impl Teacher {
    fn new(name: impl Into<String>) -> Teacher {
        Teacher { name: name.into() }
    }
}

#[derive(EnumIter, Debug, Eq, PartialEq)]
enum Course {
    Maths,
    Science,
    English,
    History,
    Geography,
}

struct StudentSubmission {
    student: Student,
    course_prefs: Vec<Course>,
}

impl StudentSubmission {
    fn new(student: Student, course_prefs: Vec<Course>) -> Self {
        Self {
            student,
            course_prefs,
        }
    }
}

struct TeacherSubmission {
    teacher: Teacher,
    courses: Vec<Course>,
}

impl TeacherSubmission {
    fn new(teacher: Teacher, courses: Vec<Course>) -> Self {
        Self { teacher, courses }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Class {
    course: Course,
    teacher: Teacher,
    students: Vec<Student>,
}

fn form_classes(course: Course, teachers: Vec<Teacher>, students: Vec<Student>) -> Vec<Class> {
    vec![]
}

fn main() {
    let teacher_submissions = vec![
        TeacherSubmission::new(Teacher::new("Teacher 1"), vec![Course::Maths]),
        TeacherSubmission::new(
            Teacher::new("Teacher 2"),
            vec![Course::History, Course::Geography],
        ),
    ];

    let student_submissions = vec![
        StudentSubmission::new(
            Student::new("Student 1"),
            vec![Course::Maths, Course::Science],
        ),
        StudentSubmission::new(
            Student::new("Student 2"),
            vec![Course::Maths, Course::English],
        ),
        StudentSubmission::new(
            Student::new("Student 3"),
            vec![Course::English, Course::Science],
        ),
        StudentSubmission::new(
            Student::new("Student 4"),
            vec![Course::History, Course::Geography],
        ),
    ];

    let mut classes: Vec<Class> = Vec::new();

    for course in Course::iter() {
        dbg!(&course);

        let mut teachers: Vec<Teacher> = Vec::new();
        let mut students: Vec<Student> = Vec::new();

        for submission in &teacher_submissions {
            if submission.courses.contains(&course) {
                teachers.push(submission.teacher.clone());
            }
        }

        for submission in &student_submissions {
            if submission.course_prefs.contains(&course) {
                students.push(submission.student.clone());
            }
        }

        for class in form_classes(course, teachers, students) {
            classes.push(class);
        }
    }

    for class in classes {
        dbg!(class);
    }
}

#[cfg(test)]
mod class_formation_tests {
    use super::*;

    #[test]
    fn one_teacher_one_class() {
        assert_eq!(
            form_classes(
                Course::English,
                vec![Teacher::new("Teacher 1")],
                vec![
                    Student::new("Student 1"),
                    Student::new("Student 2"),
                    Student::new("Student 3")
                ]
            ),
            vec![Class {
                course: Course::English,
                teacher: Teacher::new("Teacher 1"),
                students: vec![
                    Student::new("Student 1"),
                    Student::new("Student 2"),
                    Student::new("Student 3")
                ]
            }]
        )
    }

    // The following tests will have to take into account class limits on numbers of students.
    #[test]
    fn two_teachers_one_class() {}

    #[test]
    fn two_teachers_two_classes() {}
}
