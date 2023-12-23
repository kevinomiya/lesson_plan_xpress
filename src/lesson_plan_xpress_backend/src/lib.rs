#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};


type Memory = VirtualMemory<DefaultMemoryImpl>; 
type IdCell = Cell<u64, Memory>;

// Struct to represent a Lesson 
#[derive (candid::CandidType, Clone,Serialize, Deserialize)]

struct Lesson {
    id: u64,
    title: String,
    description: String,
    grade_level: String,
    subject: String,
    teacher_id: u64,
    students: Vec<u64>, // Connect lessons to students
    schedule: Vec<ScheduleEntry>, // Integrate scheduling
}

#[derive (candid::CandidType, Clone,Serialize, Deserialize)]
//Struct for Teacher 
struct Teacher {
    id: u64,
    name: String,
    subject: String,
    lessons: Vec<u64>, // Link teachers to their lessons
    availability: Vec<ScheduleEntry>, // Indicate available teaching slots
}

#[derive (candid::CandidType, Clone,Serialize, Deserialize)]
// struct for Student 
struct Student {
    id: u64,
    name: String,
    grade_level: String,
    lessons: Vec<u64>, // Connect students to their lessons
}



#[derive (candid::CandidType, Clone,Serialize, Deserialize)]
// Supporting struct schedule Entry 
struct ScheduleEntry {
    id: u64,
    day: String,
    start_time: String,
    end_time: String,
}


// Implement the Storable and BoundedStorable traits for the Lesson struct
impl Storable for Lesson {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
      Cow::Owned(Encode!(self).unwrap())
  }

  fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
      Decode!(bytes.as_ref(), Self).unwrap()
  }
}

impl  BoundedStorable for Lesson {
   const MAX_SIZE: u32 = 1024;
  const IS_FIXED_SIZE: bool = false;
}


// Implement the Storable and BoundedStorable traits for the Teacher struct
impl Storable for Teacher {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
      Cow::Owned(Encode!(self).unwrap())
  }

  fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
      Decode!(bytes.as_ref(), Self).unwrap()
  }
}

impl  BoundedStorable for Teacher {
   const MAX_SIZE: u32 = 1024;
  const IS_FIXED_SIZE: bool = false;
}

// Implement the Storable and BoundedStorable traits for the Student struct
impl Storable for Student {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
      Cow::Owned(Encode!(self).unwrap())
  }

  fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
      Decode!(bytes.as_ref(), Self).unwrap()
  }
}

impl  BoundedStorable for Student {
   const MAX_SIZE: u32 = 1024;
  const IS_FIXED_SIZE: bool = false;
}

// Implement the Storable and BoundedStorable traits for the ScheduleEntry struct
impl Storable for ScheduleEntry {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
      Cow::Owned(Encode!(self).unwrap())
  }

  fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
      Decode!(bytes.as_ref(), Self).unwrap()
  }
}

impl  BoundedStorable for ScheduleEntry {
   const MAX_SIZE: u32 = 1024;
  const IS_FIXED_SIZE: bool = false;
}


thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static STUDENT_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );
    static TEACHER_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))), 0)
            .expect("Cannot create a counter")
    );
    static LESSON_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))), 0)
            .expect("Cannot create a counter")
    );
    static SCHEDULE_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))), 0)
            .expect("Cannot create a counter")
    );
    static STUDENT_MAP: RefCell<StableBTreeMap<u64, Student, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))))
    );
    static TEACHER_MAP: RefCell<StableBTreeMap<u64, Teacher, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5))))
    );
    static LESSON_MAP: RefCell<StableBTreeMap<u64, Lesson, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6))))
    );
    static SCHEDULE_ENTRY_MAP: RefCell<StableBTreeMap<u64, ScheduleEntry, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(7))))
    );


}



// struct for Lesson Payload 
#[derive(candid::CandidType,Serialize, Deserialize)]
struct LessonPayload {
    title: String,
    description: String,
    grade_level: String,
    subject: String,
    teacher_id: u64,
}

// struct for Teacher payload
#[derive(candid::CandidType,Serialize, Deserialize)]
struct TeacherPayload {
    name: String,
    subject: String,
}

//struct for Student Payload
#[derive(candid::CandidType,Serialize, Deserialize)]
struct StudentPayload {
    name: String,
    grade_level: String,
}

//struct for Schedule Entry Payload
#[derive(candid::CandidType,Serialize, Deserialize)]
struct SchedulePayload {
    day: String,
    start_time: String,
    end_time: String,
}


// CRUD Operations 

// CRUD operations for the Lesson Struct 
#[ic_cdk::query]
fn get_all_lessons() -> Result<Vec<Lesson>, Error> {
    let lesson_map: Vec<(u64, Lesson)> =
        LESSON_MAP.with(|service| service.borrow().iter().collect());
    let lessons: Vec<Lesson> = lesson_map
        .into_iter()
        .map(|(_, lesson)| lesson)
        .collect();

    if !lessons.is_empty() {
        Ok(lessons)
    } else {
        Err(Error::NotFound {
            msg: "No Lessons found ".to_string(),
        })
    }
}


#[ic_cdk::query]
fn get_lesson(id: u64) -> Result<Lesson, Error> {
    let lesson = LESSON_MAP.with(|service| service.borrow().get(&id));
    if let Some(lesson) = lesson {
        Ok(lesson)
    } else {
        Err(Error::NotFound {
            msg: format!("Lesson with id={} not found", id),
        })
    }
}

#[ic_cdk::update]
fn add_lesson(lesson_payload: LessonPayload) -> Result<Lesson, String> {
    if lesson_payload.title.trim().is_empty() ||
        lesson_payload.subject.trim().is_empty() ||
       lesson_payload.description.trim().is_empty() ||
       lesson_payload.grade_level.trim().is_empty() {
        return Err("Invalid Lesson data Check for valid data ".to_string());
    }

    let id = LESSON_ID_COUNTER
    .with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    })
    .expect("cannot increment id counter");
    let lesson = Lesson {
        id,
        title: lesson_payload.title ,
        description: lesson_payload.description,
        grade_level: lesson_payload.grade_level,
        subject: lesson_payload.subject,
        teacher_id: lesson_payload.teacher_id,
        students: Vec::new(),
        schedule: Vec::new(),
    };
    do_insert_lesson(&lesson);
    Ok(lesson)
}

// update lesson
#[ic_cdk::update]
fn update_lesson(lesson_id: u64, lesson_payload: LessonPayload) -> Result<Lesson, Error> {
    let lesson = LESSON_MAP.with(|service| service.borrow().get(&lesson_id));
    if let Some(mut lesson) = lesson {
        update_if_not_empty(&mut lesson.title, lesson_payload.title);
        update_if_not_empty(&mut lesson.description, lesson_payload.description);
        update_if_not_empty(&mut lesson.grade_level, lesson_payload.grade_level);
        update_if_not_empty(&mut lesson.subject, lesson_payload.subject);
        lesson.teacher_id = lesson_payload.teacher_id;
        do_insert_lesson(&lesson);
        Ok(lesson)
    } else {
        Err(Error::NotFound {
            msg: format!(
                "Update Lesson  with id={}. not found",
                lesson_id
        )} )
    }

}

// helper function 
fn update_if_not_empty(field: &mut String, new_value: String) {
    if !new_value.trim().is_empty() {
        *field = new_value;
    }
}


// helper method to perform insert.
fn do_insert_lesson(lesson: &Lesson) {
    LESSON_MAP.with(|service| {
        service
            .borrow_mut()
            .insert(lesson.id, lesson.clone())
    });
}

// delete a Lesson 
#[ic_cdk::update]
fn delete_lesson(id: u64) -> Result<Lesson, Error> {
    let lesson = LESSON_MAP.with(|service| service.borrow_mut().remove(&id));
    if let Some(lesson) = lesson {
        Ok(lesson)
    } else {
        Err(Error::NotFound {
            msg: format!("Lesson with id={} not found", id),
        })
    }
}

// CRUD operations for the Teacher Struct
#[ic_cdk::query]
fn get_all_teachers() -> Result<Vec<Teacher>, Error> {
    let teacher_map: Vec<(u64, Teacher)> =
        TEACHER_MAP.with(|service| service.borrow().iter().collect());
    let teachers: Vec<Teacher> = teacher_map
        .into_iter()
        .map(|(_, teacher)| teacher)
        .collect();

    if !teachers.is_empty() {
        Ok(teachers)
    } else {
        Err(Error::NotFound {
            msg: "No Teachers found ".to_string(),
        })
    }
}


#[ic_cdk::query]
fn get_teacher(id: u64) -> Result<Teacher, Error> {
    let teacher = TEACHER_MAP.with(|service| service.borrow().get(&id));
    if let Some(teacher) = teacher {
        Ok(teacher)
    } else {
        Err(Error::NotFound {
            msg: format!("Teacher with id={} not found", id),
        })
    }
}

#[ic_cdk::update]
fn add_teacher(teacher_payload: TeacherPayload) -> Result<Teacher, String> {
    if teacher_payload.name.trim().is_empty() ||
        teacher_payload.subject.trim().is_empty() {
        return Err("Invalid Teacher data Check for valid data ".to_string());
    }

    let id = TEACHER_ID_COUNTER
    .with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    })
    .expect("cannot increment id counter");
    let teacher = Teacher {
        id,
        name: teacher_payload.name ,
        subject: teacher_payload.subject,
        lessons: Vec::new(),
        availability: Vec::new(),
    };
    do_insert_teacher(&teacher);
    Ok(teacher)
}

// update teacher

#[ic_cdk::update]

fn update_teacher(teacher_id: u64, teacher_payload: TeacherPayload) -> Result<Teacher, Error> {
    let teacher = TEACHER_MAP.with(|service| service.borrow().get(&teacher_id));
    if let Some(mut teacher) = teacher {
        update_if_not_empty(&mut teacher.name, teacher_payload.name);
        update_if_not_empty(&mut teacher.subject, teacher_payload.subject);
        do_insert_teacher(&teacher);
        Ok(teacher)
    } else {
        Err(Error::NotFound {
            msg: format!(
                "Update Teacher  with id={}. not found",
                teacher_id
        )} )
    }

}

// helper method to perform insert.
fn do_insert_teacher(teacher: &Teacher) {
    TEACHER_MAP.with(|service| {
        service
            .borrow_mut()
            .insert(teacher.id, teacher.clone())
    });
}

// delete a Teacher

#[ic_cdk::update]
fn delete_teacher(id: u64) -> Result<Teacher, Error> {
    let teacher = TEACHER_MAP.with(|service| service.borrow_mut().remove(&id));
    if let Some(teacher) = teacher {
        Ok(teacher)
    } else {
        Err(Error::NotFound {
            msg: format!("Teacher with id={} not found", id),
        })
    }
}

// CRUD operations for the Student Struct
#[ic_cdk::query]
fn get_all_students() -> Result<Vec<Student>, Error> {
    let student_map: Vec<(u64, Student)> =
        STUDENT_MAP.with(|service| service.borrow().iter().collect());
    let students: Vec<Student> = student_map
        .into_iter()
        .map(|(_, student)| student)
        .collect();

    if !students.is_empty() {
        Ok(students)
    } else {
        Err(Error::NotFound {
            msg: "No Students found ".to_string(),
        })
    }
}


#[ic_cdk::query]
fn get_student(id: u64) -> Result<Student, Error> {
    let student = STUDENT_MAP.with(|service| service.borrow().get(&id));
    if let Some(student) = student {
        Ok(student)
    } else {
        Err(Error::NotFound {
            msg: format!("Student with id={} not found", id),
        })
    }
}

#[ic_cdk::update]
fn add_student(student_payload: StudentPayload) -> Result<Student, String> {
    if student_payload.name.trim().is_empty() ||
        student_payload.grade_level.trim().is_empty() {
        return Err("Invalid Student data Check for valid data ".to_string());
    }

    let id = STUDENT_ID_COUNTER
    .with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    })
    .expect("cannot increment id counter");
    let student = Student {
        id,
        name: student_payload.name ,
        grade_level: student_payload.grade_level,
        lessons: Vec::new(),
    };
    do_insert_student(&student);
    Ok(student)
}

// update student

#[ic_cdk::update]
fn update_student(student_id: u64, student_payload: StudentPayload) -> Result<Student, Error> {
    let student = STUDENT_MAP.with(|service| service.borrow().get(&student_id));
    if let Some(mut student) = student {
        update_if_not_empty(&mut student.name, student_payload.name);
        update_if_not_empty(&mut student.grade_level, student_payload.grade_level);
        do_insert_student(&student);
        Ok(student)
    } else {
        Err(Error::NotFound {
            msg: format!(
                "Update Student  with id={}. not found",
                student_id
        )} )
    }

}

// helper method to perform insert.
fn do_insert_student(student: &Student) {
    STUDENT_MAP.with(|service| {
        service
            .borrow_mut()
            .insert(student.id, student.clone())
    });
}

// delete a Student

#[ic_cdk::update]
fn delete_student(id: u64) -> Result<Student, Error> {
    let student = STUDENT_MAP.with(|service| service.borrow_mut().remove(&id));
    if let Some(student) = student {
        Ok(student)
    } else {
        Err(Error::NotFound {
            msg: format!("Student with id={} not found", id),
        })
    }
}

// CRUD operations for the ScheduleEntry Struct
#[ic_cdk::query]
fn get_all_schedule_entries() -> Result<Vec<ScheduleEntry>, Error> {
    let schedule_entry_map: Vec<(u64, ScheduleEntry)> =
        SCHEDULE_ENTRY_MAP.with(|service| service.borrow().iter().collect());
    let schedule_entries: Vec<ScheduleEntry> = schedule_entry_map
        .into_iter()
        .map(|(_, schedule_entry)| schedule_entry)
        .collect();

    if !schedule_entries.is_empty() {
        Ok(schedule_entries)
    } else {
        Err(Error::NotFound {
            msg: "No Schedule Entries found ".to_string(),
        })
    }
}


#[ic_cdk::query]
fn get_schedule_entry(id: u64) -> Result<ScheduleEntry, Error> {
    let schedule_entry = SCHEDULE_ENTRY_MAP.with(|service| service.borrow().get(&id));
    if let Some(schedule_entry) = schedule_entry {
        Ok(schedule_entry)
    } else {
        Err(Error::NotFound {
            msg: format!("Schedule Entry with id={} not found", id),
        })
    }
}

#[ic_cdk::update]
fn add_schedule_entry(schedule_payload: SchedulePayload) -> Result<ScheduleEntry, String> {
    if schedule_payload.day.trim().is_empty() ||
        schedule_payload.start_time.trim().is_empty() ||
        schedule_payload.end_time.trim().is_empty() {
        return Err("Invalid Schedule Entry data Check for valid data ".to_string());
    }

    let id = SCHEDULE_ID_COUNTER
    .with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    })
    .expect("cannot increment id counter");
    let schedule_entry = ScheduleEntry {
        id,
        day: schedule_payload.day ,
        start_time: schedule_payload.start_time,
        end_time: schedule_payload.end_time,
    };
    do_insert_schedule_entry(&schedule_entry);
    Ok(schedule_entry)
}

// update schedule entry

#[ic_cdk::update]
fn update_schedule_entry(schedule_id: u64, schedule_payload: SchedulePayload) -> Result<ScheduleEntry, Error> {
    let schedule_entry = SCHEDULE_ENTRY_MAP.with(|service| service.borrow().get(&schedule_id));
    if let Some(mut schedule_entry) = schedule_entry {
        update_if_not_empty(&mut schedule_entry.day, schedule_payload.day);
        update_if_not_empty(&mut schedule_entry.start_time, schedule_payload.start_time);
        update_if_not_empty(&mut schedule_entry.end_time, schedule_payload.end_time);
        do_insert_schedule_entry(&schedule_entry);
        Ok(schedule_entry)
    } else {
        Err(Error::NotFound {
            msg: format!(
                "Update Schedule Entry  with id={}. not found",
                schedule_id
        )} )
    }

}

// helper method to perform insert.
fn do_insert_schedule_entry(schedule_entry: &ScheduleEntry) {
    SCHEDULE_ENTRY_MAP.with(|service| {
        service
            .borrow_mut()
            .insert(schedule_entry.id, schedule_entry.clone())
    });
}

// delete a Schedule Entry

#[ic_cdk::update]
fn delete_schedule_entry(id: u64) -> Result<ScheduleEntry, Error> {
    let schedule_entry = SCHEDULE_ENTRY_MAP.with(|service| service.borrow_mut().remove(&id));
    if let Some(schedule_entry) = schedule_entry {
        Ok(schedule_entry)
    } else {
        Err(Error::NotFound {
            msg: format!("Schedule Entry with id={} not found", id),
        })
    }
}


// add a student to a lesson
#[ic_cdk::update]
fn insert_student_to_lesson(lesson_id: u64, student_id: u64) -> Result<Lesson, Error> {
    let lesson = LESSON_MAP.with(|service| service.borrow().get(&lesson_id));
    if let Some(mut lesson) = lesson {
        lesson.students.push(student_id);
        do_insert_lesson(&lesson);
        Ok(lesson)
    } else {
        Err(Error::NotFound {
            msg: format!(
                " Lesson  with id={}. not found",
                lesson_id
        )} )
    }

}

// add a schedule to a lesson
#[ic_cdk::update]
fn insert_schedule_to_lesson(lesson_id: u64, schedule_id: u64) -> Result<Lesson, Error> {
    let lesson = LESSON_MAP.with(|service| service.borrow().get(&lesson_id));
    if let Some(mut lesson) = lesson {
        let schedule = SCHEDULE_ENTRY_MAP.with(|service| service.borrow().get(&schedule_id));
        if let Some(schedule) = schedule {
            lesson.schedule.push(schedule.clone());
            do_insert_lesson(&lesson);
            Ok(lesson)
        } else {
            return Err(Error::NotFound {
                msg: format!(
                    "Schedule not found ",
            )} )
        }
   
    } else {
        Err(Error::NotFound {
            msg: format!(
                " Lesson  with id={}. not found",
                lesson_id
        )} )
    }

}

// add a lesson to a teacher
#[ic_cdk::update]
fn insert_lesson_to_teacher(teacher_id: u64, lesson_id: u64) -> Result<Teacher, Error> {
    let teacher = TEACHER_MAP.with(|service| service.borrow().get(&teacher_id));
    if let Some(mut teacher) = teacher {
        teacher.lessons.push(lesson_id);
        do_insert_teacher(&teacher);
        Ok(teacher)
    } else {
        Err(Error::NotFound {
            msg: format!(
                " Teacher  with id={}. not found",
                teacher_id
        )} )
    }

}

// add a schedule to a teacher
#[ic_cdk::update]
fn insert_schedule_to_teacher(teacher_id: u64, schedule_id: u64) -> Result<Teacher, Error> {
    let teacher = TEACHER_MAP.with(|service| service.borrow().get(&teacher_id));
    if let Some(mut teacher) = teacher {
        let schedule = SCHEDULE_ENTRY_MAP.with(|service| service.borrow().get(&schedule_id));
        if let Some(schedule) = schedule {
            teacher.availability.push(schedule.clone());
            do_insert_teacher(&teacher);
            Ok(teacher)
        } else {
            return Err(Error::NotFound {
                msg: format!(
                    "Schedule not found ",
            )} )

        }
   
    } else {
        Err(Error::NotFound {
            msg: format!(
                "Teacher  with id={}. not found",
                teacher_id
        )} )
    }

}



// add a lesson to a student
#[ic_cdk::update]
fn insert_lesson_to_student(student_id: u64, lesson_id: u64) -> Result<Student, Error> {
    let student = STUDENT_MAP.with(|service| service.borrow().get(&student_id));
    if let Some(mut student) = student {
        student.lessons.push(lesson_id);
        do_insert_student(&student);
        Ok(student)
    } else {
        Err(Error::NotFound {
            msg: format!(
                " Student  with id={}. not found",
                student_id
        )} )
    }

}

//  function to get all lessons for a teacher
#[ic_cdk::query]
fn get_all_lessons_for_teacher(teacher_id: u64) -> Result<Vec<Lesson>, Error> {
    let teacher = TEACHER_MAP.with(|service| service.borrow().get(&teacher_id));
    if let Some(teacher) = teacher {
        let mut lessons: Vec<Lesson> = Vec::new();
        for lesson_id in &teacher.lessons {
            let lesson = LESSON_MAP.with(|service| service.borrow().get(&lesson_id));
            if let Some(lesson) = lesson {
                lessons.push(lesson.clone());
            }
        }
        Ok(lessons)
    } else {
        Err(Error::NotFound {
            msg: format!(
                "Teacher with id={}. not found",
                teacher_id
        )} )
    }

}

//  function to get all lessons for a student
#[ic_cdk::query]
fn get_all_lessons_for_student(student_id: u64) -> Result<Vec<Lesson>, Error> {
    let student = STUDENT_MAP.with(|service| service.borrow().get(&student_id));
    if let Some(student) = student {
        let mut lessons: Vec<Lesson> = Vec::new();
        for lesson_id in &student.lessons {
            let lesson = LESSON_MAP.with(|service| service.borrow().get(&lesson_id));
            if let Some(lesson) = lesson {
                lessons.push(lesson.clone());
            }
        }
        Ok(lessons)
    } else {
        Err(Error::NotFound {
            msg: format!(
                "Student with id={}. not found",
                student_id
        )} )
    }

}

//  function to get all students for a lesson
#[ic_cdk::query]
fn get_all_students_for_lesson(lesson_id: u64) -> Result<Vec<Student>, Error> {
    let lesson = LESSON_MAP.with(|service| service.borrow().get(&lesson_id));
    if let Some(lesson) = lesson {
        let mut students: Vec<Student> = Vec::new();
        for student_id in &lesson.students {
            let student = STUDENT_MAP.with(|service| service.borrow().get(&student_id));
            if let Some(student) = student {
                students.push(student.clone());
            }
        }
        Ok(students)
    } else {
        Err(Error::NotFound {
            msg: format!(
                "Lesson with id={}. not found",
                lesson_id
        )} )
    }

}

// delete a lesson from a teacher
#[ic_cdk::update]
fn delete_lesson_from_teacher(teacher_id: u64, lesson_id: u64) -> Result<Teacher, Error> {
    let teacher = TEACHER_MAP.with(|service| service.borrow().get(&teacher_id));
    if let Some(mut teacher) = teacher {
        teacher.lessons.retain(|lesson| lesson != &lesson_id);
        do_insert_teacher(&teacher);
        Ok(teacher)
    } else {
        Err(Error::NotFound {
            msg: format!(
                " Teacher  with id={}. not found",
                teacher_id
        )} )
    }

}

// delete a lesson from a student
#[ic_cdk::update]
fn delete_lesson_from_student(student_id: u64, lesson_id: u64) -> Result<Student, Error> {
    let student = STUDENT_MAP.with(|service| service.borrow().get(&student_id));
    if let Some(mut student) = student {
        student.lessons.retain(|lesson| lesson != &lesson_id);
        do_insert_student(&student);
        Ok(student)
    } else {
        Err(Error::NotFound {
            msg: format!(
                " Student  with id={}. not found",
                student_id
        )} )
    }

}

// delete a student from a lesson
#[ic_cdk::update]
fn delete_student_from_lesson(lesson_id: u64, student_id: u64) -> Result<Lesson, Error> {
    let lesson = LESSON_MAP.with(|service| service.borrow().get(&lesson_id));
    if let Some(mut lesson) = lesson {
        lesson.students.retain(|student| student != &student_id);
        do_insert_lesson(&lesson);
        Ok(lesson)
    } else {
        Err(Error::NotFound {
            msg: format!(
                " Lesson  with id={}. not found",
                lesson_id
        )} )
    }

}

// delete a schedule from a lesson
#[ic_cdk::update]
fn delete_schedule_from_lesson(lesson_id: u64, schedule_id: u64) -> Result<Lesson, Error> {
    let lesson = LESSON_MAP.with(|service| service.borrow().get(&lesson_id));
    if let Some(mut lesson) = lesson {
        lesson.schedule.retain(|schedule| schedule.id != schedule_id);
        do_insert_lesson(&lesson);
        Ok(lesson)
    } else {
        Err(Error::NotFound {
            msg: format!(
                " Lesson  with id={}. not found",
                lesson_id
        )} )
    }

}

// delete a schedule from a teacher
#[ic_cdk::update]
fn delete_schedule_from_teacher(teacher_id: u64, schedule_id: u64) -> Result<Teacher, Error> {
    let teacher = TEACHER_MAP.with(|service| service.borrow().get(&teacher_id));
    if let Some(mut teacher) = teacher {
        teacher.availability.retain(|schedule| schedule.id != schedule_id);
        do_insert_teacher(&teacher);
        Ok(teacher)
    } else {
        Err(Error::NotFound {
            msg: format!(
                " Teacher  with id={}. not found",
                teacher_id
        )} )
    }

}





// Error type for the service
#[derive(candid::CandidType, Deserialize, Serialize)]
enum  Error {
    NotFound { msg: String },
}

// Export the candid interface
ic_cdk::export_candid!();