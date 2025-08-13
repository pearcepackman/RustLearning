use std::io;

fn addTask(){
    println!("Adding task!");
}

fn readAll(){
    println!("Reading all tasks!");
}

fn updateTask(){
    println!("Updating a task!");
}

fn deleteTask(){
    println!("Deleting a task!");
}

fn main() {
    /*let mut input: String = String::new(); // Create a string variable
    io::stdin() // Get the standard input stream
        .read_line(&mut input) // The read_line function reads data until it reaches a '\n' character
        .expect("Unable to read Stdin"); // In case the read operation fails, it panics with the given message

    println!("You entered: {}", input);*/

    println!("--------- To Do List ---------");
    println!("");
    println!("1. Add a task");
    println!("2. Read all tasks");
    println!("3. Update a task");
    println!("4. Delete a task");
    println!("");
    println!("Choice: ");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read input");
    
    println!("You chose #: {}", input);
    let choice: i32 = input.trim().parse().unwrap();
    
    if choice == 1{
        addTask();
    }
    else if choice == 2{
        readAll();
    }
    else if choice == 3{
        updateTask();
    }
    else if choice == 4{
        deleteTask();
    }
    else{
        println!("Whoops, not sure what you want!");
    }
}