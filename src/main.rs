use chrono::{DateTime, Utc};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};

struct Usuario{
    nombre: String,
    email: String,

}

struct Tarea{
    id: u32,
    descripcion: String,
    fecha_hora: DateTime<Utc>,
    completada: bool,
}

struct ListaDeTareas{
    usuario: Usuario,
    tareas: Vec<Tarea>,
    file_path: String,
}
impl ListaDeTareas{
    fn new(usuario: Usuario) -> ListaDeTareas {
        ListaDeTareas {
            usuario,
            tareas: Vec::new(),
            file_path,
        }
    }


    fn agregar_tarea(&mut self, descripcion: String, fecha_hora: DateTime<Utc>){
        let id = self.tareas.len() as u32 + 1;
        let tarea = Tarea {
            id,
            descripcion: descripcion.clone(),
            completada: false,
            fecha_hora,
        };
        self.tareas.push(tarea);
        self.guardar_tarea_txt();
        
    }

    fn completar_tarea(&mut self, id_tarea: u32) {
        if let Some(tarea) = self.tareas.iter_mut().find(|t| t.id == id_tarea) {
            tarea.completada = true;
            self.guardar_tarea_txt();
            Ok(());
        } else {
            Err("Tarea no encontrada".to_string());
        }
    }

    fn imprimir_tareas(&self){
        println!("");
        for tarea in &self.tareas {
            let status = if tarea.completada {"[X]"} else {"[ ]"};
            println!(
                "{} Tarea {}: {} - Fecha y hora: {}",
                status, tarea.id, tarea.descripcion, tarea.fecha_hora
            );
        }
    }

    fn guardar_tarea_txt(&self){
        let mut file: OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.file_path)
            .unwrap();

        for tarea in &self.tareas{
            let tarea_str = format!(
                "{};{};{};{}\n",
                tarea.id,
                tarea.descripcion,
                tarea.fecha_hora.to_rfc3339(),
                tarea.completada
            );
            file.write_all(tarea_str.as_bytes()).unwrap();

        }

    }

    
}



fn main() {
    println!("hola mundo");
}
