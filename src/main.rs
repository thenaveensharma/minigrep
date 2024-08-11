fn main() {
    let tweet = Tweet {
        username: String::from("the_naveen_sharma"),
        content: String::from("Follow me to learn rust"),
        reply: false,
        retweet: false,
    };

    print!("1 new tweet {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    notify(&article);
}

pub trait Summary {
    fn summarize_autor(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_autor())
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_autor(&self) -> String {
        todo!()
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_autor(&self) -> String {
        todo!()
    }
}

type Meters = u32;
impl Summary for Meters {
    fn summarize(&self) -> String {
        format!("{}", self)
    }

    fn summarize_autor(&self) -> String {
        todo!()
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
