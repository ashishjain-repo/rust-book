fn main() {
    // This code here calculate the index of the string where there is empty space. We call this function and pass the string by reference and instead of any manipulation we store the content as bytes using .as_bytes on string and then iterating over them by making them tuple using the enumerate. Then we look for the byte ' ' empty space wherever it is in the string return the index. What happes after is that since we are only getting the index of it rust allows us to do all that even the moment we received the index we are clearing the string, which means if we go back use the word to get the content out of string it will fail because there is not string that contains "hello world" but the word will still have the value 5 but now this value has no meaning but rust still permits it because rust did not know the whole context.
    {
        let mut s = String::from("hello world");
        let word = first_word(&s);
        s.clear();
        println!("Is word still have the value: word={}", word);
        fn first_word(s: &String) -> usize {
            let bytes = s.as_bytes();
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return i;
                }
            }
            s.len()
        }
    }

    // String Slices are the reference to the part of the string instead of the whole string, it is like using [1:3] in python.
    {
        let s = String::from("Hello World");
        let hello = &s[0..5];
        let world = &s[6..11];
        println!("Var hello: {}, Var world: {}", hello, world);
    }

    // We can drop the first index if we want to start from the beginning and that applies to the last index as well, we can drop that as well which will mean that the end of the index
    {
        let s = String::from("Hello");
        // The two variables are doing the same thing
        let slice = &s[0..2];
        let slice = &s[..2];
        // The two variables are doing the same thing
        let len = s.len();
        let slice = &s[0..len];
        let slice = &s[0..];
        // We can also drop both values to get the whole string
        let slice = &s[0..len];
        let slice = &s[..];
    }

    // Here we have completed the function that we started with, this time by using Slices instead of getting reusable index we are actually getting the word that we are looking for.
    {
        let s = String::from("hello");
        let word = first_word(&s);
        println!("The first word is: {}", word);
        fn first_word(s: &String) -> &str {
            let bytes = s.as_bytes();
            for(i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }
            &s[..]
        }
    }

    // Slices in arrays
    {
        let a = [1,2,3,4,5];
        let slice = &a[1..3];
        assert_eq!(slice, &[2, 3]);
    }
}
