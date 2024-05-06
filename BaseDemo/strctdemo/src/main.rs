fn main() {
    let mut usr1 = User {
        addr: String::from("yetangcun"),
        name: String::from("xiaoxiao"),
        age: 23
    };

    usr1.addr = String::from("yetangcun.");
    println!("Hello, {0}", usr1.name);
    println!("you're from {0}", usr1.addr);

    let usr2 = User {
        age: 24,
        ..usr1
    };
    println!("you're from the {0}", usr2.addr);
    
    let name = String::from("xiaoxiao");
    let addr = String::from("yetangcun007");
    let usr3 = User {
        age: 23,
        addr,
        name
    };
    println!("you're from the {0},{1}", usr3.addr, usr3.age);

    let prms = (15,16); // 元组
    let reslt = do_area(prms);
    println!("The area is: {reslt}");

    let hight=16;
    let wdth = 15;

    let rct = Rectgle {
        hight,
        wdth
    };

    // let rslt = do_areas(&rct);
    //println!("The last result is: {rslt}");

    let rslt1 = rct.do_areas();
    println!("The last result is: {rslt1}");

    dbg!(&rct);
e : {0},{1}",_rct.wdth,_rct.hight);
}

fn do_area (prms: (u32,u32)) -> u32 {
    prms.0 * prms.1
}

fn do_areas (rct: &Rectgle) -> u32 {
    rct.wdth * rct.hight
}


// 结构体定义
struct User {
    name: String,
    age:i32,
    addr: String
}

#[derive(Debug)]
    let hg = 19;
    let wd = 19;
    let other = Rectgle {
        wdth:wd,
        hight:hg
    };

    let rs1 = rct.do_cmpare(&other);
    let rs2 = other.do_cmpare(&rct);
    println!("the content are : {rs1},{rs2}");

    let _rct = Rectgle::sqe(27, 27);
    println!("the cnt ar
struct Rectgle {
    wdth: u32,
    hight: u32
}

impl Rectgle {
    fn do_areas(&self) -> u32 {
        self.wdth * self.hight
    }

    fn do_cmpare(&self, other:&Rectgle) -> bool {
        self.wdth>other.wdth && self.hight > other.hight
    }

    fn sqe (w:u32, h:u32) -> Self {
        Self {
            wdth:w,
            hight:h
        }
    }
}