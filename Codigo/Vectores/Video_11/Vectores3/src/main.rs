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


    let numeros = vec![1,2,3]; //otra forma de crear un vector con el macro vec![], el tipo se infiere por los elementos que contiene, en este caso i32

    //iteramos el vector
    for n in numeros{
        println!("{} ", n);
    }

    //lo de abajo da error, lo comentamos ya no podemos acceder al vector, si o lo hubieramos iterado con el for
    //anterior no dario error, es por tema de dueño de la variable aerriba en ->  let numeros = vec![1,2,3];
    //el vector numeros es el dueño de los datos que contiene, al iterar con el for n es una variable que toma el valor de cada elemeto del vector
    //en el bucle for se crea un iterador que va tomando cada elemento del vector y se lo asigna a n
    //ahora la variable numeros queda invalidada, el dueño pasa de la variable numeros al iterador
    //esto es asi porque en Rust no hay Garbage Collector
    //en Java o C# no pasaria esto, ya que Java toma referencias a los objetos de la variable numeros, en Rust se mueve la propiedad de los datos del vector a la variable n, por eso numeros queda invalidada 
    //aqui vemos la diferencia entre mover y copiar, en Rust se mueve la propiedad de los datos del vector a la variable n, por eso numeros queda invalidada, en Java o C# no pasaria esto, ya que Java toma referencias a los objetos de la variable numeros, 
    //en Rust se mueve la propiedad de los datos del vector a la variable n, por eso numeros queda invalidada
    // comentamos lo de abajo
    //println!(" {:?}", numeros);

    //para que esto no ocurra usamos borrowing con referencias usando &, con el & le decimos que queremos una referencia a cada elemento del vector, no movemos la propiedad de los datos del vector a la variable n,
    //ponemos otro ejemplo con referecias
    let numeros2 = vec![4,5,6];

    for n in &numeros2{ //con el & le decimos que queremos una referencia a cada elemento del vector, no movemos la propiedad de los datos del vector a la variable n, por eso numeros2 no queda invalidada
        println!("{} ", n);
    }

    //ahora si podemos acceder al vector numeros2 despues del bucle for, porque no se ha movido la propiedad de los datos del vector a la variable n, por eso numeros2 no queda invalidada
    //aqui si numeros2 es propietario del vector y no se ha movido la propiedad de los datos del vector a la variable n, por eso numeros2 no queda invalidada
    println!(" {:?}", numeros2);

    //si queremos modificar los elementos del vector, necesitamos una referencia mutable, con &mut le decimos que queremos una referencia mutable a cada elemento del vector, no movemos la propiedad de los datos del vector a la variable n, por eso numeros3 no queda invalidada
    let mut numeros3 = vec![7,8,9];

    for n in &mut numeros3{ //con el &mut le decimos que queremos una referencia mutable a cada elemento del vector, no movemos la propiedad de los datos del vector a la variable n, por eso numeros3 no queda invalidada
        *n += 1; //con el * desreferenciamos la referencia mutable para modificar el valor del elemento del vector, con el * desreferenciamos la referencia mutable para modificar el valor del elemento del vector, con * le decimos dame el valor al que apunta esta referencia
    }

    println!(" {:?}", numeros3);
}
