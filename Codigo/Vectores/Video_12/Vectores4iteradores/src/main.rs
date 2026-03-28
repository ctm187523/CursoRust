
//ver codigos anteriores de Vectores
//Ver en Imagenes Video_12 imagen -> UsoIteradoresVSnoUsarlos

//Es conveniente usar iteradores cuando quieras:
    // - Transformar datos(map)
    // - Filtrar datos(filter)
    // - Encadenar operaciones sobre los datos
    // - Escribir código más limpio y legible
fn main() {
  
  let numeros=vec![1,2,3];

  for n in  numeros {
    println!("{} ", n);
  }

    //comentamos lo de abajo porque da error, como comentamos en el ejemplo anterior
    //ahora numeros ya no es propietariod de los datos del vector ahora el propietario es el iterador
   //println!("{:?}",numeros);

   //si usamos la propiedar .iter() para crear un iterador sobre el vector numeros
   //hace lo mismo que si usamos la referencias &numeros seria asi con referencias como en el video anterior:
    // for n in &numeros {
    //  println!("{} ", n);
    // }

    println!("Segundo ejemplo con .iter() sin usar referencias: ");
    println!("-----------------------------------------------------------------");
    //vamos a usar ahora la funcion .iter() sin usar referencias
    let numeros2=vec![8,9,10];
   
    for n in numeros2.iter() {
        println!("{} ", n);
    }

    println!("{:?}", numeros2);

   println!("Tercer ejemplo con .iter()_mut para vectores mutables: ");
   //si usamos ahora .iter_mut() en el bucle for hacemos que sea mutable el iterador
   //como haciamos en el ejemplo ejemplo anterior Vectores3:
   
    // for n in &mut numeros3 {
    //   *n += 1; //con el * desreferenciamos la referencia mutable para modificar el valor del elemento
    // }

    //usamos ahora .iter_mut() para vectores mutables
    let mut numeros3=vec![11,12,13];

    for n in numeros3.iter_mut() {
        *n += 1; //con el * desreferenciamos la referencia mutable para modificar el valor del elemento del vector, con el * desreferenciamos la referencia mutable para modificar el valor del elemento del vector, con * le decimos dame el valor al que apunta esta referencia
    }

    println!("{:?}", numeros3);

    println!("Cuarto ejemplo con .into_iter()_mut para vectores mutables: ");

    //si usamos ahora .into_iter() en el bucle for hacemos que el iterador tome la propiedad de los datos del vector, 
    
    let numeros4=vec![14,15,16];
    for n in numeros4.into_iter() {
        println!("{} ", n);
    }

    //comentamos el codigo de abajo porque da error, numeros4 pierde la propiedad de los datos del vector
    //println!("{:?}", numeros4);

}
