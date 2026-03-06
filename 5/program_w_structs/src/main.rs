fn main() {
    {
        let width1: u32 = 30;
        let height1: u32 = 50;
        println!(
            "The area of the rectangle is {} square pixels.",
            area(width1, height1)
        );
        fn area(width: u32, height: u32) -> u32 {
            width * height
        }
    }
    // Same function with only one signature using tuple instead of two parameters
    {
        let rect1 = (30, 50);
        println!(
            "The area of the rectangle is {} square pixels.",
            area(rect1)
        );
        fn area(dimensions: (u32, u32)) -> u32 {
            dimensions.0 * dimensions.1
        }
    }

    // The previous function is better but it makes the code unreadble which can introduce errors, we are now going to use structs to accomplish the same thing but with more readble and meaningful code. Also we want main to retain the ownership so we pass it as a reference or borrow the value and then return the appropriate calculation
    {
        struct Rectangle {
            width: u32,
            height: u32,
        }
        let rect1: Rectangle = Rectangle {
            width: 30,
            height: 50,
        };
        println!(
            "The area of the rectangle is {} square pixels.",
            area(&rect1)
        );
        fn area(rectangle: &Rectangle) -> u32 {
            rectangle.height * rectangle.width
        }
    }

    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }
        let rect1: Rectangle = Rectangle {
            width: 30,
            height: 50,
        };
        // This line is errornous because the println macro cannot format the struct
        // println!("rect1 is {rect1}");

        // Now we are going to add derive debug which can help us debug inputs/outputs to the struct and see if that returns or prints the struct. Alone this method of telling complier to show the rectngle won't work we have to add #[derive(Debug)] to struct to get this output. Because Struct is build on multiple componentes and regular formatting cannot format that because it has ':' and values and keys and curly braces, so macro get confuse and asks us to explicitly tell it what we want.
        println!("rect1 is {rect1:?}");
        // This formatting is good for small structs but when we have larger structs we can use the following, which will format it nicely and is readable.
        println!("rect1 is {rect1:#?}");
    }

    // This is an example on using dbg! instead of standard input/outupt macro of println, but this macro instead of using references takes ownership. So we have to explicitely tell dbg! or pass the reference instead of just regular variable because of ownership issues.
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        let scale = 2;
        let rect1 = Rectangle {
            width: dbg!(30 * scale),
            height: 50,
        };

        dbg!(&rect1);
    }
}
