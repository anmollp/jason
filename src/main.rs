use jason::parse_from_str;

fn main() {
    let example_json = r#"
    {
      "id": 1024,
      "username": "jdoe_99",
      "email": "john.doe@example.com",
      "isActive": true ,
      "roles": ["Admin", "Editor"],
      "preferences": {
        "theme": "dark",
        "notifications": "enabled"
      },
      "loginCount": 42
    }
    "#;
    let result = parse_from_str(example_json);
    match result {
        Ok(value) => println!("{:?}", value),
        Err(e) => println!("Error: {}", e),
    }
}
