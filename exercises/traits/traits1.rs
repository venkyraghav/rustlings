// traits1.rs
// Time to implement some traits!
//
// Your task is to implement the trait
// `AppendBar' for the type `String'.
//
// The trait AppendBar has only one function,
// which appends "Bar" to any object
// implementing this trait.
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a hint.

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    //Add your code here
    fn append_bar(self) -> Self {
       let mut s = String::from(self);
       s.push_str("Bar");
       s
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Name {
    fname: String,
    lname: String,
}

impl Name {
    pub fn new() -> Name {
        Name {fname: "".to_string(), lname: "".to_string()}
    }
}

impl AppendBar for Name {
    fn append_bar(self) -> Self {
        let mut s = Name::new();

        s.fname.push_str(&self.fname);
        s.fname.push_str("Bar");

        s.lname.push_str(&self.lname);
        s.lname.push_str("Bar");
        s
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);

    let s2 = Name { fname: "Venky".to_string(), lname: "Narayanan".to_string()};
    let s2 = s2.append_bar();
    println!("s2: {:?}", s2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }

    #[test]
    fn is_name_bar() {
        let s = Name {fname: "Venky".to_string(), lname: "Narayanan".to_string()}.append_bar();
        let s2 = Name {fname: "VenkyBar".to_string(), lname: "NarayananBar".to_string()};
        
        assert_eq!(s, s2);
    }
}
