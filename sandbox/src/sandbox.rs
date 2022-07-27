use rand::prelude::*;

macro_rules! range {
    ($stop:expr) => {
        0..$stop
    };
    ($start:expr, $end:expr) => {
        $start..$end
    };
    ($start:expr, $end:expr, $step:expr) => {
        ($start..$end).step_by($step)
    };
}

pub struct Hello;

impl Hello {
    pub fn say() -> i32 {
        random()
    }
}

pub struct World;

impl World {
    pub fn say() {
        println!("World! {}", Hello::say());
    }
}

struct John;

struct Tim;

trait Say {
    fn say(&self);
}

impl Say for John {
    fn say(&self) {
        println!("Hi!");
    }
}

impl Say for Tim {
    fn say(&self) {
        println!("Hello!");
    }
}

struct Person {
    name: String,
    age: u32,
    job_status: JobStatus,
}

impl Person {
    fn new() -> Person {
        Person {
            name: String::new(),
            age: 0,
            job_status: JobStatus::Unemployed,
        }
    }

    fn say_name(&self) -> &Self {
        println!("Name is {}.", self.name);
        self
    }

    fn say_age(&self) -> &Self {
        println!("Age is {}.", self.age);
        self
    }

    fn say_job(&self) -> &Self {
        println!("Job status is {:?}", self.job_status);
        self
    }
}

#[derive(Debug)]
enum JobStatus {
    Unemployed,
    Employed,
}

fn intercept_func(code: i32) -> Result<i32, String> {
    println!("intercept_func {}", code);
    Ok(100)
}

fn error_handling(result: Result<i32, String>) -> Result<i32, String> {
    let code = result?;
    println!("{}", code);
    Ok(code)
}

pub struct Sandbox;

impl Sandbox {
    pub fn run(&self) {
        let p1 = Person {
            name: String::from("Sumire Uesaka"),
            age: 30,
            job_status: JobStatus::Employed,
        };
        let p2 = Person {
            name: String::from("Taro Yamada"),
            age: 42,
            job_status: JobStatus::Unemployed,
        };
        println!("{}({}) is {:?}", p1.name, p1.age, p1.job_status);
        println!("{}({}) is {:?}", p2.name, p2.age, p2.job_status);

        // Related Function called.
        let p3 = Person::new();
        // Method chain is implemented by Impl for struct.
        p3.say_name().say_age().say_job();

        let result: Result<i32, String> = Ok(200);
        match result {
            Ok(code) => println!("code: {}", code),
            Err(err) => println!("Err: {}", err),
        }

        let result2: Result<i32, String> = Ok(200);
        let result3: Result<i32, String> = Err("error".to_string());
        println!("{:?}", result2.unwrap_or(-1));
        println!("{:?}", result3.unwrap_or(-1));

        let result4: Result<i32, String> = Ok(200);
        let result5: Result<i32, String> = Err("error".to_string());
        let next_result = result4.and_then(intercept_func);
        let _ = error_handling(next_result);
        let _ = error_handling(result5);

        let vec1: Vec<i32> = range!(0, 40).collect();
        println!("{:?}", vec1);
        println!("{:?}", vec1.get(100));

        for (_, val) in vec1.iter().enumerate() {
            if val % 3 == 0 && val % 5 == 0 {
                println!("FizzBuzz {}", val);
            } else if val % 3 == 0 {
                println!("Fizz {}", val);
            } else if val % 5 == 0 {
                println!("Buzz {}", val);
            } else {
                println!("{}", val);
            }
        }

        // Box<T> is allocated on the heap.
        // The pointer is on the stack.
        // It is useful to place the unknown sided variables on the heap.
        let byte_array = [b'a', b'b', b'c'];
        self.print(Box::new(byte_array));

        for v in 0..10 {
            if v == 9 {
                println!("{}", v);
            } else {
                print!("{}", v);
            }
        }

        let words = String::from("Hello, world!");
        match words.find('o') {
            Some(index) => println!("o is in {}", index),
            None => println!("Not found"),
        }

        let unknown_val: i32 = Hello::say();
        let known_val = match unknown_val {
            12 => 144,
            _ => -1,
        };
        println!("{:?}", known_val);

        let sayings: Vec<Box<dyn Say>> = vec![Box::new(John), Box::new(Tim)];
        for item in sayings {
            item.say();
        }

        World::say();

        let x;
        let tmp;
        {
            x = 0;
            tmp = x;
        }
        println!("{}", tmp);

        let aa;
        let r;
        {
            // let r = 1;だとエラー
            r = 1;
            aa = self.ret0(&r);
        }
        println!("{}", aa);
    }

    fn ret0<'a>(&self, v: &'a i32) -> &'a i32 {
        &v
    }

    fn print(&self, s: Box<[u8]>) {
        println!("{:?}", s);
    }
}
