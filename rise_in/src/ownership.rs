/*
In this task, students will create a simple Rust program that demonstrates the concepts of ownership, borrowing, and references. 
The program will take two strings as input, concatenate them, and then print the result without violating any ownership rules.
*/

fn _concatenate_strings(str1: &str, str2: &str) -> String {
    
    let mut result = String::new();
    result.push_str(str1);
    result.push_str(str2);
    result

}

pub fn concatenater() {
    println!("Welcome to ownership tasks on Rise In")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concatenate_strings() {
        let string1 = String::from("Hello");
        let string2 = String::from(" World");
        let concatenated = _concatenate_strings(&string1, &string2);
        assert_eq!(concatenated, "Hello World");
    }

}