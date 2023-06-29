use chrono::{DateTime, Utc};

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
}
impl ListaDeTareas{
    fn new(usuario: Usuario) -> ListaDeTareas {
        ListaDeTareas {
            usuario,
            tareas: Vec::new(),
        }
    }


    fn agregar_tarea(&mut self, descripcion: String, fecha_hora: DateTime<Utc>){
        let id = self.tareas.len() as u32 + 1;
        let tarea = Tarea {
            descripcion,
            fecha_hora,
            completada: false,
        };
        self.tareas.push(tarea);
    }

    fn completar_tarea(&mut self, id_tarea: u32) {
        if let Some(tarea) = self.tareas.iter_mut().find(|t| t.id == id_tarea) {
            tarea.completeda = true;
            Ok(())
        } else {
            Err("Tarea no encontrada".to_string())
        }
    }
}



fn main() {
    
}
