

fn main() {

        use std::collections::HashMap;

        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

    /*
    This code will print {"world": 2, "hello": 1, "wonderful": 1}.
    The or_insert method actually returns a mutable reference (&mut V)
    to the value for this key. Here we store that mutable reference in
    the count variable, so in order to assign to that value, we must
    first dereference count using the asterisk (*). The mutable reference
    goes out of scope at the end of the for loop, so all of these
    changes are safe and allowed by the borrowing rules.
    */
        println!("{:?}", map);

}
