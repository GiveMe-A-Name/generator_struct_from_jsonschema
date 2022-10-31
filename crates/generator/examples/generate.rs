use generator::generator;
generator!("fixtures/person.json");

fn main() {
    let person = Person {
        first_name: "hello".to_owned(),
        last_name: "world".to_owned(),
    };
    println!("{:?}", person);
    // let r#type = 10;
    // let name = "xx".to_string();
    // let s = &Some(name);
    // hello(s.as_deref());
}
