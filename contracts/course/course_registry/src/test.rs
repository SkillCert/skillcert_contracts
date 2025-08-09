use crate::CourseRegistry;
use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env, String, Symbol, Vec};

use crate::{
    functions::{
        get_course::course_registry_get_course,
        get_courses_by_instructor::course_registry_get_courses_by_instructor,
        get_prerequisites_by_course::get_prerequisites_by_course_id,
        remove_module::course_registry_remove_module,
        list_categories::course_registry_list_categories,
    },
    schema::{Course, CourseModule, DataKey},
};

const COURSE_KEY: Symbol = symbol_short!("course");

// Helpers
fn create_test_env() -> Env {
    let env = Env::default();
    env.budget().reset_unlimited();
    env
}

fn create_sample_module(env: &Env) -> CourseModule {
    CourseModule {
        id: String::from_str(env, "test_module_123"),
        course_id: String::from_str(env, "test_course_123"),
        position: 0,
        title: String::from_str(env, "Introduction to Blockchain"),
        created_at: 0,
    }
}

fn create_sample_course(env: &Env, id: u128, creator: Address) -> Course {
    Course {
        id: String::from_str(env, &id.to_string()),
        title: String::from_str(env, &format!("Course {}", id)),
        description: String::from_str(env, "Test Description"),
        creator,
        published: true,
        price: 1000,
        category: Some(String::from_str(env, "Programming")),
        language: Some(String::from_str(env, "English")),
        thumbnail_url: None,
        prerequisites: Vec::new(&env),
    }
}

// 🔹 Tests

#[test]
fn test_remove_module_success() {
    let env = create_test_env();
    let contract = env.register_stellar_asset_contract_v2(Address::generate(&env));
    let contract_id = contract.address();
    let module = create_sample_module(&env);
    let module_id = module.id.clone();

    env.mock_all_auths();

    env.as_contract(&contract_id, || {
        let storage = env.storage().persistent();
        let key = DataKey::Module(module_id.clone());
        storage.set(&key, &module);
    });

    let result = env.as_contract(&contract_id, || {
        course_registry_remove_module(&env, module_id.clone())
    });

    assert!(result.is_ok());

    env.as_contract(&contract_id, || {
        let storage = env.storage().persistent();
        let key = DataKey::Module(module_id.clone());
        assert!(!storage.has(&key));
    });
}

#[test]
fn test_remove_multiple_different_modules() {
    let env = create_test_env();
    let contract = env.register_stellar_asset_contract_v2(Address::generate(&env));
    let contract_id = contract.address();

    env.mock_all_auths();

    let mut module1 = create_sample_module(&env);
    module1.id = String::from_str(&env, "module_1");

    let mut module2 = create_sample_module(&env);
    module2.id = String::from_str(&env, "module_2");

    env.as_contract(&contract_id, || {
        let storage = env.storage().persistent();
        storage.set(&DataKey::Module(module1.id.clone()), &module1);
        storage.set(&DataKey::Module(module2.id.clone()), &module2);
    });

    let result1 = env.as_contract(&contract_id, || {
        course_registry_remove_module(&env, module1.id.clone())
    });
    assert!(result1.is_ok());

    let result2 = env.as_contract(&contract_id, || {
        course_registry_remove_module(&env, module2.id.clone())
    });
    assert!(result2.is_ok());

    env.as_contract(&contract_id, || {
        let storage = env.storage().persistent();
        assert!(!storage.has(&DataKey::Module(module1.id.clone())));
        assert!(!storage.has(&DataKey::Module(module2.id.clone())));
    });
}

#[test]
fn test_remove_module_storage_isolation() {
    let env = create_test_env();
    let contract = env.register_stellar_asset_contract_v2(Address::generate(&env));
    let contract_id = contract.address();
    let module = create_sample_module(&env);
    let module_id = module.id.clone();

    env.mock_all_auths();

    env.as_contract(&contract_id, || {
        let storage = env.storage().persistent();
        storage.set(&DataKey::Module(module_id.clone()), &module);

        let other_key = DataKey::Module(String::from_str(&env, "other_module"));
        let mut other_module = create_sample_module(&env);
        other_module.id = String::from_str(&env, "other_module");
        storage.set(&other_key, &other_module);
    });

    let result = env.as_contract(&contract_id, || {
        course_registry_remove_module(&env, module_id)
    });
    assert!(result.is_ok());

    env.as_contract(&contract_id, || {
        let storage = env.storage().persistent();
        assert!(!storage.has(&DataKey::Module(module.id.clone())));
        assert!(storage.has(&DataKey::Module(String::from_str(&env, "other_module"))));
    });
}

#[test]
fn test_get_course_success() {
    let env = Env::default();
    let course_id = String::from_str(&env, "course_123");
    let title = String::from_str(&env, "Test Course");
    let description = String::from_str(&env, "A test course description");
    let creator = Address::generate(&env);
    let published = true;
    let price = 23;

    let course = Course {
        id: course_id.clone(),
        title: title.clone(),
        description: description.clone(),
        price: price,
        creator: creator.clone(),
        published,
        category: None,
        language: None,
        thumbnail_url: None,
        prerequisites: Vec::new(&env),
    };

    let contract_id = env.register_contract(None, crate::CourseRegistry);

    let key = Symbol::new(&env, "course");
    env.as_contract(&contract_id, || {
        env.storage()
            .instance()
            .set(&(key, course_id.clone()), &course);
    });

    let retrieved = env.as_contract(&contract_id, || course_registry_get_course(&env, course_id));

    assert_eq!(retrieved.id, course.id);
    assert_eq!(retrieved.title, course.title);
    assert_eq!(retrieved.description, course.description);
    assert_eq!(retrieved.creator, course.creator);
    assert_eq!(retrieved.published, course.published);
}

#[test]
#[should_panic(expected = "Course not found")]
fn test_get_course_not_found() {
    let env = Env::default();
    let contract_id = env.register_contract(None, crate::CourseRegistry);

    let fake_id = String::from_str(&env, "not_found");

    env.as_contract(&contract_id, || course_registry_get_course(&env, fake_id));
}

#[test]
fn test_get_courses_by_instructor_empty() {
    let env = Env::default();
    let instructor = Address::generate(&env);
    let contract_id = env.register_contract(None, crate::CourseRegistry);

    let courses = env.as_contract(&contract_id, || {
        course_registry_get_courses_by_instructor(&env, instructor.clone())
    });

    assert_eq!(courses.len(), 0);
}

#[test]
fn test_get_courses_by_instructor_found() {
    let env = Env::default();
    let instructor = Address::generate(&env);
    let contract_id = env.register_contract(None, crate::CourseRegistry);

    let course_id = String::from_str(&env, "1");
    let course = Course {
        id: course_id.clone(),
        title: String::from_str(&env, "Rust 101"),
        description: String::from_str(&env, "Intro to Rust"),
        creator: instructor.clone(),
        published: true,
        price: 1500,
        category: Some(String::from_str(&env, "Programming")),
        language: Some(String::from_str(&env, "English")),
        thumbnail_url: None,
        prerequisites: Vec::new(&env),
    };

    let key = (symbol_short!("course"), course_id);
    env.as_contract(&contract_id, || {
        env.storage().persistent().set(&key, &course);
    });

    let results = env.as_contract(&contract_id, || {
        course_registry_get_courses_by_instructor(&env, instructor.clone())
    });

    assert_eq!(results.len(), 1);
    assert_eq!(results.get(0).unwrap().id, course.id);
}

#[test]
fn test_get_prerequisites_by_course_id() {
    let env = Env::default();
    let contract_id = env.register_contract(None, crate::CourseRegistry);
    //let contract_id: Address = env.register(CourseRegistry, {});
    let course_id = String::from_str(&env, "course_123");

    let course = Course {
        id: course_id.clone(),
        title: String::from_str(&env, "Test Course"),
        description: String::from_str(&env, "Test Description"),
        creator: Address::generate(&env),
        published: true,
        price: 1000,
        category: None,
        language: Some(String::from_str(&env, "English")),
        thumbnail_url: None,
        prerequisites: Vec::new(&env),
    };
    let key = (symbol_short!("course"), course_id.clone());
    env.as_contract(&contract_id, || {
        env.storage().persistent().set(&key, &course);
    });
    let prerequisites = env.as_contract(&contract_id, || {
        get_prerequisites_by_course_id(&env, course_id.clone())
    });
    assert!(prerequisites.is_empty());
}

#[test]
fn test_list_categories_counts() {
    let env = Env::default();
    let contract_id = env.register(CourseRegistry, ());

    env.as_contract(&contract_id, || {
        // Create 3 courses: 2 in "Programming", 1 in "Data"
        CourseRegistry::create_course(
            env.clone(),
            String::from_str(&env, "A"),
            String::from_str(&env, "d"),
            10,
            Some(String::from_str(&env, "Programming")),
            None,
            None,
        );
        CourseRegistry::create_course(
            env.clone(),
            String::from_str(&env, "B"),
            String::from_str(&env, "d"),
            10,
            Some(String::from_str(&env, "Data")),
            None,
            None,
        );
        CourseRegistry::create_course(
            env.clone(),
            String::from_str(&env, "C"),
            String::from_str(&env, "d"),
            10,
            Some(String::from_str(&env, "Programming")),
            None,
            None,
        );

        // Call the function to list categories
        let cats = course_registry_list_categories(&env);
        assert_eq!(cats.len(), 2); // Should have 2 unique categories

        // Verify counts without assuming order
        let mut prog = 0u128;
        let mut data = 0u128;
        for c in cats.iter() {
            let cname = c.name.clone();
            if cname.to_string() == "Programming" {
                prog = c.count;
            }
            if cname.to_string() == "Data" {
                data = c.count;
            }
        }
        assert_eq!(prog, 2);
        assert_eq!(data, 1);
    });
}

#[test]
fn test_list_categories_empty() {
    let env = Env::default();
    let contract_id = env.register(CourseRegistry, ());
    // No courses created, should return empty vector
    let cats = env.as_contract(&contract_id, || course_registry_list_categories(&env));
    assert_eq!(cats.len(), 0);
}

#[test]
fn test_list_categories_ignores_none() {
    let env = Env::default();
    let contract_id = env.register(CourseRegistry, ());

    env.as_contract(&contract_id, || {
        // Course without category (None)
        CourseRegistry::create_course(
            env.clone(),
            String::from_str(&env, "A"),
            String::from_str(&env, "d"),
            10,
            None,
            None,
            None,
        );
        // Course with category (Some)
        CourseRegistry::create_course(
            env.clone(),
            String::from_str(&env, "B"),
            String::from_str(&env, "d"),
            10,
            Some(String::from_str(&env, "Programming")),
            None,
            None,
        );

        let cats = course_registry_list_categories(&env);
        assert_eq!(cats.len(), 1); // Only "Programming" should be returned
        let c = cats.get(0).unwrap();
        assert_eq!(c.name, String::from_str(&env, "Programming"));
        assert_eq!(c.count, 1);
    });
}

#[test]
fn test_list_categories_with_id_gaps() {
    let env = Env::default();
    let contract_id = env.register(CourseRegistry, ());

    env.as_contract(&contract_id, || {
        // Create course 1
        CourseRegistry::create_course(
            env.clone(),
            String::from_str(&env, "Course 1"),
            String::from_str(&env, "Desc"),
            10,
            Some(String::from_str(&env, "Programming")),
            None,
            None,
        );
        // Create course 2
        CourseRegistry::create_course(
            env.clone(),
            String::from_str(&env, "Course 2"),
            String::from_str(&env, "Desc"),
            10,
            Some(String::from_str(&env, "Data")),
            None,
            None,
        );

        // Manually delete course 2 to create an ID gap
        let key = (symbol_short!("course"), String::from_str(&env, "2"));
        env.storage().persistent().remove(&key);

        // Create course 3 (this will still have ID 3)
        CourseRegistry::create_course(
            env.clone(),
            String::from_str(&env, "Course 3"),
            String::from_str(&env, "Desc"),
            10,
            Some(String::from_str(&env, "Programming")),
            None,
            None,
        );

        // Call the function - it should skip missing ID 2 but still count 1 and 3
        let cats = course_registry_list_categories(&env);
        let mut prog = 0u128;
        let mut data = 0u128;
        for c in cats.iter() {
            if c.name.to_string() == "Programming" {
                prog = c.count;
            }
            if c.name.to_string() == "Data" {
                data = c.count;
            }
        }
        assert_eq!(prog, 2); // Course 1 and Course 3
        assert_eq!(data, 0); // Course 2 was deleted
    });
}
