use std::fmt::Display;

// 定义声明trait-类似于C#中的接口
pub trait Summary {
    fn summarize(&self) -> String;

    fn tst_fn(&self) -> String {
        String::from("dft")
    }
}

// trait作为参数类型
pub fn trait_func(itm: &impl Summary) {
    println!("{}",itm.summarize());
}
pub fn trait_func2<T:Summary>(item:&T) {
    println!("{}", item.summarize());
}

// 一个类型实现多个trait
pub fn trait_fn3(itm:&(impl Summary+Display)) {
    println!("{}",itm.summarize());
}
pub fn trait_fn4<T:Summary+Display>(itm:&T) {
    println!("{}", itm.summarize());
}
pub fn trait_fn5<T>(itm:&T)
where T:Summary+Display {
    println!("fn 5: {}", itm.summarize());
}

pub fn trait_fn6() -> impl Summary {
    Tweet {
        username:String::from("Tom"),
        content:String::from("Hello, world!"),
        reply:false,
        retweet:false
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
impl Display for NewsArticle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}