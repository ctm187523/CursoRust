/*                                             BORROWING
  -¿Que es?
    - Mecanismo mediante el cual uan variable permite que otra parte del programa acceda a su 
      valor sin transferir la propiedad(ownership).
    
    - Rust lo hace creando una referencia a él (mutable o immutable), permitiendo leerlo o modificarlo
      temporalmente sin cambiar el dueño real.
    
    - Con esto el acceso a la memoria es seguro sin punteros colgantes, accesos simultáneos o modificaciones
      no controladas.(ver en imagenes/VIDEO_6/RustVSC++.png)
*/

fn main(){

    let saludo=String::from("hola");

    //creamos un prestamo a la variable saludo usamos el simbola &(punteros) 
    //referencia es un prestamo apunta a la variable saludo en el stack y la variable saludo apunta
    //al valor "hola" en el heap
    let referencia=&saludo;

    //los prestamos por defecto son immutables como las variables, si queremos usar el metodo
    //push_str para añadir mas texto en la variable prestada, no nos deja ya que por defecto es  immutable
    //para que se pueda modificar la variable dueña, saludo y la variable que recibe el prestamo referencia deben
    //tener la palabra reservada mut para hacerlas mutables.
    //comentamos la linea de abajo porque es erronea
    //referencia.push_str("mundi cruel");

    println!("Original: {} ",saludo);      //imprimimos el valor de la variable dueña

    println!("Prestamo: {} ",referencia);  //imprimimos el valor de la variable que tiene el prestamo con la variable saludo

    //si la variable saludo tiene un prestamo no la podemos eliminar, para evitar los punteros colgantes
    //Rust no lo impide, comentamos las 2 lineas siguientes ya que son erroneas es para provar el
    //sitema de seguridad de Rust
    // std::mem::drop(saludo);
    // println!("Prestamo: {} ",referencia);

    println!("-----------------------------------------------------------------");

    //Existen los prestamos mutables y immutables como con las variables, los immutables son pro defecto
    let mut saludo2=String::from("hello");      //creamos una variable mutavle

    let referencia2=&mut saludo2;          //creamos un prestamo a la variable mutable

    referencia2.push_str(" cruel world");       //añadimos texto a la variable con el prestamo a saludo2

    //comentamos la linea de abajo que imprime el valor de la variable dueña saludo2, si la descomentamos
    //da error, este error es una medidad de seguridad de Rust, mientras exista una referencia mutable como es el caso
    //de la variable referencia2 no podemos usar la variable dueña saludo2 para nada ni siquiera para leerla
    //Rust garantiza que un prestamo mutable tenga acceso exclusivo, para garantizar que no haya 2 alias simultaneos
    //al mismo valor, violacion de exclusividad, podemos tener muchas referencias immutables a la variable dueña saludo
    //pero solo podemos tener una unica referencia mutable y no se pueden mezclar referencias mutables y immutables al mismo tiempo
    //una referencia mutable excluye todo uso de la variable dueña saludo2 mientras este activa
   // println!("Original: {} ",saludo2);      //imprimimos el valor de la variable dueña

    println!("Prestamo: {} ",referencia2);  //imprimimos el valor de la variable que tiene el prestamo con la variable saludo2

    //si cambiamos el orden de impresion primero el valor de la variable dueña saludo2 y luego el valor de la variable
    //que toma prestado de variable2(referencia2), ya no tenemos error, porque la variable que toma prestada la variable
    //saludo2(referencia2) una vez se imprime deja de usarse en el resto del programa, el ciclo de vida de prestamo2 termina
    //con lo cual ahora si podemos acceder a la variable dueña saludo2

    println!("--------Invertimos lineas de codigo primero imprimimos el valor de la variable que tiene el prestamo----------------");

    println!("Prestamo: {} ", referencia2);
    println!("Original: {} ", saludo2);

}










