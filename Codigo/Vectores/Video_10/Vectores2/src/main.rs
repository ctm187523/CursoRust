/*
        VECTORES DINAMICOS

    -¿Qué es un Vec?
      - Un Vec<T> es una colección de elementos del mismo tipo (T)
      - Se almacena en el heap, lo que permite que su tamaño sea dinámico y pueda crecer o reducirse en tiempo de ejecución.
      - Puede cambiar tamaño de forma dinamica a diferencia de los arrays que tienen un tamaño fijo.
    -¿Qué guarda un Vec en memoria?
       -En el stack guarda una cabecera con 3 datos:
            1. ptr->puntero al heap donde estan los elementos del vector
            2. len->número de elementos actuales en el vector
            3. cap->número de elementos caben sin recolocar, reserva un espacio en el heap de primera, guarda un espacio de memoria inicial
               si sigue añadiendo elementos y se supera la capacidad, se crea un nuevo espacio en el heap con más capacidad, se copian los elementos
               y se libera el espacio anterior. Si se recoloca tiene un coste computacional, el cap indica cuantos elementos
               caben sin necesidad de recolocar.

    -¿Ejemplos de casos de uso de Vec en lugar de arrays?
       Cuando no sabes de antemano:
         -Cuantos datos vas a tener
         -Cuantos usuarios, vas a tener en una aplicacion
         -Cuantos resultados de una busqueda obtendras al realizar una consulta
         -Cuantas lineas tiene un archivo
         -Cuantos registros devuelve una BBDD(Base de Datos)

*/

//      OPERACIONES CON VECTORES

fn main() {


    //Crear un Vector vacio y añadir elementos posteriormente, el tipo es i32 pero podriamos no ponerlo el tipo
    //y en el primer push se infiere el tipo -> let mut numeros2 = Vec::new();
    let mut numeros: Vec<i32> = Vec::new(); //debe ser mutable para añadir elementos

    //Añadir elementos al vector con el metodo push
    numeros.push(25);
    numeros.push(10);
    numeros.push(20);
    numeros.push(30);
    numeros.push(35);
    numeros.push(100);

    println!("Vector numeros despues los push: {:?}", numeros);

    //eliminamos elementos del Vector con pop
    //si ponemos numeros.pop(); eliminamos el ultimo elemento
    numeros.pop();
    println!("Vector numeros despues del pop, se elimina el ultimo elemento: {:?}", numeros);

    //la funcion pop() nos devuelve un Option<T> de tipo cualquiera, esto es asi si intentas eliminar
    //un elemento que no existe, siempre devuelve un Option<T>, para que el programa no caiga y no de error
    //en otros lenguajes su quieres eliminar un elemento que no existe te da error, en Rust devuelve un Option
    //tanto si ha elminado el elemento como no y asi no cae el programa.
    let ultimo = numeros.pop();  //Visual estudio code por inferencia de tipo ya te pone que es de tipo Option<i32> podria haber un numero o no
    //si hay algo el Optiomn devuelve Some y menciona el ultimo elemento que elimina, si borramos los push de arriba
    //como no hay elementos devuelve none
    println!("Devuelve lo que contiene la devolucion del metodo pop() el Option: {:?}", ultimo);

    //Accedemos a los elemetos del Vector podemos usar [] y get()
    //con [] accedemos como un array en otros lenguajes, fijarse como en el println no ponemos :? entre los corchetes
    //porque estamos accediendo a un elemento concreto del vector, no a todo el Vector, como accedemos a i32 
    //i32 implementa Display por lo que no hace falta el {:?}
    println!("Imprimimos el elemento 2: {}", numeros[2]);

    //si queremos acceder a una posicion que no existe como la posicion 5, ya que al hacer anteriormente un pop,
    //la hemos eliminado tendremos un error index out of bounds, comentamos la linea para que no de error
    //print!("nos pasamos de los elementos del Vector {}", numeros[5]);

    //si usamos get() en lugar de pop() lo hacemos de una forma mas segura
    // si queremos acceder a una posicion del Vector que no existe no dara error, hay que poner {:?}
    // con get(), ya que no implementa display, si nos hemos pasado como en el caso de abajo devuelve none
    //Rust evita el Null
    println!("nos pasamos de los elementos del Vector, usamos get() para que no de error {:?}", numeros.get(5));

    //si devuelve un valor existente en el vector devuelve Some y entre paréntesis el valor que devuelve
    //nos devuelve un Option<T>
    println!("Usamos get() para la devolucion de un elemento del array, {:?}", numeros.get(3));


}
