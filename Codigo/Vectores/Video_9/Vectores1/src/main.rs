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

use core::num;
use std::vec;

fn main() {
    //Crear un Vector numerico con valores inicializados, vec! es una macro que crea un vector
    let numeros = vec![1, 2, 3];

    //imprimimos el vector con el trait Debug({:?}), ver leccion anterior
    println!("Vector numeros: {:?}", numeros);

    //Crear un Vector vacio y añadir elementos posteriormente, el tipo es i32 pero podriamos no ponerlo el tipo
    //y en el primer push se infiere el tipo -> let mut numeros2 = Vec::new();
    let mut numeros2: Vec<i32> = Vec::new(); //debe ser mutable para añadir elementos

    //Añadir elementos al vector con el metodo push
    numeros2.push(10);
    numeros2.push(20);
    numeros2.push(30);

    println!("Vector numeros despues de push: {:?}", numeros);

    //si no inferimos el tipo y no hacemos por ejemplo un push, el vector estara vacio y no sabra que tipo es
    //visual studio code pone Vec<{unknown}> en el tooltip, y no compila da un error lo comentamos
    //let mut numeros3 = Vec::new();

    //cap nos muestra la capacidad actual del vector sin recolocacion, podemos indicar cual sera el maximo con with_capacity
    //si indicamos los elementos que caben, no habra recolocacion hasta que se supere esa capacidad
    let mut numeros4: Vec<i32> = Vec::with_capacity(500);
    println!("Capacidad inicial del vector numeros4: {}", numeros4.capacity());



}
