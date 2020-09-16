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
    let maths = Course::new("Maths", 1, 5);
    let science = Course::new("Science", 1, 2);

    let teacher_submissions = [
        TeacherSubmission::new(Teacher::new("Teacher 1"), vec![maths.clone()]),
        TeacherSubmission::new(Teacher::new("Teacher 2"), vec![science.clone()]),
    ];

    let student_submissions = [
        StudentSubmission::new(Student::new("Student 1"), vec![maths.clone()]),
        StudentSubmission::new(Student::new("Student 2"), vec![science.clone()]),
        StudentSubmission::new(
            Student::new("Student 3"),
            vec![maths.clone(), science.clone()],
        ),
        StudentSubmission::new(
            Student::new("Student 4"),
            vec![science.clone(), maths.clone()],
        ),
    ];

    let courses = [maths, science];

    let mut classes = Vec::new();

    for course in &courses {
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

    dbg!(classes);
}
