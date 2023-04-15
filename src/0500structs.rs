struct Color(u8,u8,u8);
struct Point(i32,i32,i32);

struct MyEnum;

impl MyEnum{
    pub const A: i32 = 123;
    pub const B: i32 = 456;
}

const black:Color = Color(0,0,0);
const white:Color = Color(255,255,255);

struct Polygon{
    color:Color,
    point1:Point,
    point2:Point,
    point3:Point,
}

struct Object{
    name:String,
    shape:Vec<Polygon>,
}

type Scene = Vec<Object>;
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Rectangle{
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32{
    rectangle.height*rectangle.width
}

impl Rectangle{
fn implarea(&self) -> u32{
    self.height*self.width
}
}

fn main(){
    let car :Object = Object{
        name: String::from("Car"),
        shape: vec![Polygon{color:white, point1:Point(1, 1, 2), point2: Point(1, 1, 1), point3: Point(1, 1, 1)},
        Polygon{color:white, point1:Point(2,2,2), point2:Point(2,2,2), point3:Point(2,2,2)},
        Polygon{color:Color(126, 126, 126), point1:Point(2,2,2), point2:Point(2,2,2), point3:Point(2,2,2)}]
    };
    let house :Object = Object{
        name: String::from("House"),
        shape: vec![Polygon{color:white, point1:Point(1, 1, 2), point2: Point(1, 1, 1), point3: Point(1, 1, 1)},
        Polygon{color:white, point1:Point(2,2,2), point2:Point(2,2,2), point3:Point(2,2,2)},
        Polygon{color:Color(126, 126, 126), point1:Point(2,2,2), point2:Point(2,2,2), point3:Point(2,2,2)}]
    };

    let mut scene:Scene = vec!(car,house);

    let mut user1 = User {
        email: String::from("email@email.com"),
        username: String::from("username123"),
        active: true,
        sign_in_count: 1,
    };

    user1.sign_in_count = 2;

    println!("user {} has signed in {} time/s.", user1.username, user1.sign_in_count);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("the area of height {} times width {} is {}", rect1.height, rect1.width, area(&rect1));
    println!("the area of height {} times width {} is {}", rect1.height, rect1.width, rect1.implarea());

    println!{"MyEnum A is {}", MyEnum::A};
    


}
   
