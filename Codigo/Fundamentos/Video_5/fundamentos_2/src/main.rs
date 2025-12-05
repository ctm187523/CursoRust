/*
    Rust hace las variables inmutables por defecto por una razón muy importante:
    → inmutabilidad = seguridad + menos bugs + más optimización.

    Te explico cada motivo de forma clara.

    ✅ 1. Evitar errores comunes desde el diseño del lenguaje

    En muchos lenguajes, cualquier variable puede cambiarse en cualquier lado.
    Eso causa errores difíciles de encontrar:

    valores que cambian sin querer

    estados inconsistentes

    efectos colaterales invisibles

    bugs en multithreading

    Rust decide lo contrario:

    Solo puedes cambiar algo si tú explícitamente dices que quieres poder cambiarlo.

    Ejemplo:

    let x = 10; // inmutable
    x = 20;     // ❌ error: te obliga a pensar


    Si realmente quieres cambiarlo:

    let mut x = 10;
    x = 20;     // ✔️ ahora sí


    Rust te fuerza a escribir mut para indicar:
    “Esta variable cambiará su valor a lo largo del programa.”

    Esto documenta tus intenciones y reduce muchos errores.

    ✅ 2. Seguridad en concurrencia

    La inmutabilidad es clave para que Rust garantice concurrencia segura.

    Si dos hilos leen la misma variable inmutable → no hay problema.

    Si quieres que varios hilos modifiquen un valor, ya no es trivial, y Rust te obliga a usar herramientas explícitas:

    Mutex

    RwLock

    Arc

    Atomic*

    Así sabe que estás protegiendo el acceso concurrente.

    ✅ 3. Permite más optimizaciones del compilador

    Una variable inmutable significa "este valor no va a cambiar".

    El compilador puede:

    reutilizar registros

    eliminar copias

    tratar valores como constantes internas

    hacer análisis de flujo más simple y más agresivo

    Rust prioriza rendimiento tipo C, así que esto le viene muy bien.

    ✅ 4. Hace el código más legible y fácil de razonar

    Una variable mutable puede cambiar en cualquier punto del código.

    Una variable inmutable:

    no cambia

    no te puede “sorprender”

    no tienes que rastrear dónde se modificó

    En programas grandes, esto ahorra dolores de cabeza.

    🎯 En resumen

    Rust hace las variables inmutables por defecto porque:

    Evita errores de estado mutado accidentalmente.

    Facilita la concurrencia segura (sin data races).

    Permite más optimización y mejor rendimiento.

    El código es más claro y predecible.

    Y si quieres mutabilidad, solo lo dices explícitamente con mut.

*/


//las constantes se puede declarar fuera de la funcion, no se guardan ni en el heap ni el stack se guarda
//en el fichero binario ejecutable, por convencion se le ponen en Mayúscula y siempre hay que especificar el tipo
const SALUDO:&str ="Soy una constante";

//comentamos lo de abajo porque es erroneo una constante no puede almacenar un valor que obtenemos de forma dinámica mediante una función
//const RESULTADO:i32 = obtener_numero(10);

fn main() {
    
    let mut contador=30;    //variable mutable con la palabra reservada mut, podemos cambiar el valor de la variable

    contador=contador+1;

    println!("Contador después de incrementar: {}", contador);

    let saludo ="Hola";    //variable inmutabe, hay que declararla siempre dentro de una funcion, las constantes si se pueden declarar fuera de la funcion

    println!("{}", saludo);

    println!("{}", SALUDO);  //uso el valor de la constante creada arriba en encima de la funcion main

    cualquiera();   //llamamos a la funcion creada abajo

    //creamos una variable inmutable, puede recibir un valor de forma dinamica de una funcion, no como en el caso comentado de arriba aplicado a la constante RESULTADO
    let resultado = obtener_numero(10);

     println!("resultado de una variable inmutable que de forma dinámica toma un valor de una función: {}", resultado);
}

fn cualquiera(){
     println!("Estoy en la función cualquiera {}", SALUDO);   //podemos usar la constante SALUDO ya que es generañ
}


//la funcion devuelve el valor que recibe por parámetro, ponemos -> para indicar que devuelve un numero entero(i32)
fn obtener_numero(n:i32)->i32{

    n
}
