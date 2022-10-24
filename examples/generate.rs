mod generated {
    use generate_struct_from_json::generator;
    generator!("./fixtures/person.json");
}

use generated::*;

fn main() {}
