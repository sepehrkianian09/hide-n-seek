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
use std::fs::File;
use std::io::Read;
use std::io::Write;

fn write_json<T: Serialize>(a_struct: &T, file_name: &str) -> Result<(), Box<dyn Error>> {
    // Serialize to JSON string
    let json_string = serde_json::to_string_pretty(a_struct)?; // Use `to_string` for compact output

    // Create or overwrite the file
    let mut file = File::create(file_name)?;

    // Write the JSON string to the file
    file.write_all(json_string.as_bytes())?;

    Ok(())
}

fn read_json<T: for<'a> Deserialize<'a>>(file_name: &str) -> Result<T, Box<dyn Error>> {
    // Open the file
    let mut file = File::open(file_name)?;

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let deserialized_json = serde_json::from_str::<T>(&contents)?;
    // Deserialize the JSON string into a Person struct

    Ok(deserialized_json)
}

#[test]
fn sample_serde_write() -> Result<(), Box<dyn Error>> {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    write_json(&person, "tests/serde_person.json")?;

    println!("JSON written to person.json");

    Ok(())
}

#[test]
fn sample_serde_read() -> Result<(), Box<dyn Error>> {
    let person: Person = read_json("tests/serde_person.json")?;

    println!("Deserialized from file: {:?}", person);
    assert_eq!(
        person,
        Person {
            name: "Alice".to_string(),
            age: 30,
        }
    );

    Ok(())
}
