use json_schema::Schema;

fn main() {
    let s = include_str!("../fixtures/person.json");
    let schema: Schema = serde_json::from_str(s).unwrap();
    let structs = schema.into_structs();
    println!("{:#?}", structs);
}
