// Ver imagenes video 13
// Iteradores Map, Filter, Collect


fn main() {

    let numeros = vec![1, 2, 3, 4, 5, 6];
    let mut resultado = Vec::new();     //vector mutable vacio para guardar el resultado
   
    //con bucle for de manera imperativa
    //queremos coger lo numeros pares y multiplicarlos por 2
    for numero in &numeros {
        if numero % 2 == 0 {    //operador modulo %
            resultado.push(numero * 2);
        }
    }   

    println!("{:?}", resultado);

    //vamos a hacer lo mismo pero con iteradores de manera funcional
    let resultado_funcional: Vec<i32> = numeros.iter()   //iteramos sobre el vector numeros
        .filter(|&x| x % 2 == 0)   //filtramos los numeros pares, con |&x| usamos la funcion closure o funcion anonima, el &x es para desreferenciar el valor del iterador, seria como el arrow function en JavaScript, o funciones lambda en otros lenguajes, el & es para evitar mover el valor del iterador, ya que iter() devuelve referencias a los elementos del vector
        .map(|x| x * 2)            //multiplicamos por 2 los numeros filtrados
        .collect();                //colectamos el resultado en un nuevo vector

    //podriamos haber hecho lo anterior tambien usando punteros en lugar de referencias
    let resultado_con_punteros: Vec<i32> = numeros.iter()   //iteramos sobre el vector numeros
        .filter(|n| *n % 2 == 0)   //filtramos los numeros pares, con |x| usamos la funcion closure o funcion anonima, el **x es para desreferenciar el valor del iterador, seria como el arrow function en JavaScript, o funciones lambda en otros lenguajes, el & es para evitar mover el valor del iterador, ya que iter() devuelve referencias a los elementos del vector
        .map(|n| *n * 2)            //multiplicamos por 2 los numeros filtrados
        .collect();                //colectamos el resultado en un nuevo vector

    //resumen de los dos ejemplo abajo del codigo

    println!("{:?}", resultado_funcional);
    println!("{:?}", resultado_con_punteros);

    //codigo equivalente a los dos ejemplos anteriores, sin usar funciones anonimas
    fn es_par(x: &&i32) -> bool {  //debemos usar &&i32 es decir referencia a una referencia, porque iter() devuelve referencias a los elementos del vector, entonces la funcion debe aceptar una referencia a una referencia para poder trabajar con ella
        **x % 2 == 0   //desreferenciamos dos veces para obtener el valor real y comprobar si es par
    }

    fn multiplicar_por_dos(x: &i32) -> i32 {  //usamos &i32 porque iter() devuelve referencias a los elementos del vector, no devuelve valores sino referencias, entonces la funcion debe aceptar una referencia para poder trabajar con ella
        x * 2
    }

    let resultado_funcional_sin_closure: Vec<i32> = numeros.iter()
        .filter(es_par)   //filtramos los numeros pares usando la funcion es_par
        .map(multiplicar_por_dos)  //multiplicamos por 2 los numeros filtrados usando la funcion multiplicar_por_dos
        .collect();                //colectamos el resultado en un nuevo vector

    println!("{:?}", resultado_funcional_sin_closure);

    }

  

// Diferencia entre |&x| y |n| *n
// 👉 Cuando usas iter() en Rust, no trabajas con valores (i32), sino con referencias (&i32), es decir, “direcciones” que apuntan al valor.

// 👉 Desreferenciar (*) significa acceder al valor real al que apunta esa referencia.

// 🔑 Diferencia entre tus dos bloques

// 👉 |&x|

// Desreferencia automáticamente al recibir el dato
// Convierte &i32 → i32 al entrar en la closure
// Ya trabajas directamente con el valor

// 👉 |n| *n

// Recibes la referencia (&i32)
// Tú decides cuándo acceder al valor usando *n
// Desreferencia manual
// 🧠 Resumen final

// 👉 Ambos hacen lo mismo
// 👉 La diferencia es:

// |&x| → desreferencia automática al inicio
// |n| *n → desreferencia manual dentro del código

// 👉 Es decir:
// en uno ya tienes el valor directamente, en el otro tienes que “ir a buscarlo” desde la referencia usando *


