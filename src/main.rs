use timetable::{form_classes, Course, Student, Teacher};

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

fn main() {
    let maths = Course::new("Maths", 5, 25);
    let english = Course::new("English", 10, 30);
    let history = Course::new("History", 5, 30);
    let science = Course::new("Science", 10, 25);

    let teacher_submissions = [
        TeacherSubmission::new(Teacher::new("Teacher 1"), vec![maths.clone()]),
        TeacherSubmission::new(Teacher::new("Teacher 2"), vec![english.clone()]),
    ];

    let student_submissions = [
        StudentSubmission::new(
            Student::new("Student 1"),
            vec![maths.clone(), english.clone()],
        ),
        StudentSubmission::new(
            Student::new("Student 2"),
            vec![english.clone(), history.clone()],
        ),
        StudentSubmission::new(
            Student::new("Student 3"),
            vec![history.clone(), science.clone()],
        ),
        StudentSubmission::new(
            Student::new("Student 4"),
            vec![science.clone(), maths.clone()],
        ),
    ];

    let courses = [maths, english, history, science];

    let mut classes = Vec::new();

    for course in &courses {
        dbg!(&course);

        let teachers: Vec<Teacher> = teacher_submissions
            .iter()
            .filter(|submission| submission.courses.contains(&course))
            .map(|submission| submission.teacher.clone())
            .collect();

        let students: Vec<Student> = student_submissions
            .iter()
            .filter(|submission| submission.course_prefs.contains(&course))
            .map(|submission| submission.student.clone())
            .collect();

        for class in form_classes(course.clone(), teachers, students) {
            classes.push(class);
        }
    }

    for class in classes {
        dbg!(class);
    }
}
