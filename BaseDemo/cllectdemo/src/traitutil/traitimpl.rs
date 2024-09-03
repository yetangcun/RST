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

pub struct Pair<T,T1> {
    x: T,
    y: T1
}
impl<T,T1> Pair<T,T1> {
    pub fn news(x: T, y: T1) -> Self {
        Self { x, y }
    }
}
pub fn trait_fn7<T,T1> (na_obj:T, twt_obj:T1) 
where T:Summary, T1:Summary 
{
    // let na_obj = NewsArticle {
    //     headline:String::from("hello"),
    //     location:String::from("beijing"),
    //     author:String::from("tom"),
    //     content:String::from("hello world!")
    // };

    
    // let twt_obj = Tweet {
    //     username:String::from("tim "),
    //     content:String::from("welcome!"),
    //     reply:false,
    //     retweet:false
    // };

    let res_obj = Pair::<T,T1>::news(na_obj, twt_obj);

    println!("Haha, {}",res_obj.x.summarize());
}