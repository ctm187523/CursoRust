//En este ejemplo vamos a ver los HashMap, que es una colección que almacena pares de clave-valor. Es similar a un diccionario en otros lenguajes de programación.
// En Rust, los HashMap se encuentran en el módulo std::collections. Para usarlo, primero debemos importarlo con use std::collections::HashMap;
// Luego, podemos crear un nuevo HashMap con HashMap::new() y agregar elementos con el método insert(). También podemos acceder a los valores utilizando la clave con el método get().
// Además, los HashMap pueden almacenar cualquier tipo de dato como clave o valor, siempre y cuando implementen el trait Eq y Hash para las claves.
//la clave tiene que ser unica, si damos diferentes claves no trabaja bien dara el ultimo valor asignado a esa clave

//Diferecia entre Vecto VS HashMap
//Un vector es ideal cuando tenga una lista ordenada de elementos.
//Un HasMap es mas útil cuando quiero buscar un dato por una clave, por ejemplo un identificador, palabra. etc
//Un HashMap se utiliza para diferentes cosas: como Caché de datos para optimizar rendimiento, en lugar de consulatar
//una base de datos, podemos almacenar los datos en un HashMap y acceder a ellos de manera rápida utilizando la clave, etc
//tambien se usan en Configuración de aplicaciones, ejemplo idioma:Español, etc en Configuración de aplicaciones.
//tambien se usan en Agrupamiento de datos por categorías, ejemplo con productos, ejemplo programación -> Rust, Java,Phyton
//podemos tener varios valores en un HasMap, podemos tener listas como valores.
//tambien se usan en Sesiones de usuario. ej clave -> token_usuario -- valor -> datos_usuario


//para trabajar con Hasmap, primero debemos importarlo con use std::collections::HashMap;
use std::collections::HashMap;

fn main() {

    let mut notas = HashMap::new();  //es mut para que sea mutable y podamos agregar elementos al HashMap

    notas.insert("Ana", 7.5); //insertamos el nombre del alumno como clave y la nota como valor
    notas.insert("Juan", 4.1);
    notas.insert("Maria", 8.2);
    notas.insert("Pedro", 9.1);

    //insertamos el nombre del alumno como clave y la nota como valor, si la clave ya existe, 
    //se reemplaza el valor anterior por el nuevo valor, lo comentamos para ver las siguiente instruccion
    //notas.insert("Ana", 9.5); 

    //si repetimos la clave, el valor se reemplaza pero tambien devuelve el valor anterior
    //podemos rescatar el valor anterior aunque el valor se reemplaze
    let valor_anterior = notas.insert("Ana",9.5);

    //si queremos insertar una nueva clave y valor y no sabemos si la clave ya existe,
    //podemos usar el metodo entry.or_insert() que nos permite insertar un valor si la clave no existe,
    // y si existe no hace nada y nos devuelve una referencia al valor existente, en este caso el valor de Juan es 4.1, es un valor mutable
    notas.entry("Juan").or_insert(5.0); //como la clave "Juan" ya existe, no se inserta el valor 5.0 y nos devuelve una referencia al valor existente, que es 4.1

   //vamos a obtener en una variable el valor de Juan con la funcion entry.or_insert
   let valor_juan = notas.entry("Juan").or_insert(5.0); //como la clave "Juan" ya existe, no se inserta el valor 5.0 y nos devuelve una referencia al valor existente, que es 4.1   
   
   //podemos modificar el valor de Juan a traves de la referencia que nos devuelve entry.or_insert()
   //ya que es un valor mutable, podemos modificarlo directamente
    *valor_juan = 2.5; //modificamos el valor de Juan


    // si no existe con entry.or_insert() podemos insertar un nuevo valor, en este caso el valor de Luis es 6.0
    notas.entry("Luis").or_insert(6.0); //como la clave "Luis" no existe, se inserta el valor 6.0 y nos devuelve una referencia al valor existente, que es 6.0


    //recorremos el hashmap con un for
    for (alumno, nota) in &notas { //con &notas obtenemos una referencia al HashMap para no consumirlo, el resultado es un iterador que devuelve tuplas de clave-valor, en este caso tuplas de tipo (&str, f64)
        println!("El alumno {} tiene una nota de {}", alumno, nota); //imprimimos el nombre del alumno y su nota
    }

    //imprimimos el valor recuperado de Ana, que es el valor anterior antes de ser reemplazado
    println!("El valor anterior de Ana era: {:?}", valor_anterior); //el resultado es Some(7.5) porque el valor anterior era 7.5, si la clave no existiera el resultado seria None

    //imprimimos el valor recuperado de Juan, que es una referencia al valor existente, que es 4.1
    //lo comentamos porque da error ver la explicacion que pongo abajo
    //println!("El valor de Juan es: {:?}", valor_juan); //el resultado es 4.1 porque el valor existente es 4.1, si la clave no existiera el resultado seria None


    /*
    
        La razón por la que sí te deja imprimir valor_anterior es que su tipo de dato no mantiene ninguna relación ni enlace con el HashMap. El HashMap queda completamente libre.
        Aquí tienes la explicación detallada de por qué uno funciona y el otro falla:
        1. El caso de valor_anterior (No bloquea el HashMap)Cuando haces esto:
                let valor_anterior = notas.insert("Ana", 9.5);
            El método .insert() saca el valor antiguo de dentro del HashMap y te lo entrega por completo.
            En Rust, el tipo de dato f64 (los números decimales) tiene el trait Copy.
            Esto significa que valor_anterior es simplemente una copia independiente del número 7.5 envuelto en un Option.
            Al ser un valor independiente, no tiene ninguna referencia (puntero) apuntando al HashMap.
            El HashMap está libre de préstamos y puedes hacer con él lo que quieras (como el bucle for).
            
        2. El caso de valor_juan (Bloquea el HashMap)Cuando haces esto en tu código original:
                let valor_juan = notas.entry("Juan").or_insert(5.0);
           El método .or_insert() no te devuelve un número independiente. Te devuelve una referencia mutable (&mut f64)
           que apunta directamente al número que está guardado dentro de la memoria del HashMap.
           Mientras la variable valor_juan exista, Rust interpreta: "Cuidado, hay una variable que tiene permiso exclusivo 
           para modificar las entrañas del HashMap".Por las reglas de seguridad de Rust (Borrow Checker), 
           si tienes un préstamo mutable activo (valor_juan), está estrictamente prohibido hacer cualquier otra lectura o escritura
           en el HashMap hasta que esa variable deje de usarse.
           Como tu println! final intentaba usar valor_juan, Rust mantenía esa referencia mutable "viva" durante el bucle for, 
           causando el error de compilación.
           En resumen:valor_anterior es un dato independiente (un número copiado). No toca al HashMap.
                      valor_juan es un puntero directo al interior del HashMap. Lo mantiene congelado/bloqueado para otros usos.
     */

}