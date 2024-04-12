use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Error {
    Invalid_Organization,
    Invalid_Lesson
}

#[derive(Clone, Debug, PartialEq)]
enum College {
    Computer,
    Information,
    Science,
    Art,
    Physical,
    Culture
}


#[derive(Clone, Debug, PartialEq)]
struct Organization {
    name: String,
    description: String,
    creator: Student
}

impl Organization {

    fn new(name: String, description: String, creator: Student) -> Self {
        Organization {
            name, description, creator
        }
    }

    fn add(name:String, description: String, creator: Student, organization_arr: &[Organization]) -> Vec<Organization> {
        let mut Organization_vec = organization_arr.to_vec();
        let add_organization = Organization::new(name, description, creator);
        Organization_vec.push(add_organization);
        Organization_vec
    }

    fn remove(name:String, organization_arr: &[Organization]) -> Vec<Organization> {
        let mut Organization_vec: Vec<Organization> = Vec::new();
        for Organization in organization_arr {
            if Organization.name == name {
                continue;
            }
            Organization_vec.push(Organization.clone());
        }
        Organization_vec
    }

    fn update(name:String, description: String, creator: Student, organization_arr: &[Organization]) -> Vec<Organization> {
        let mut organization_vec: Vec<Organization> = Vec::new();
        for Organization in organization_arr {
            if Organization.name == name {
                let update_organization = Organization::new(name.clone(), description.clone(), creator.clone());
                organization_vec.push(update_organization);
                continue;
            }
            organization_vec.push(Organization.clone());

        }
        organization_vec
    }

    fn query(name: String, organization_arr: &[Organization]) -> Option<Organization> {
        let mut query_organization: Option<Organization> = None;
        for Organization in organization_arr {
            if Organization.name == name {
                query_organization = Some(Organization.clone());
                break;
            }
        }
        query_organization
    }

    fn participate(student: Student, organization_name: String, mut organization_map: HashMap<&str, Vec<Student>>) -> Result<HashMap<&str, Vec<Student>>, Error> {
        match organization_map.get_mut(organization_name.as_str()) {
            Some(organization_vec) => {
                organization_vec.push(student);
                return  Ok(organization_map);
            },
            None => {
                return Err(Error::Invalid_Organization);
            }
        }
    }

    fn quit(student_name: String, organization_name: String, mut organization_map: HashMap<&str, Vec<Student>>) -> Result<HashMap<&str, Vec<Student>>, Error> {
        match organization_map.get_mut(organization_name.as_str().clone()) {
            Some(organization_vec) => {
                let mut new_organization_vec: Vec<Student> = Vec::new();
                for student in &mut * organization_vec {
                    if student.name == student_name {
                        continue;
                    }
                    new_organization_vec.push(student.clone());
                };
                *organization_vec = new_organization_vec;
                return Ok(organization_map);
            },
            None => {
                return Err(Error::Invalid_Organization);
            }
        }
    }

}


#[derive(Clone)]
struct Lesson {
    name: String,
    teacher: String,
    credit: u64
}

impl Lesson {

    fn new(name: String, teacher: String, credit: u64) -> Self {
        Lesson {
            name,
            teacher,
            credit
        }
    }

    fn add(name: String, teacher: String, credit: u64, lesson_arr: &[Lesson]) -> Vec<Lesson> {
        let add_lesson = Lesson::new(name, teacher, credit);
        let mut lesson_vec = lesson_arr.to_vec();
        lesson_vec.push(add_lesson);
        lesson_vec
    }

    fn remove(name: String, lesson_arr: &[Lesson]) -> Vec<Lesson> {
        let mut lesson_vec: Vec<Lesson> = Vec::new();
        for lesson in lesson_arr {
            if lesson.name == name {
                continue;
            };
            lesson_vec.push(lesson.clone());
        }
        lesson_vec
    }

    fn update(name: String, teacher: String, credit: u64, lesson_arr: &[Lesson]) -> Vec<Lesson> {
        let mut lesson_vec: Vec<Lesson> = Vec::new();
        for lesson in lesson_arr {
            if lesson.name == name {
                let update_lesson = Lesson::new(name.clone(), teacher.clone(), credit);
                lesson_vec.push(update_lesson);
                continue;
            }
            lesson_vec.push(lesson.clone());
        }
        lesson_vec
    }

    fn query(name: String, lesson_arr: &[Lesson]) -> Option<Lesson> {
        for lesson in lesson_arr {
            if lesson.name == name {
                return Some(lesson.clone());
            }
        }
        None
    }

    fn participate(student: Student, lesson_name: String, mut lesson_map: HashMap<&str, Vec<Student>>) -> Result<HashMap<&str, Vec<Student>>, Error> {
        match lesson_map.get_mut(lesson_name.as_str()) {
            Some(lesson_vec) => {
                lesson_vec.push(student);
                return Ok(lesson_map);
            },
            None => {
                return Err(Error::Invalid_Lesson)
            }
        }
    }

    fn quit(student_name: String, lesson_name: String, mut lesson_map: HashMap<&str, Vec<Student>>) -> Result<HashMap<&str, Vec<Student>>, Error> {
        match lesson_map.get_mut(lesson_name.as_str().clone()) {
            Some(lesson_vec) => {
                let mut new_lesson_vec: Vec<Student> = Vec::new();
                for student in &mut *lesson_vec {
                    if student.name == student_name {
                        continue;
                    }
                    new_lesson_vec.push(student.clone());
                };
                *lesson_vec = new_lesson_vec;
                return Ok(lesson_map);
            },
            None => {
                return Err(Error::Invalid_Lesson)
            }
        }
    }

}

#[derive(Clone, Debug, PartialEq)]
struct Student {
    name: String,
    age: u64,
    student_number: u64,
    college: College,
    class_number: u64
}

impl Student {
    fn new(name: String, age: u64, student_number: u64, college: College, class_number: u64) -> Self {
        Student {
            name,
            age,
            student_number,
            college,
            class_number
        }
    }

    fn add(name: String, age:u64, student_number: u64, college: College, class_number: u64, student_arr: &[Student]) -> Vec<Student> {
        let add_student = Student::new(name, age, student_number, college, class_number);
        let mut student_vec = student_arr.to_vec();
        student_vec.push(add_student);
        student_vec
    }

    fn remove(name: String, student_arr: &[Student]) -> Vec<Student> {
        let mut student_vec: Vec<Student> = Vec::new();
        for student in student_arr {
            if student.name == name {
                continue;
            };
            student_vec.push(student.clone());
        }
        student_vec
    }
    fn update(name: String, age:u64, student_number: u64, college: College, class_number: u64, student_arr: &[Student]) -> Vec<Student> {
        let mut student_vec: Vec<Student> = Vec::new();
        for student in student_arr{
            if student.name == name {
                let update_student = Student::new(name.clone(), age, student_number, college.clone(), class_number);
                student_vec.push(update_student);
                continue;;
            };
            student_vec.push(student.clone());
        }
        student_vec
    }
    fn query(name: String, student_arr: &[Student]) -> Option<Student> {
        let mut query_student: Option<Student> = None;
        for student in student_arr {
            if student.name == name {
                query_student = Some(student.clone());
                break;
            }
        }
        query_student
    }
}


fn main() {
    let mut students: Vec<Student> = Vec::new();

    let mut organizations: Vec<Organization> = Vec::new();
    let mut organization_map: HashMap<&str, Vec<Student>> = HashMap::new();

    let mut lessons: Vec<Lesson> = Vec::new();
    let mut lesson_map: HashMap<&str, Vec<Student>> = HashMap::new();

    // add student
    students = Student::add("Bob".to_string(), 18u64, 20236868u64, College::Computer, 2301u64, &students);
    assert_eq!(students.len(), 1);

    // update student
    students = Student::update("Bob".to_string(), 18u64, 20236868u64, College::Art, 2304u64, &students);
    assert_eq!(students[0], Student::new("Bob".to_string(), 18u64, 20236868u64, College::Art, 2304u64));

    // query student
    assert_eq!(
        Student::query("Bob".to_string(), &students),
        Some(Student::new("Bob".to_string(), 18u64, 20236868u64, College::Art, 2304u64))
    );
    assert_eq!(
        Student::query("Alice".to_string(), &students),
        None
    );
    // delete student
    students = Student::add("Alice".to_string(), 20u64, 20237272u64, College::Science, 2302u64, &students);
    assert_eq!(students.len(), 2);
    students = Student::remove("Alice".to_string(), &students);
    assert_eq!(students.len(), 1);


    // add association
    organizations = Organization::add(
        "BlockChain organization".to_string(),
        "Research BlockChain".to_string(),
        Student::query("Bob".to_string(), &students).unwrap(),
        &organizations
    );
    assert_eq!(organizations.len(), 1);
    organization_map.insert("BlockChain organization", Vec::new());
    assert_eq!(
        organization_map.contains_key("BlockChain organization"),
        true
    );

    // update association
    organizations = Organization::update(
        "BlockChain organization".to_string(),
        "Research BlockChain technology and industry".to_string(),
        Student::query("Bob".to_string(), &students).unwrap(),
        &organizations
    );
    assert_eq!(
        organizations[0],
        Organization::new(
            "BlockChain organization".to_string(),
            "Research BlockChain technology and industry".to_string(),
            Student::query("Bob".to_string(), &students).unwrap()        
        )
    );
    // query association
    assert_eq!(
        Organization::query(
            "BlockChain organization".to_string(),
            &organizations
        ),
        Some(Organization::new(
            "BlockChain organization".to_string(),
            "Research BlockChain technology and industry".to_string(),
            Student::query("Bob".to_string(), &students).unwrap()
        ))
    );
    assert_eq!(
        Organization::query(
            "Science organization".to_string(),
            &organizations
        ),
        None
    );
    // delete association
    organizations = Organization::add(
        "Science organization".to_string(),
        "Research BlockChain".to_string(),
        Student::query("Bob".to_string(), &students).unwrap(),
        &organizations
    );
    assert_eq!(organizations.len(), 2);
    organizations = Organization::remove("Science organization".to_string(), &organizations);
    assert_eq!(organizations.len(), 1);

    // participate association
    organization_map = Organization::participate(
        Student::query("Bob".to_string(), &students).unwrap(),
        "BlockChain organization".to_string(),
        organization_map
    ).unwrap();
    assert_eq!(
        organization_map.get("BlockChain organization").unwrap().len(), 
        1
    );

    assert_eq!(
        Organization::participate(
            Student::query("Bob".to_string(), &students).unwrap(), 
            "BlockChain".to_string(),
            organization_map.clone()
        ),
        Err(Error::Invalid_Organization)
    );

    //exit association
    organization_map = Organization::quit(
        "Bob".to_string(), 
        "BlockChain organization".to_string(), 
        organization_map.clone()
    ).unwrap();

}