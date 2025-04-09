// This is a simple example of how to create and use structs in Rust.
struct MyStruct {
    name: String,
    age: u32,
}

impl MyStruct {
    fn new(name: String, age: u32) -> Self {
        MyStruct { name, age }
    }

    fn display(&self) {
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
    }
}

fn main() {
    let my_struct = MyStruct::new(String::from("Alice"), 25);
    my_struct.display();
}
