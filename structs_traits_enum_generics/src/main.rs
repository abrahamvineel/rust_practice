
struct Person {
    name: String,
    age: i32,
    gender: char,
    salary: i32,
    citizenship: String
}

impl Person {
    fn new() -> Person {
        Self {
            name: String::from("Hello"),
            age: 40,
            gender: 'M',
            salary: 112_789,
            citizenship: "".to_string(),
        }
    }

    fn compute_taxes(&self) -> f32 {
        (self.salary as f32 /3.) * 0.5
    }
}

struct Numbers(i32, i32);

impl Numbers {
    fn greater(&self) -> i32 {
        if self.0 > self.1 {
            self.0
        } else {
            self.1
        }
    }

    fn lesser(&self) -> i32 {
        if self.0 < self.1 {
            self.0
        } else {
            self.1
        }
    }
}

struct Student {
    name: String,
    age: i32,
    gender: char,
    citizenship: String
}

// trait GeneralInfo {
//     fn info(&self) -> (&str, u8, char);
//
//     fn country_info(&self) -> str;
// }

// impl GeneralInfo for Person {
//     fn info(&self) -> (&str, i32, char) {
//         (&self.name, self.age, self.gender)
//     }
//
//     fn country_info(&self) -> &str {
//         &self.citizenship
//     }
// }
//
// impl GeneralInfo for Student {
//     fn info(&self) -> (&str, i32, char) {
//         (&self.name, self.age, self.gender)
//     }
//
//     fn country_info(&self) -> &str {
//         &self.citizenship
//     }
// }

// fn main() {
//     let person1 = Person{
//         name: String::from("Hello"),
//         age: 40,
//         gender: 'M',
//         salary: 112_789,
//         citizenship: String::from("India")
//     };
//
//     println!("The values are {} {} {}", person1.name, person1.age, person1.gender);
//     println!("The taxes are {} {} ", person1.name, person1.compute_taxes());
//
//     let person2 = Person::new();
//     println!("The values are {} {} {}", person2.name, person2.age, person2.gender);
//     println!("The taxes are {} {} ", person2.name, person2.compute_taxes());
//
//
//     let person3 = Person {
//         age: 50,
//         name: String::from("abc"),
//         ..person1
//     };
//
//     println!("The values are {} {} {}", person3.name, person3.salary, person2.gender);
//
//
//     let mut person4 = Person::new();
//     println!("The values are {} {} {}", person4.name, person4.salary, person4.gender);
//     person4.name = String::from("asjkdh");
//     println!("The values are {} {} {}", person4.name, person4.salary, person4.gender);
//
//     let some_numbers = Numbers(32, 16);
//     println!("The values are {} {} ", some_numbers.0, some_numbers.1);
//     println!("The value is {} ", some_numbers.greater());
//
//
//     let person1 = Person{
//         name: String::from("Hello"),
//         age: 40,
//         gender: 'M',
//         salary: 112_789,
//         citizenship: String::from("India")
//     };
//
//     let student1 = Student {
//         name: String::from("hello"),
//         age: 23,
//         gender: 'M',
//         citizenship: String::from("India")
//     };
// }


// struct Circle {
//     radius: f32
// }
//
// struct Rectangle {
//     length: f32,
//     width: f32
// }
//
// trait GeneralInfo {
//     fn area(&self) {
//         println!("No implementation exists");
//     }
//
//     fn perimeter(&self);
// }
//
// impl GeneralInfo for Circle {
//     // fn area(&self) {
//     //     let area = 3.14 * (self.radius * self.radius);
//     //     println!("The area of the circle is {} ", area);
//     // }
//
//     fn perimeter(&self) {
//         let circumference = 2.0 * 3.14 * self.radius;
//         println!("The circumference of the circle is {}", circumference);
//     }
// }
//
//
// impl GeneralInfo for Rectangle {
//     fn area(&self) {
//         let area = (self.length * self.width);
//         println!("The area of the rect is {} ", area);
//     }
//
//     fn perimeter(&self) {
//         let circumference = 2.0 * (self.length + self.width);
//         println!("The circumference of the rec is {}", circumference);
//     }
// }
//
// fn main() {
//     let circle = Circle {
//         radius: 3.0
//     };
//
//     let rect = Rectangle {
//         length: 10.0,
//         width: 10.0
//     };
//
//     circle.area();
//     circle.perimeter();
//
//     rect.area();
//     rect.perimeter();
// }

// struct Data {
//     some_data: Vec<i32>,
// }
//
// trait BasicStats {
//     fn mean(&self) -> f32;
//     fn variance(&self) -> f32;
// }
//
// impl BasicStats for Data {
//     fn mean(&self) -> f32 {
//         let mut sum = 0;
//         for i in self.some_data.iter() {
//             sum += *i;
//         }
//         sum as f32 / self.some_data.len() as f32
//     }
//
//     fn variance(&self) -> f32 {
//         let mu = self.mean();
//         let mut sum_squared_diff = 0.0;
//         for i in self.some_data.iter() {
//             sum_squared_diff += ( *i as f32 - mu) * (*i as f32 - mu);
//         }
//         sum_squared_diff / self.some_data.len() as f32
//     }
// }
//
// fn main() {
//     let my_data = Data {
//         some_data: vec![5,6,7,8,9],
//     };
//
//     println!("The mean is {}", my_data.mean());
//     println!("The mean is {}", my_data.variance());
// }


//Enums

// enum Conveyance {
//     Car(i32),
//     Train(i32),
//     Air(i32),
// }
//
// impl Conveyance {
//     fn travel_allowance(&self) -> f32 {
//         let allowance = match self {
//             Conveyance::Car(miles) => *miles as f32 * 14.0 * 2.0,
//             Conveyance::Train(miles) => *miles as f32 * 18.0 * 2.0,
//             Conveyance::Air(miles) => *miles as f32 * 35.0 * 2.0,
//         };
//         allowance
//     }
// }
//
// fn main() {
//     let participant_1 = Conveyance::Car(60);
//     let par_2 = Conveyance::Train(120);
//     let par_3 = Conveyance::Air(50);
//     // println!("the value of the option is {}", participant_1 as i32);
//
//     println!("The participant 1 has travel allowance of {} ", participant_1.travel_allowance());
// }

// #[derive(Debug)]
// enum Value {
//     Integer(i32),
//     Float(f32),
// }
//
// fn main() {
//     let val = vec![Value::Integer(12), Value::Float(25.4)];
//     println!("The value of the integer is {:?} and the float is {:?} ", val[0], val[1]);
//
//     for i in val.iter() {
//         match i {
//             Value::Integer(num) => println!("The value of the integer is {}", num),
//             Value::Float(num) => println!("The value of the float is {}", num)
//         }
//     }
// }

// fn squarei32(x:i32) -> i32 {
//     x*x
// }
//
// fn squaref32(x:f32) -> f32 {
//     x*x
// }
//
// fn square<T>(x:T) -> T
//     where T: std::ops::Mul<Output = T> + Copy + std::ops::Add {
//     x+x;
//     x*x
// }
//
// fn main() {
//     println!("The square of the number is {}", square(3));
//     println!("The square of the number is {}", square(7.2));
// }

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U>
where T: std::fmt::Debug, U: std::fmt::Debug {
    fn printing(&self) {
        println!("The value of the points {:?}, {:?}", self.x, self.y);
    }
}
fn main() {
    let p1 = Point {x: 5, y : 5};
    let p2 = Point {x: 1.0, y: 4.0};
    let p3 = Point {x: 5, y: 5.0};
}
