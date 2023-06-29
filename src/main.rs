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
        
    }

    fn completar_tarea(&mut self, id_tarea: u32) {
        if let Some(tarea) = self.tareas.iter_mut().find(|t| t.id == id_tarea) {
            tarea.completada = true;
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

    fn agregar_tarea_txt(){
        
    }

    
}



fn main() {
    
}
