//Rust code to display the menu of a restaurant
//and the food choice and the total of the food ordered by people.

use std::io;

fn main() {

    let mut name = String::new();
    println!("\nHello, please enter your name");
     io::stdin().read_line(&mut name).expect("Invalid input!");

     println!("Hello {} welcome to Students Center Restaurant! 
        This is our menu:", name);
        println!("(1)Poundo Yam/ Edinkaiko Soup.....N3,200");  
        println!("(2)Fried Rice & Chicken.....N3,000");
        println!("(3)Amala & Ewedu Soup.....N2,500");
        println!("(4)Eba & Egusi Soup.....N2,000");
        println!("(5)White Rice & Stew.....N2,500");
        
        let mut answer = String::new();
        println!("What else would you like to to order?");
        io::stdin().read_line(&mut answer).expect("Invalid input");
        let function: f64 = answer.trim().parse().expect("Invalid input");

        if function == 1;
        {
            let mut input1 = String::new(); 
            println!("How many portions of Poundo Yam/ Edinkaiko Soup would you like?");
            io::stdin().read_line(&mut input1).expect("Invalid input");
            let a :i32 = input1.trim().parse().expect("Invalid input");
        }

       else if function == 2;
        {
        let mut input2 = String::new();
        println!("How many portions of Fried Rice & Chicken would you like? ");
        io::stdin().read_line(&mut input2).expect("Invalid input");
        let a: i32 = input2.trim().parse().expect("Invalid input");
    }
    else if function == 3;
    {
        let mut input3 = String::new();
        println!("How many portions of Amala & Ewedu Soup would you like? ");
        io::stdin().read_line(&mut input3).expect("Invalid input");
        let a: i32 = input3.trim().parse().expect("Invalid input");
    }
    else if function == 4;
    {
        let mut input4 = String::new();
        println!("How many portions of Eba & Egusi Soup would you like? ");
        io::stdin().read_line(&mut input4).expect("Invalid input");
        let a: i32 = input4.trim().parse().expect("Invalid input");
    }
    else if function == 5;
    {
        let mut input5 = String::new();
        println!("How many portions of White Rice & Stew would you like? ");
        io::stdin().read_line(&mut input5).expect("Invalid input");
        let a: i32 = input5.trim().parse().expect("Invalid input");
    }

            let mut seconds = String::new();
            println!("Would you like to order anything else?");
            io::stdin().read_line(&mut seconds).expect("Invalid input");

            if seconds = yes;
            {
              let mut answer = String::new();
        println!("Which would you like to eat today?");
        io::stdin().read_line(&mut answer).expect("Invalid input");
        let function: f64 = answer.trim().parse().expect("Invalid input");
                }
        if function == 1.0
        {
            let mut input1 = String::new(); 
            println!("How many portions of Poundo Yam/ Edinkaiko Soup would you like?");
            io::stdin().read_line(&mut input1).expect("Invalid input");
            let a :i32 = input1.trim().parse().expect("Invalid input");
        }

       else if function == 2.0
        {
        let mut input2 = String::new();
        println!("How many portions of Fried Rice & Chicken would you like? ");
        io::stdin().read_line(&mut input2).expect("Invalid input");
        let a: i32 = input2.trim().parse().expect("Invalid input");
    }
    else if function == 3.0
    {
        let mut input3 = String::new();
        println!("How many portions of Amala & Ewedu Soup would you like? ");
        io::stdin().read_line(&mut input3).expect("Invalid input");
        let a: i32 = input3.trim().parse().expect("Invalid input");
    }
    else if function == 4.0ssss   {
        let mut input4 = String::new();
        println!("How many portions of Eba & Egusi Soup would you like? ");
        io::stdin().read_line(&mut input4).expect("Invalid input");
        let a: i32 = input4.trim().parse().expect("Invalid input");
    }
    else if function == 5;
    {
        let mut input5 = String::new();
        println!("How many portions of White Rice & Stew would you like? ");
        io::stdin().read_line(&mut input5).expect("Invalid input");
        let a: i32 = input5.trim().parse().expect("Invalid input");
    }
            else if seconds = no;
            {
            println!("Thank you for eating here!")
        }

   }