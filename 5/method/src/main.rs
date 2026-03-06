fn main() {
    // Methods are like functions but they are defines within the context of the struct. They are like OOP program where struct is class and methods are member functions. And to refer to the content in the struct we use self and then use the dot notation to refer to the variables or keys. This method in the context of struct is implemented using impl block with the name of the struct that it will belong to. Also we have used &self to tell the method that we do not want to give the ownership but pass it as reference. We can also use the name of the instance as the method the disctiction between them is that method require parantheses and instance does not. These same name methods can be used to make getters since Rust does not have something automatic set getters and setters.

    {
        struct Rectangle {
            width: u32,
            height: u32,
        }

        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }
            fn width(&self) -> bool {
                self.width > 0
            }
        }
        let rectangle: Rectangle = Rectangle {
            width: 30,
            height: 50,
        };
        // We can also see here that we did not use the reference like before in the functions, this happens becuase of auto referencing feature in rust, which matches the signature of the method automatically so it is implict not explicit, we can explicitly do it but rust does it for us.
        println!(
            "The area of the rectangle is {} square pixels",
            rectangle.area()
        );
        println!("The width is greater than zero: {}", rectangle.width());
    }

    // Methods with more parameters
    // here the point of doing this is to see how a method can take multiple parameters just like a function. Here we have declared can_hold function which takes a parameter of &Rectangle to see if it is bigger than the rectable we are working on.
    {
        struct Rectangle {
            width: u32,
            height: u32,
        }
        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }

            fn can_hold(&self, other: &Rectangle) -> bool {
                self.width > other.width && self.height > other.height
            }
        }

        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };

        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    }

    // Associated Functions
    // These functions are the functions that reside inside the impl block but do not take Self as a parameter but instead they are used as a Constructor and are used with :: like String::from as we have seen before.
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }
        impl Rectangle {
            fn square(size: u32) -> Self {
                Self {
                    width: size,
                    height: size,
                }
            }
        }
        // Here we created an instance of the object/struct Rectangle using impl associated function
        let rec_square: Rectangle = Rectangle::square(50);
        println!("rec_square is {rec_square:?}");
    }

    // Multiple impl blocks
    // It is a valid syntax to have multiple impl blocks there is no reason to do this here, but there will be cases where it will be useful.
    {
        struct Rectangle {
            width: u32,
            height: u32,
        }
        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }
        }

        impl Rectangle {
            fn can_hold(&self, other: &Rectangle) -> bool {
                self.width > other.width && self.height > other.height
            }
        }
    }
}
