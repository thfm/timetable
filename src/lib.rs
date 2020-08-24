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

#[derive(Debug, Eq, PartialEq, Clone)]
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
    teachers: Vec<Teacher>,
    mut students: Vec<Student>,
) -> Vec<Class> {
    // Steps:
    // 1. If there are no students or no teachers, return no classes. ✓
    // 2. Divide the students evenly between each teacher. ✓
    // 3. If each class is within course boundaries, return the classes.
    // 4. If any class has less than the minimum amount of students for the course - an excess of
    //    teachers - then 'remove' one teacher and try forming classes again (until there are no
    //    more teachers left).
    // 5. If any class has more than the maximum amount of students for the course - an excess of
    //    students - then fill up each class to capacity and reject the remaining students.
    if teachers.is_empty() || students.is_empty() {
        return Vec::new();
    }

    let num_students = students.len();
    let num_teachers = teachers.len();
    let students_per_class = num_students / num_teachers;
    let mut remainder = num_students % num_teachers;

    let mut classes = Vec::new();

    for teacher in teachers {
        let students_in_this_class = if remainder > 0 {
            remainder -= 1;
            0..(students_per_class + 1)
        } else {
            0..students_per_class
        };

        classes.push(Class::new(
            course.clone(),
            teacher,
            students.drain(students_in_this_class).collect(),
        ));
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
        let english = Course::new("English", 2, 5);

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
}
