#![warn(missing_debug_implementations, rust_2018_idioms)]

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Student {
    name: String,
}

impl Student {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Teacher {
    name: String,
}

impl Teacher {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Course {
    name: String,
    min_students: usize,
    max_students: usize,
}

impl Course {
    pub fn new(name: impl Into<String>, min_students: usize, max_students: usize) -> Self {
        Self {
            name: name.into(),
            min_students,
            max_students,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Class {
    course: Course,
    teacher: Teacher,
    students: Vec<Student>,
}

impl Class {
    fn new(course: Course, teacher: Teacher, students: Vec<Student>) -> Self {
        Self {
            course,
            teacher,
            students,
        }
    }
}

pub fn form_classes(
    course: Course,
    mut teachers: Vec<Teacher>,
    mut students: Vec<Student>,
) -> Vec<Class> {
    // If there are no students or no teachers, there can't possibly be any classes
    if teachers.is_empty() || students.is_empty() {
        return Vec::new();
    }

    let mut classes = Vec::with_capacity(teachers.len());

    {
        let mut students = students.clone();

        let base_num_of_students_per_class = students.len() / teachers.len();
        let mut num_leftover_students = students.len() % teachers.len();

        for teacher in &teachers {
            let mut num_students_in_this_class = base_num_of_students_per_class;

            // If there are leftover students, take one of them and add them to the current class
            if num_leftover_students > 0 {
                num_leftover_students -= 1;
                num_students_in_this_class += 1;
            }

            classes.push(Class::new(
                course.clone(),
                teacher.clone(),
                students.drain(0..num_students_in_this_class).collect(),
            ));
        }
    }

    for class in &classes {
        if class.students.len() < course.min_students {
            // If there is an excess of teachers, remove one teacher and try forming classes again
            teachers.pop();
            return form_classes(course, teachers, students);
        } else if class.students.len() > course.max_students {
            // If there is instead an excess of teachers, remove one student and try forming classes again
            students.pop();
            return form_classes(course, teachers, students);
        }
    }

    classes
}

#[cfg(test)]
mod class_formation_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn no_teachers_no_classes() {
        assert_eq!(
            form_classes(
                Course::new("English", 2, 5),
                Vec::new(),
                vec![Student::new("Student 1")]
            ),
            Vec::new()
        );
    }

    #[test]
    fn no_students_no_classes() {
        assert_eq!(
            form_classes(
                Course::new("English", 2, 5),
                vec![Teacher::new("Teacher 1")],
                Vec::new()
            ),
            Vec::new()
        );
    }

    #[test]
    fn students_divide_evenly() {
        let english = Course::new("English", 2, 5);

        assert_eq!(
            form_classes(
                english.clone(),
                vec![Teacher::new("Teacher 1"), Teacher::new("Teacher 2")],
                vec![
                    Student::new("Student 1"),
                    Student::new("Student 2"),
                    Student::new("Student 3"),
                    Student::new("Student 4")
                ]
            ),
            vec![
                Class::new(
                    english.clone(),
                    Teacher::new("Teacher 1"),
                    vec![Student::new("Student 1"), Student::new("Student 2")]
                ),
                Class::new(
                    english,
                    Teacher::new("Teacher 2"),
                    vec![Student::new("Student 3"), Student::new("Student 4")]
                )
            ]
        )
    }

    #[test]
    fn students_divide_unevenly() {
        let english = Course::new("English", 1, 5);

        assert_eq!(
            form_classes(
                english.clone(),
                vec![Teacher::new("Teacher 1"), Teacher::new("Teacher 2")],
                vec![
                    Student::new("Student 1"),
                    Student::new("Student 2"),
                    Student::new("Student 3"),
                ]
            ),
            vec![
                Class::new(
                    english.clone(),
                    Teacher::new("Teacher 1"),
                    vec![Student::new("Student 1"), Student::new("Student 2")]
                ),
                Class::new(
                    english,
                    Teacher::new("Teacher 2"),
                    vec![Student::new("Student 3")]
                )
            ]
        )
    }

    #[test]
    fn excess_of_teachers() {
        let english = Course::new("English", 2, 5);

        assert_eq!(
            form_classes(
                english.clone(),
                vec![Teacher::new("Teacher 1"), Teacher::new("Teacher 2")],
                vec![Student::new("Student 1"), Student::new("Student 2")]
            ),
            vec![Class::new(
                english,
                Teacher::new("Teacher 1"),
                vec![Student::new("Student 1"), Student::new("Student 2")]
            )]
        )
    }

    #[test]
    fn excess_of_students() {
        let english = Course::new("English", 1, 2);

        assert_eq!(
            form_classes(
                english.clone(),
                vec![Teacher::new("Teacher 1")],
                vec![
                    Student::new("Student 1"),
                    Student::new("Student 2"),
                    Student::new("Student 3")
                ]
            ),
            vec![Class::new(
                english,
                Teacher::new("Teacher 1"),
                vec![Student::new("Student 1"), Student::new("Student 2")]
            )]
        )
    }
}
