pub fn load() {
    let s = Spring {
        sysbol: String::from("this is spring"),
    };
    s.summar();

    let d = Dog {
        name: String::from("dog"),
    };
    d.live();

    let t = Tree {
        name: String::from("this is tree"),
    };
    set_plant(&t);

    let g = Grass {
        name: String::from("this is a grass"),
    };
    set_plant(&g);
    two_args_unsample(&t, &g);
    two_args_sample(&g, &g);

    let test = TestTrait {
        name: String::from(""),
    };
    unsame_type(&test);
    unsame_type_t(&test);
    unsame_type_args(&test, &test);
    let res = unsame_type_where(&test, &test);
    res.live();
}

// 定义一个接口
pub trait Summer {
    fn summar(&self) {
        println!("last")
    }
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

// 定义一个默认方法的接口
// 注意默认的方法一定要加 self 的默认参数
pub trait Animal {
    fn live(&self) {
        println!("I am live")
    }
}

pub struct Dog {
    pub name: String,
}

impl Animal for Dog {}

// 一个默认函数，一个实现函数
pub trait Plant {
    fn grew(&self);
    fn bear(&self) {
        println!("I can bear fruit")
    }
}

pub struct Tree {
    pub name: String,
}

impl Plant for Tree {
    fn grew(&self) {
        println!("I am Tree and grewing:{}", &self.name)
    }
}

pub struct Grass {
    pub name: String,
}

impl Plant for Grass {
    fn grew(&self) {
        println!("I am grass and grewing");
    }
}

// 接口作为参数
fn set_plant(p: &impl Plant) {
    p.bear();
    p.grew();
}

// 两个都是接口参数，但是两个参数可以不一样的实现
fn two_args_unsample(arg1: &impl Plant, arg2: &impl Plant) {
    arg1.bear();
    arg2.bear();
}

// 两个都是接口参数，但是两个参数可以不一样的实现
fn two_args_sample<T: Plant>(arg1: &T, arg2: &T) {
    arg1.bear();
    arg2.bear();
}

pub struct TestTrait {
    pub name: String,
}

impl Plant for TestTrait {
    fn grew(&self) {
        println!("test_trait grew:{}", &self.name)
    }
}
impl Animal for TestTrait {}

// 不同类型的约束边界
// arg 必须同时实现 Plant 和 Animal
fn unsame_type(arg: &(impl Plant + Animal)) {
    arg.live()
}

fn unsame_type_t<T: Plant + Animal>(arg: &T) {
    arg.bear()
}

fn unsame_type_args<T: Plant + Animal, U: Plant + Animal>(t: &T, u: &U) {
    t.bear();
    u.bear();
}

// 定义多个类型，并反馈一个接口
fn unsame_type_where<T, U>(t: &T, u: &U) -> impl Animal
where
    T: Plant + Animal,
    U: Plant + Animal,
{
    t.bear();
    u.bear();
    println!("lots of type");
    Dog {
        name: String::from("I am dog"),
    }
}
