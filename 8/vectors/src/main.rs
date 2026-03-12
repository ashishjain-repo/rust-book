fn main() {
    {
        // Vec to create a vector
        let v: Vec<i32> = Vec::new();
        // vec! macro to create vector with values
        let v2 = vec![1, 2, 3];
    }
    // Updating a Vector, we have to make the variable mutable in order to mutate the values inside.
    {
        let mut v = Vec::new();
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
    }

    // Reading elements of vectors. We can use the index method to get the reference value at that index in the vector, or we can use the get method on the vector which returns Option<&T> then which we can use with match and other validation as needed.
    {
        let v = vec![1, 2, 3, 4, 5];
        let third: &i32 = &v[2];
        println!("The third element in {third}");
        let third: Option<&i32> = v.get(2);
        match third {
            Some(third) => println!("The third elemt is {third}"),
            None => println!("There is no third element"),
        }
    }

    // When we pass the index that is outside of the actual index using [] syntax we get an error, but when we use .get method we get a None values since that index does not exist.
    {
        let v = vec![1, 2, 3, 4, 5];
        // let does_not_exist = &v[100];
        let does_not_exist = v.get(100);
    }
    // Cannot borrow as mutable when it is also being borrowed before as a immutable reference. This will create a compile time error.
    {
        let mut v = vec![1, 2, 3, 4, 5];
        let first = &v[0];
        // v.push(6);
        println!("The first element is: {first}");
    }

    // Iterating over the values in a vector
    {
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{i}");
        }
    }

    // Iterating over mutable vetor and chaning the values of elements
    {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }
    }

    {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    }
}
