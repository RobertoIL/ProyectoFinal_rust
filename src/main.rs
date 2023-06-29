use chrono::{Datetime, Utc};

struct usuario{
    nombre: String,
    email: String,

}


struct tarea{
    nombre: String,
    fecha_hora: Datetime<Utc>,


}


fn main() {
    let now: Datetime<Utc> = Utc::now();
    println!("{}", now);
}
