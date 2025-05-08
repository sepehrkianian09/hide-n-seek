
use serde::{Deserialize, Serialize};

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

use std::error::Error;
use application::common::JsonIo;

#[test]
fn sample_serde_io() -> Result<(), Box<dyn Error>> {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    let json_io = JsonIo::new("tests/serde_person.json");

    json_io.write_json(&person)?;
    println!("JSON written to person.json");

    let read_person: Person = json_io.read_json()?;
    println!("Deserialized from file: {:?}", person);

    assert_eq!(person, read_person,);

    Ok(())
}
