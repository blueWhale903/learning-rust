struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str)  {
        self.last_name = last.to_string();
    }
}

pub fn run() {
    let person = Person::new("Long", "Hoang");
    println!("{}", person.get_full_name());

    let mut person2 = Person::new("Arthur", "Hoang");
    println!("{}", person2.get_full_name());

    person2.set_last_name("Freeman");
    println!("{}", person2.get_full_name());

}