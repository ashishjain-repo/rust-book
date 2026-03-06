fn main() {
    // Enums are way to store several variants where each variant can optionally store different types and amounts of data. Here we have Ip Address version 4 and 6, and an instance can be of one variant at a time, to define an instance we use '::' and the name of the variant.
    {
        enum IpAddrKind {
            V4,
            V6,
        }

        let four: IpAddrKind = IpAddrKind::V4;
        let six: IpAddrKind = IpAddrKind::V6;

        fn route(ip_kind: IpAddrKind) {}
    }

    // We can be tempted to use struct to pass the data, but with enums we can define the type anda store the data directly.
    {
        enum IpAddr {
            V4(String),
            V6(String),
        }

        let home = IpAddr::V4(String::from("127.0.0.1"));
        let loopback = IpAddr::V6(String::from("::1"));
    }

    // Struct lacks in ability to have once instance with multiple values like an IP has which is 4 values that are between 0-255 and seperated with '.'
    {
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }
        let home = IpAddr::V4(127, 0, 0, 1);
        let loopback = IpAddr::V6(String::from("::1"));
    }

    // Standard library have something existing for something like Ipaddress which looks like this. This shows that we can put any kind of data as an enum variant.
    {
        struct Ipv4Addr {}
        struct Ipv6Addr {}

        enum IpAddr {
            V4(Ipv4Addr),
            V6(Ipv6Addr),
        }
    }
    // Here is an another example that contains varity of types embedded for its variants. We can use the structs for all of those but we can use the enum and pass easily as a parameter in the function if needed.
    {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }
        struct QuitMessage; // unit struct
        struct MoveMessage {
            x: i32,
            y: i32,
        }
        struct WriteMessage(String); // tuple struct
        struct ChangeColorMessage(i32, i32, i32); // tuple struct
    }

    // There is one similarity between struct and enum that they both can have member functions inside the impl or implementation block.
    {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }
        impl Message {
            fn call(&self) {

            }
        }

        let m = Message::Write(String::from("hello"));
        m.call();
    }

    // Option Enum
    // It is a direct alternative to null, instead of having null pointer which is either someting or nothing. Rust does not have null values but has Option<T>, where None represents the absense of a value and Some represent some data. To keep the program same if the value is None we have to handle it in a way so our program does not crash and also program does not lead to having bugs.
    {
        enum Option<T> {
            None,
            Some(T)
        }

        let some_number = Some(5);
        let some_char = Some('e');
        // let absent_number: Option<i32> = None; This line has error because it in None and we have not handled that None.
    }
}
