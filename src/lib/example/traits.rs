pub fn load() {
    let s = Spring {
        sysbol: String::from("this is spring"),
    };
    s.summar();
}

// 定义一个接口
pub trait Summer {
    fn summar(&self);
}

pub struct Spring {
    pub sysbol: String,
}

// 实现一个接口
impl Summer for Spring {
    fn summar(&self) {
        println!("spring start :{}", &self.sysbol)
    }
}
