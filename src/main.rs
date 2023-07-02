use chrono::{DateTime, NaiveDateTime, Utc, TimeZone};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};


fn main() {
    let file_path = "tasks.txt".to_string(); // Ruta del archivo de texto

    let mut task_list = TaskList::new(file_path.clone());

    // Cargar tareas existentes desde el archivo
    task_list.load_tasks_from_file();

    loop {
        println!("----- Task Manager -----");
        println!("1. Register User");
        println!("2. Add Task");
        println!("3. Print Tasks");
        println!("4. Complete Task");
        println!("5. Exit");
        println!("------------------------");
        println!("Enter your choice:");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse::<u32>() {
            Ok(choice) => {
                match choice {
                    1 => {
                        println!("Enter your name:");
                        let mut name = String::new();
                        std::io::stdin()
                            .read_line(&mut name)
                            .expect("Failed to read input");

                        println!("Enter your email:");
                        let mut email = String::new();
                        std::io::stdin()
                            .read_line(&mut email)
                            .expect("Failed to read input");

                        task_list.register_user(name.trim().to_string(), email.trim().to_string());
                        println!("User registered successfully!");
                    }
                    2 => {
                        if task_list.user.is_none() {
                            println!("Please register a user first!");
                            continue;
                        }

                        println!("Enter task description:");
                        let mut description = String::new();
                        std::io::stdin()
                            .read_line(&mut description)
                            .expect("Failed to read input");

                        let date_time = Utc::now(); // Puedes modificar esto según tus necesidades

                        task_list.add_task(description.trim().to_string(), date_time);
                    }
                    
                    3 => {
                        task_list.print_tasks();
                    }
                    4 => {
                        println!("Enter task ID to mark as completed:");
                        let mut input_id = String::new();
                        std::io::stdin()
                            .read_line(&mut input_id)
                            .expect("Failed to read input");

                        match input_id.trim().parse::<u32>() {
                            Ok(task_id) => {
                                task_list.complete_task(task_id).unwrap_or_else(|err| {
                                    println!("Error: {}", err);
                                });
                            }
                            Err(_) => {
                                println!("Invalid task ID");
                            }
                        }
                    }

                    5 => {
                        task_list.save_tasks_to_file();
                        println!("Adios :)");
                        break;
                    }
                    
                    _ => {
                        println!("Invalid choice");
                    }
                }
            }
            Err(_) => {
                println!("Invalid choice");
            }
        }
    }
}


struct User {
    name: String,
    email: String,
}

struct Task {
    id: u32,
    description: String,
    completed: bool,
    date_time: DateTime<Utc>,
}

struct TaskList {
    user: Option<User>,
    tasks: Vec<Task>,
    file_path: String,
}

impl TaskList {
    fn new(file_path: String) -> TaskList {
        TaskList {
            user: None,
            tasks: Vec::new(),
            file_path,
        }
    }

    fn register_user(&mut self, name: String, email: String) {
        let user = User { name, email };
        self.user = Some(user);
    }

    fn add_task(&mut self, description: String, date_time: DateTime<Utc>) {
        let id = self.tasks.len() as u32 + 1;
        let task = Task {
            id,
            description: description.clone(),
            completed: false,
            date_time,
        };
        self.tasks.push(task);
        self.save_tasks_to_file();
    }

    fn complete_task(&mut self, task_id: u32) -> Result<(), String> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.completed = true;
            self.save_tasks_to_file();
            Ok(())
        } else {
            Err("Task not found".to_string())
        }
    }

    fn load_tasks_from_file(&mut self) {
        let file = match OpenOptions::new().read(true).open(&self.file_path) {
            Ok(file) => file,
            Err(_) => return, // No se puede abrir el archivo, no hay tareas para cargar
        };

        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(task_str) = line {
                let task_parts: Vec<&str> = task_str.split(';').collect();
                if task_parts.len() == 4 {
                    if let (Ok(id), Ok(completed)) = (task_parts[0].parse::<u32>(), task_parts[3].parse::<bool>()) {
                        let date_time = match NaiveDateTime::parse_from_str(task_parts[2], "%Y-%m-%dT%H:%M:%S%.f") {
                            Ok(parsed) => Utc.from_local_datetime(&parsed).single().unwrap_or_else(|| Utc::now()),
                            Err(_) => Utc::now(),
                        };

                        let task = Task {
                            id,
                            description: task_parts[1].to_string(),
                            completed,
                            date_time,
                        };
                        self.tasks.push(task);
                    }
                }
            }
        }
    }

    fn save_tasks_to_file(&self) {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.file_path)
            .unwrap();

        for task in &self.tasks {
            let task_str = format!(
                "{};{};{};{}\n",
                task.id,
                task.description,
                task.date_time.to_rfc3339(),
                task.completed
            );
            file.write_all(task_str.as_bytes()).unwrap();
        }
    }

    fn print_tasks(&self) {
        println!("Tasks:");
        for task in &self.tasks {
            println!("ID: {}", task.id);
            println!("Description: {}", task.description);
            println!("Completed: {}", task.completed);
            println!("Date and Time: {}", task.date_time.to_rfc3339());
            if let Some(user) = &self.user {
                println!("User Name: {}", user.name);
                println!("User Email: {}", user.email);
            }
            println!("----------------------");
        }
    }
}
