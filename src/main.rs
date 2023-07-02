use chrono::{DateTime, TimeZone, NaiveDateTime, Local};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let file_path = "tasks.txt".to_string(); // Ruta del archivo de texto

    let mut task_list = TaskList::new(file_path.clone());

    
    task_list.load_tasks_from_file();

    loop {
        println!("------------------------");
        println!("📕📕📕 Mis Tareas📕📕📕");
        println!("1️. Resgistrar usuario");
        println!("2️. Añadir Tarea");
        println!("3️. Mostrar Tareas");
        println!("4️. Completar Tarea");
        println!("5️. Limpiar Tareas");
        println!("6️. Exit");
        println!("------------------------");
        println!("Ingrese la opcion: ");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Error de lectura");

        match input.trim().parse::<u32>() {
            Ok(choice) => {
                match choice {
                    1 => {
                        println!("Ingrese su nombre: ");
                        let mut name = String::new();
                        std::io::stdin()
                            .read_line(&mut name)
                            .expect("Error de lectura");

                        println!("Ingrese su email: ");
                        let mut email = String::new();
                        std::io::stdin()
                            .read_line(&mut email)
                            .expect("Error de lectura");

                        task_list.register_user(name.trim().to_string(), email.trim().to_string());
                        println!("Usuario registrado exitosamente 👍:)");
                    }
                    2 => {
                        if task_list.user.is_none() {
                            println!("Registrar usuario");
                            continue;
                        }

                        println!("Ingrese la descripcion de la tarea");
                        let mut description = String::new();
                        std::io::stdin()
                            .read_line(&mut description)
                            .expect("Error de lectura");

                         

                        task_list.add_task(description.trim().to_string());
                    
                    }
                    3 => {
                        task_list.print_tasks();
                    }
                    4 => {
                        println!("Ingrese la ID de la Tarea");
                        let mut task_id_input = String::new();
                        std::io::stdin()
                            .read_line(&mut task_id_input)
                            .expect("Error de lectura");

                        let task_id = match task_id_input.trim().parse::<u32>() {
                            Ok(id) => id,
                            Err(_) => {
                                println!("ID invalida. Por favor ingrese nuevamente.");
                                continue;
                            }
                        };

                        match task_list.complete_task(task_id) {
                            Ok(_) => println!("Tarea completada 👌👌👌."),
                            Err(err) => println!("Error: {}", err),
                        }
                    }

                    5 => {
                        task_list.clear_tasks();
                    }
                    6 => {
                        task_list.save_tasks_to_file();
                        println!("Adios, vuelva pronto 👋👋👋");
                        break;
                    }
                    _ => {
                        println!("Opcion incorrecta, ingrese una opcion valida.");
                    }
                }
            }
            Err(_) => {
                println!("Opcion incorrecta, ingrese una opcion valida.");
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
    date_time: DateTime<Local>,
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


    fn add_task(&mut self, description: String) {
        let id = (self.tasks.len() + 1) as u32;
        let task = Task {
            id,
            description,
            completed: false,
            date_time: Local::now(),
        };
        self.tasks.push(task);
        println!("Task added successfully!");
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
                            Ok(parsed) => Local.from_local_datetime(&parsed).single().unwrap_or_else(|| Local::now()),
                            Err(_) => Local::now(),
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

    fn clear_tasks(&mut self) {
        self.tasks.clear();
        println!("All tasks have been cleared.");
    }
}
