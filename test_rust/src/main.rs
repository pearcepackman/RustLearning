use std::io;
use std::collections::HashMap;

fn add_task(tasks: &mut HashMap<i32, String>, task_num: &mut i32){
    println!("Adding task!");
    let mut task: String = String::new();
    io::stdin()
        .read_line(&mut task)
        .expect("Unable to read Task");
    tasks.insert(task_num, task);

}

fn read_all(){
    println!("Reading all tasks!");
}

fn update_task(){
    println!("Updating a task!");
}

fn delete_task(){
    println!("Deleting a task!");
}

fn main() {
    /*let mut input: String = String::new(); // Create a string variable
    io::stdin() // Get the standard input stream
        .read_line(&mut input) // The read_line function reads data until it reaches a '\n' character
        .expect("Unable to read Stdin"); // In case the read operation fails, it panics with the given message

    println!("You entered: {}", input);*/

    //Main menu
    println!("--------- To Do List ---------");
    println!("");
    println!("1. Add a task");
    println!("2. Read all tasks");
    println!("3. Update a task");
    println!("4. Delete a task");
    println!("");
    println!("Choice: ");
    let mut input: String = String::new();
    //Get line from user
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read input");
    
    println!("You chose #: {}", input);
    //Make the choice an integer
    let choice: i32 = input.trim().parse().unwrap();

    let mut tasks = HashMap::<i32, String>::new();
    let task_num: i32 = 1;
    tasks.insert(task_num, input);
    for(task_num, input) in & tasks{
        println!("{task_num}, {input}")
    }

    if choice == 1{
        add_task(&mut tasks, &mut task_num);
        main();
    }
    else if choice == 2{
        read_all();
    }
    else if choice == 3{
        update_task();
    }
    else if choice == 4{
        delete_task();
    }
    else if choice == 5{
        main();
    }
    else{
        println!("Whoops, not sure what you want!");
    }
}