struct Todo {
    id: u32,
    content: String,
    completed: bool,
}

fn main() {
    let mut todos: Vec<Todo> = Vec::new();
    let mut next_id = 2;

    todos.push(Todo {
        id: (1),
        content: ("hello".to_string()),
        completed: (false),
    });

    loop {
        println!("\n --- --- \n");

        for todo in &todos {
            println!(
                "[{}] {} - {}",
                todo.id,
                todo.content,
                if todo.completed {
                    "Completed"
                } else {
                    "Not completed"
                }
            )
        }

        println!("\n --- --- \n");

        println!("[e] for edit todo.");
        println!("[a] add todo.");

        let mut mode = String::new();

        get_input(&mut mode);

        clear_terminal();

        match mode.trim() {
            "a" => add_mode(&mut todos, &mut next_id),
            "e" => edit_mode(&mut todos),
            _ => {
                println!("Unexpected input.\n")
            }
        }
    }
}

fn get_input(variable: &mut String) {
    std::io::stdin()
        .read_line(variable)
        .expect("Cannot read input.");
}

fn clear_terminal() {
    std::process::Command::new("clear").status().unwrap();
}

fn add_mode(todos: &mut Vec<Todo>, next_id: &mut u32) {
    clear_terminal();

    let mut content = String::new();

    println!("Write todo :");

    get_input(&mut content);

    todos.push(Todo {
        id: (*next_id),
        content,
        completed: (false),
    });

    *next_id += 1;
}

fn edit_mode(todos: &mut Vec<Todo>) {
    println!("Select todo :");

    let mut selected_todo = String::new();

    get_input(&mut selected_todo);
    clear_terminal();

    let selected_todo: usize = match selected_todo.trim().parse() {
        Ok(number) if number > 0 => number,
        _ => {
            println!("Unexpected input.");
            return;
        }
    };

    match todos.get_mut(selected_todo - 1) {
        Some(todo) => {
            println!(
                "{} - {}",
                todo.content,
                if todo.completed {
                    "Completed"
                } else {
                    "Not completed"
                }
            );

            let mut mode = String::new();

            println!("[c] for switch complete state.");
            println!("[d] for delete.");

            get_input(&mut mode);

            clear_terminal();

            match mode.trim() {
                "c" => {
                    todo.completed = !todo.completed;
                }
                "d" => {
                    todos.remove(selected_todo - 1);
                }
                _ => {
                    println!("Unexpected input.");
                }
            }
        }
        None => {
            println!("Can't find todo.");
        }
    }
}
