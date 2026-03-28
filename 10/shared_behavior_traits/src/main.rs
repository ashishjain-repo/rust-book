fn main() {
    {
        // trait is like a signature which in other languages are interfaces. This define the interface method without the body just a signature and whoever utlize this trait can make their own body but by following the trait(signature).
        pub trait Summary {
            fn summarize(&self) -> String;
        }
        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String,
        }
        impl Summary for NewsArticle {
            fn summarize(&self) -> String {
                return format!("{}, by {} ({})", self.headline, self.author, self.location);
            }
        }
        pub struct SocialPost {
            pub username: String,
            pub content: String, 
            pub reply: bool,
            pub repost: bool
        }
        impl Summary for SocialPost {
            fn summarize(&self) -> String {
                return format!("{}: {}", self.username, self.content);
            }
        }
    }
}
