use std::collections::HashMap;
fn main() {
    // We can create hashmaps using the new keyword and using the standard library `std::collections::HashMap`. We can add new data using the insert method with k being key and v being value.
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Red"), 20);
    }
    // We can use get method on hashmaps to get the value out, the parameter with get is the name of the key
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Red"), 20);
        // get method returns options &V we use copied to get Option<32> then unwrap it and if there is no value associated with the key it will return 0
        let team_name = String::from("Blue");
        let score = scores.get(&team_name).copied().unwrap_or(0);
    }
    // Iterating over the HashMap like Vectors using for loop
    {
        let mut scores: HashMap<String, i32> = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Red"), 50);
        for (key, value) in &scores {
            println!("{key}: {value}");
        }
    }
    // Managing Ownership in Hash Maps. After the values being passed in insert method the field_name, and field_value variables are not available anymore because we transfererd the ownership.
    {
        use std::collections::HashMap;

        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
    }
    // Overwriting a value in an existing key-val. This code will discard the old value and the new value will be added.
    {
        let mut scores: HashMap<String, i32> = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 50);
    }
    // Adding key-val if key not present. Here the key is Blue and we are adding yellow which does not exist but this will add in the HashMap with the provided value, and when we try to do Blue again using the .entry method it won't work since it already exist.
    {
        let mut scores:HashMap<String, i32> = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);
        println!("{scores:?}")
    }

    // Updating value based on its old value. In this case we are creating keys based on the words from a string and giving a default value of the  0 and then increasing it by one, but if the word is encouontererd again it will not create another key but it will definitily increase the value by one, which will increase the count or the occurences of that key in the HashMap. The entry function returns a mutable reference to that paricular value of that key which we update with count += 1.
    {
        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{map:?}")
    }
}
