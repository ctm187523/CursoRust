//seguimos con vectores, las funciones que trabajamos ahora son funciones que permiten
//operar, obtener resultados
//por ejemplo sum(),find(), etc

fn main() {
    
    let numeros = vec![1,2,3,4,5,6];

    let resultado:i32= 
         numeros.iter()
        .filter(|n| *n % 2 == 0)       //usamos solo los numeros pares, con el * desreferenciamos la referencia que nos da el iterador para obtener el valor del elemento del vector, con el * desreferenciamos la referencia que nos da el iterador para obtener el valor del elemento del vector
        .map(|n| n * 2)                //multiplicamos por 2 cada numero par, con el map aplicamos una funcion a cada elemento del vector, en este caso multiplicamos por 2 cada numero par
        .sum();                       //sum() es una funcion que suma todos los elementos del vector, en este ejemplo suma los numeros pares del vector, 12

    println!("El resultado es: {:?}", resultado);

     let resultado2:Option<&i32>=     //find devuelve una referencia al elemento del vector que cumple la condicion, por eso el tipo de resultado2 es Option<&i32>, porque puede que no se encuentre el elemento que estamos buscando, en este caso se encuentra el numero 4, pero si no se encontrara ningun numero mayor a 3, el resultado seria None
         numeros.iter()
        .find(|n|**n>3);                      //find() es una funcion que busca un elemento en el vector que cumpla una condicion, en este caso buscamos el primer numero que sea mayor a 3, con el **n desreferenciamos la referencia que nos da el iterador para obtener el valor del elemento del vector
                                              //se almacena find en un tipo Option, porque puede que no se encuentre el elemento que estamos buscando, en este caso se encuentra el numero 4, pero si no se encontrara ningun numero mayor a 3, el resultado seria None
    println!("El resultado2 es: {:?}", resultado2);

    let resultado3:Option<&i32>=     //find devuelve una referencia al elemento del vector que cumple la condicion, por eso el tipo de resultado2 es Option<&i32>, porque puede que no se encuentre el elemento que estamos buscando, en este caso se encuentra el numero 4, pero si no se encontrara ningun numero mayor a 3, el resultado seria None
    numeros.iter()
    .find(|n|**n>7);                     //find() es una funcion que busca un elemento en el vector que cumpla una condicion, en este caso buscamos el primer numero que sea mayor a 7, con el **n desreferenciamos la referencia que nos da el iterador para obtener el valor del elemento del vector, como no se encuentra ningun numero mayor a 7, el resultado seria None
                                              //se almacena find en un tipo Option, porque puede que no se encuentre el elemento que estamos buscando, en este caso se encuentra el numero 4, pero si no se encontrara ningun numero mayor a 3, el resultado seria None
    println!("El resultado3 es: {:?}", resultado3);


    let resultado4=    
    numeros.iter()
    .filter(|n|*n%2==0)              //usamos solo los numeros pares, con el * desreferenciamos la referencia que nos da el iterador para obtener el valor del elemento del vector, con el * desreferenciamos la referencia que nos da el iterador para obtener el valor del elemento del vector
    .count();                       //count() es una funcion que cuenta el numero de elementos del vector que cumplen la condicion, en este caso contamos el numero de numeros pares del vector, 3
                                         
    println!("El resultado4 es: {:?}", resultado4);

    let resultado5:bool=     
    numeros.iter()
   .any(|n|*n%2==0);              //con any() verificamos si al menos un elemento del vector cumple la condicion, en este caso verificamos si al menos un numero es par, con el * desreferenciamos la referencia que nos da el iterador para obtener el valor del elemento del vector, como hay numeros pares en el vector, el resultado seria true
                                         
    println!("El resultado5 es: {:?}", resultado5);

    let resultado6:bool=     
    numeros.iter()
   .all(|n|*n%2==0);              //con all() verificamos si todos los elementos del vector cumplen la condicion, en este caso verificamos si todos los numeros son pares, con el * desreferenciamos la referencia que nos da el iterador para obtener el valor del elemento del vector, como no todos los numeros son pares en el vector, el resultado seria false             
                                         
    println!("El resultado6 es: {:?}", resultado6);

    let resultado7:Option<&i32>=     
    numeros.iter()
   .max();              //con max() obtenemos el valor maximo del vector, en este caso el numero 6, como el vector no esta vacio, el resultado seria Some(6)

    println!("El resultado7 es: {:?}", resultado7);


   let resultado8:Option<&i32>=
   numeros.iter()
   .min();        //con min() obtenemos el valor minimo del vector, en este caso el numero 1, como el vector no esta vacio, el resultado seria Some(1)  
                                         
   println!("El resultado8 es: {:?}", resultado8);
}
