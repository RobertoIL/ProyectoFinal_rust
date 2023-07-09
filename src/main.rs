use chrono::{DateTime, TimeZone, NaiveDateTime, Local};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let file_path = "tareas.txt".to_string(); // Ruta del archivo de texto

    let mut lisa_tareas = ListaTarea::new(file_path.clone());

    
    lisa_tareas.cargar_tareas_txt();

    loop {
        println!("---------------------------------------------");
        println!("📕📕📕 Gestor de Tareas📕📕📕");
        println!("1️. Registrar usuario");
        println!("2️. Añadir Tarea");
        println!("3️. Mostrar Tareas");
        println!("4️. Completar Tarea");
        println!("5️. Limpiar Tareas");
        println!("6️. Exit");
        println!("---------------------------------------------");
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

                        lisa_tareas.registrar_usuario(name.trim().to_string(), email.trim().to_string());
                        println!("Usuario registrado exitosamente 👍:)");
                    }
                    2 => {
                        if lisa_tareas.usuario.is_none() {
                            println!("Registrar usuario");
                            continue;
                        }

                        println!("Ingrese la descripcion de la tarea");
                        let mut description = String::new();
                        std::io::stdin()
                            .read_line(&mut description)
                            .expect("Error de lectura");

                         

                        lisa_tareas.añadir_tarea(description.trim().to_string());
                    
                    }
                    3 => {
                        lisa_tareas.mostrar_tareas();
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

                        match lisa_tareas.completar_tarea(task_id) {
                            Ok(_) => println!("Tarea completada 👌👌👌."),
                            Err(err) => println!("Error: {}", err),
                        }
                    }

                    5 => {
                        lisa_tareas.borrar_tareas();
                    }
                    6 => {
                        lisa_tareas.guardar_tareas_txt();
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



struct Usuario {
    nombre: String,
    email: String,
}


struct Tarea {
    id: u32,
    descripcion: String,
    completada: bool,
    date_time: DateTime<Local>,
}

struct ListaTarea {
    usuario: Option<Usuario>,
    tareas: Vec<Tarea>,
    file_path: String,
}

impl ListaTarea {
    fn new(file_path: String) -> ListaTarea {
        ListaTarea {
            usuario: None,
            tareas: Vec::new(),
            file_path,
        }
    }

    fn registrar_usuario(&mut self, nombre: String, email: String) {
        let usuario = Usuario { nombre, email };
        self.usuario = Some(usuario);
    }


    fn añadir_tarea(&mut self, descripcion: String) {
        let id = (self.tareas.len() + 1) as u32;
        let tarea = Tarea {
            id,
            descripcion,
            completada: false,
            date_time: Local::now(),
        };
        self.tareas.push(tarea);
        println!("Tarea añadida correctamente!");
    }

    fn completar_tarea(&mut self, id_tarea: u32) -> Result<(), String> {
        if let Some(tarea) = self.tareas.iter_mut().find(|t| t.id == id_tarea) {
            tarea.completada = true;
            self.guardar_tareas_txt();
            Ok(())
        } else {
            Err("Tarea no encontrada".to_string())
        }
    }

    fn cargar_tareas_txt(&mut self) {
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

                        let tarea = Tarea {
                            id,
                            descripcion: task_parts[1].to_string(),
                            completada: completed,
                            date_time,
                        };
                        self.tareas.push(tarea);
                    }
                }
            }
        }
    }
    fn guardar_tareas_txt(&self) {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.file_path)
            .unwrap();

        for tarea in &self.tareas {
            let task_str = format!(
                "{};{};{};{}\n",
                tarea.id,
                tarea.descripcion,
                tarea.date_time.to_rfc3339(),
                tarea.completada
            );
            file.write_all(task_str.as_bytes()).unwrap();
        }
    }

    fn mostrar_tareas(&self) {
        println!("Mis Tareas:");
        for tarea in &self.tareas {
            println!("ID: {}", tarea.id);
            println!("Descripcion: {}", tarea.descripcion);
            println!("Completada: {}", tarea.completada);
            println!("Fecha y hora: {}", tarea.date_time.to_rfc3339());
            if let Some(user) = &self.usuario {
                println!("Nombre usuario: {}", user.nombre);
                println!("email: {}", user.email);
            }
            println!("----------------------");
        }
    }

    fn borrar_tareas(&mut self) {
        self.tareas.clear();
        println!("Todas las tareas han sido eliminadas.");
    }
}
