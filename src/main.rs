use std::collections::HashMap;

fn main() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let mut rect2 = Rectangle {
        width: 200,
        height: 200,
    };

    let square_1 = Rectangle::create_square(50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let team_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(team_scores.into_iter()).collect();

    for val in scores.values() {
        println!("{}", val);
    }

    scores.insert(String::from("Blues"), 10);
    scores.insert(String::from("Reds"), 20);

    let yellow_score = scores.get("Yellow");

    match yellow_score {
        Some(50) => println!("Thats 50 Goals"),
        _ => (),
    }

    let mut new_user = User {
        email: String::from("example@gmail.com"),
        username: String::from("Test_User"),
        active: true,
        sign_in_count: 1,
    };

    let new_user_2 = set_up_user(
        String::from("Andrew"),
        String::from("admin@allenair.net.au"),
    );

    let black = Colour(0, 0, 0);
    let red = Colour(255, 0, 0);

    println!("The Rectangle Area is {:?}", rect1.area());
    println!("The User's  Name Is {} Their Email Is {} Is Their Account Active? {} & Their Sign In Count Is {} Times",
              new_user.username,new_user.email,new_user.active,new_user.sign_in_count);

    new_user.email = String::from("NEWTEST@GMAIL.COM");
    println!("The User's  Name Is {} Their Email Is {} Is Their Account Active? {} & Their Sign In Count Is {} Times",
              new_user.username,new_user.email,new_user.active,new_user.sign_in_count);

    println!("The User's  Name Is {} Their Email Is {} Is Their Account Active? {} & Their Sign In Count Is {} Times",
              new_user_2.username,new_user_2.email,new_user_2.active,new_user_2.sign_in_count);

    println!(
        "RGB Value Of Black Colour Is: R{}, B{}, G{}",
        black.0, black.1, black.2
    );
    println!(
        "RGB Value Of Red Colour Is: R{}, B{}, G{}",
        red.0, red.1, red.2
    );

    println!("W{}, H{} ", rect1.width, rect1.height);

    let answer: bool = rect1.can_contain_self(&mut rect2);

    println!("{}", &answer);

    println!(
        "The  Square Dimensions Are W{}, H{}",
        square_1.width, square_1.height
    );

    let v = vec![1, 2, 3];

    let mut v1 = Vec::new();

    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);

    let third: &i32 = &v[2];
    let ssc = vec![
        SpreadSheetCell::Name(String::from("Andy")),
        SpreadSheetCell::Age(43),
        SpreadSheetCell::JobRole(String::from("Manager")),
    ];

    println!("The value at the 3rd element of vector V is {}", third);

    println!("{:?} {:?} {:?}", ssc[0], ssc[1], ssc[2]);

    match v.get(2) {
        Some(third) => println!("The 3rd element is {}", third),
        None => println!("No such element"),
    }

    for i in &mut v1 {
        *i += 50;
        println!("{}", i);
    }

    while let Some(top) = v1.pop() {
        println!("{}", top);
    }
}

#[derive(Debug)] //Allows us to use the {:?} debug formatting option in println! as our struct doesnt define the Display formatting
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&mut self) -> u32 {
        self.width * self.height
    }

    fn can_contain_self(&self, rect: &mut Rectangle) -> bool {
        if self.width >= rect.width && self.height >= rect.height {
            println!("This Rectangle Can Contain The Passed In Rectangle");
            true
        } else {
            {
                println!("This Rectangle Cannot Contain The Passed In Rectangle");
                println!("The Area Of Passed In Rectangle Is {}", rect.area());
                false
            }
        }
    }

    fn create_square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn set_up_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u32,
    active: bool,
}

struct Colour(u32, u32, u32);

#[derive(Debug)]
enum SpreadSheetCell {
    Name(String),
    Age(u32),
    JobRole(String),
}
