
use std::io;
fn main() {

    //how to print to the screen
    println!("Hello, world!");

   // different ways to make a variable, immutable by default
    // let  v1 = 1;
    // let v2 = (2);
    // let v3 = {3};
    // let v4 = {
    //     let c = 12;
      //  last expression is returned to v3
    //     c + 1
    // };

    //different ways to print a variable
    // println!("{} {} {} {}",v1,v2,v3,v4); //normal
    // println!("{2} {0} {3} {1}", v2, v4, v1, v3); //using position
    // println!("{a} {b} {c} {d}", b = v2, a = v1, c = v3, d = v4); //using tmp variables
    // println!("{:?} {:?} {:?} {:?}",v1,v2,v3,v4); //using debug mode

//    //variable types
//     let a: i32 = -12; //signed 32 bit integer 
//     let b: u64 = 5; //unsigned 64 bit integer
//     let s = "hello world"; //we don't have to specify a type
//     let s: &str = "hello"; //string slice, immutable 
//     let mut st: String = "hello_world".to_string(); //dynamic string, similar to a vector or arraylist
//     st.push('a');
//     st.pop();

    //mutable
    // let var = 10;
    // var = 20;
    // print!("{} ", var);

    //shadowing 
    // let s = s.len();
    // println!("{}",s);

    //constants can't be shadowed, can never change, need type
    //   const a: i32 = 10;
   // const a: i32 = 20;
    //   println!("{}",a);

    //immutable can be shadowed 
    // let b = 10;
    // let b = 20;
    // println!("{}",b);



    //if else if else
    // let  b = 1;

    // if b == 1{
    //     print!("{} is equal to 1", b);
    // }else if b > 1{
    //     print!("{} is greater than 1", b);
    // }else if b < 1{
    //     print!("{} is less than 1", b);
    // }else if b >= 1{
    //     print!("{} is greater/equal to 1", b);
    // }else if b <= 1{
    //     print!("{} is less/equal to 1", b);
    // }else if b > 10 && b < 12{
    //     println!("{} is eleven", b)
    // }else if b < 0 || b > 0{
    //     print!("{} it's a number",b);
    // }else{
    //     print!("{} it's something",b)
    // }


  //  ----redo the match video ---
    //match, similar to switch in other languages 
    // let b = 5;

    // match b{
    //     10 =>{
    //         println!("{}",b);
    //     }

    //     2 | 3 | 5 | 7 =>{
    //         println!("{} is prime", b);
    //     }

    //     _ =>{
    //         println!("{} default",b); //will run if nothing else does
    //     }
    // }

    //array
    // let array = [1,2,3,5,6];
    // let array2: [f32; 3] = [2.5,1.3,5.3];
    // println!("{} {:?}",array[0], array2);

    //tuple
    // let tuple = (2.6, "hi", 25, false);
    // let tuple2: (i32, bool, &str) = (1,true,"hello");
    // println!("{:?} {} {:?}", tuple2, tuple.1, tuple);



    //loops 

    // let mut counter = 0;

    // let result = loop{
    //   counter += 1;
    //   println!("{}", counter);
    //   if counter == 1000 {
    //     break counter * 5;
    //   }
    // };

    // println!("{}", result);

    //while loop 
    // let mut counter = 1000;
    // while counter != 0{
    //   println!("{}", counter);
    //   counter -= 1;
    // }

    //for loop, Iterators 
    // let array = [1,2,4,6,8,9,19,29];
    // for element in array.iter(){
    //   println!("{}",element);
    // }

    //for loop in reverse
    // for element in (1..10).rev(){
    //   println!("{}", element);
    // }

    // let mut line = String::new();
    // match io::stdin().read_line(&mut line){
    //   Ok(b)=>{
    //     println!("{}", line);
    //   }

    //   Err(e) => {
    //     println!("{}", e);
    //   }
      
    // }


    //ownership 
//      let a = 7;
//      let mut b = a;
//      b = b + 2;
//      println!("{} {}",a,b);


//     // heap, transfer happens
//     let mut vec = Vec::new();
//     vec.push(1);
//     vec.push(123.0 as i32);
//    // println!("{:?}",vec);

//    //will cause a move, vec is now unusable 
//     let vec2 = vec.clone();

//   //  print collection
//     print!("{:?} {:?} ", vec,vec2);



//borrowing 

// let mut one = 12;

// //mutable borrow
// let two = &mut one;
// *two = 8;

// println!("{}",two);
// println!("{}",one);


//more borrowing 
//    let mut one = String::from("hi");
//    let mut two = &mut one;

//    two.push('b');
//    //one.push('c');
//    println!("{}", one);
 
  // two.push('c');



  //functions that can modify the data
//   let mut star = String::from("add star");

//   add_star(&mut star);

//   println!("{}",star);


  //same thing can be done with primitives
//   let mut one = 1;
//   let mut c = add_one(&mut one);

//   println!("{}",one);


    //struct creation 
    // let s = Student{
    //     name: "bob".to_string(),
    //     age: 20,
    //     gpa: 4.0,
    //     major: "it".to_string()
    // };

    // println!("{} {} {} {}",s.name, s.age, s.gpa, s.major);


    //struct creation with new
    // let mut student = Student::new();
    // student.age = 10; 
    // student.major = "networking".to_string();
    // student.print_student();
    
    // println!("{}",student.get_major());

    // // //coping properties over
    // let student2 = Student { 
    //     name: "billy".to_string(), 
    //     ..student 
    // };
    // student2.print_student();
    // student2.get_major();


    //let red = Color(0,1,0);


    // let red = Colors::Red;
    // let blue = Colors::Blue;

    // println!("{:?} {:?}",red,blue);

}


//borrows a mutbale stirng 
// fn add_star(star: &mut String){
//     star.push('*');
// }

//borrows a mutable int
// fn add_one(one: &mut i32) -> i32{
//     //* will dereference the variable so we can access it's data
//     *one = *one + 1;
//     return *one;
// }

// #[derive(Debug)]
// enum Colors{
//     Red,
//     Green,
//     Blue
// }


//c style struct 
// struct Student{
//     name: String,
//     age: i32,
//     gpa: f32,
//     major: String
// }


//tuple struct
// struct Color(i32,i32,i32);






//impl
// impl Student{
//     fn new() -> Self{
//         Self{
//             name: "default".to_string(),
//             gpa: 4.0,
//             age: 20,
//             major: "it".to_string()
//         }
//     }


//     fn print_student(&self){
//         println!("Name: {}  Age: {}  GPA: {}  Major: {}",self.name, self.age, self.gpa, self.major);
//     }
// }


//trait, works like an interface
// trait StudentMajor{
//     fn get_major(&self) -> &String;
// }

// impl StudentMajor for Student{
//     fn get_major(&self) -> &String{
//           &self.major
//     }
// }

