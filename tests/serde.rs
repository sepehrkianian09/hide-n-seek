use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}


#[test]
fn sample_serde_equals() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    // Convert to JSON string
    let json_string = serde_json::to_string(&person).unwrap();
    let deserialized_person: Person = serde_json::from_str(&json_string).unwrap();
    assert_eq!(person, deserialized_person);
}

#[test]
#[should_panic(expected = r#"assertion `left == right` failed
  left: Person { name: "Alice", age: 40 }
 right: Person { name: "Alice", age: 30 }"#)]
fn sample_serde_non_equals() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    let person2 = Person {
        name: "Alice".to_string(),
        age: 40,
    };

    // Convert to JSON string
    let json_string = serde_json::to_string(&person).unwrap();
    let deserialized_person: Person = serde_json::from_str(&json_string).unwrap();
    assert_eq!(person2, deserialized_person);
}