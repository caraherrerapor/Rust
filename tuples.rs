
// las tuplas pueden ser usadas como funcion con argumentos y retornar valores
fn reverse(pair: (i32,bool))->(bool,i32){
    // le puede ser usada para en enlazar miembros de una tupla o variables
    let (integer, boolean )= pair;

    (boolean ,integer)
}

#[derive(Debug)]

struct Matrix(f32,f32,f32,f32);



fn main(){
    // una tupla con con un grupo de diferentes tipos
    let long_tuple = (1u8,2u16,3u32,4u64,
                      -1i8,-2i16,-3i32,-4i64,
                      0.1f32,0.2f64,
                      'a',true);
    // los valores pueden ser extraidos de la tupla usando su indexacion
     println!("long_tuple first value: {}",long_tuple.0);
     println!("long_tuple second value: {}",long_tuple.1);
    


}

