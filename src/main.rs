fn main() {
    let tweet = Tweet {
        username: String::from("the_naveen_sharma"),
        content: String::from("Follow me to learn rust"),
        reply: false,
        retweet: false,
    };

    print!("1 new tweet {}", tweet.summarise());
}

pub trait Summary {
    fn summarise(&self) -> String;
}
pub struct NewsArtical {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArtical {
    fn summarise(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarise(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

type Meters = u32;
impl Summary for Meters {
    fn summarise(&self) -> String {
        format!("{}", self)
    }
}
