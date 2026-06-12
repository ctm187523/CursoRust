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
    
    //vamos hacer un hasmap para almacenar notas de alumnos, la clave sera el nombre del alumno y el valor sera la nota del alumno.
    let mut notas = HashMap::new();  //es mut para que sea mutable y podamos agregar elementos al HashMap

    notas.insert("Ana", 7.5); //insertamos el nombre del alumno como clave y la nota como valor
    notas.insert("Juan", 4.1);
    notas.insert("Maria", 8.2);
    notas.insert("Pedro", 9.1);

    //imprimimos el HashMap
    println!("Las notas de los alumnos son: {:#?}", notas);  //con {:#?} imprimimos el HashMap de manera mas legible al poner la almohadilla #


    //imprimimos la nota de Maria
    let nota_maria = notas.get("Maria"); //con get() obtenemos el valor asociado a la clave "Maria", el resultado es un Option<&f64> porque la clave puede no existir en el HashMap, si la clave existe el resultado seria Some(8.2), si la clave no existe el resultado seria None
    println!("La nota de Maria es: {:?}", nota_maria); 

    //recorremos el hashmap con un for
    for (alumno, nota) in &notas { //con &notas obtenemos una referencia al HashMap para no consumirlo, el resultado es un iterador que devuelve tuplas de clave-valor, en este caso tuplas de tipo (&str, f64)
        println!("El alumno {} tiene una nota de {}", alumno, nota); //imprimimos el nombre del alumno y su nota
    } 

}