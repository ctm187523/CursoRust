
/*

    En Rust, NLL significa Non-Lexical Lifetimes (tiempos de vida no léxicos).
    Es una mejora del compilador que hace que las referencias (borrows) duren solo lo necesario, no hasta el final del bloque como pasaba antes.

    Es clave para que Rust sea más usable sin perder seguridad.

    fn main() {
    let r;

        {
            let s = String::from("Hola");
            r = &s;
        }
    }

        🧠 ¿Por qué sí compila?

    Porque aunque r está declarado fuera, nunca se usa después.

    Con NLL (Non-Lexical Lifetimes) el compilador razona así:

    s vive solo dentro del bloque interno

    r = &s crea un préstamo válido en ese punto

    r no se lee ni se usa nunca

    El último uso real de la referencia es la propia asignación

    El borrow termina ahí mismo

    Cuando s muere, no queda ninguna referencia activa que se use

    👉 No hay posibilidad de “use-after-free”, así que Rust lo permite.

    🔑 Punto clave (muy importante)

    Rust no prohíbe crear una referencia que “apunte a algo que va a morir”
    Lo que prohíbe es USAR esa referencia después de que el dato muera

    En tu código:

    La referencia existe

    Pero no se usa

    Así que no hay peligro

    ❌ Prueba mínima que lo rompe

En cuanto uses r fuera del bloque, deja de compilar:

fn main() {
    let r;

    {
        let s = String::from("Hola");
        r = &s;
    }

    println!("{}", r); // ❌ ERROR
}

    Error típico:

    s does not live long enough
    borrowed value does not live long enough

    Aquí sí habría un dangling reference real.

    🔍 ¿Esto es gracias a NLL?
    Sí, indirectamente

    Antes de NLL, el compilador era más conservador y este tipo de código podía fallar más fácilmente.

    Con NLL:

    El compilador analiza usos reales

    Ve que r no se usa

    El lifetime efectivo del borrow es ultra corto

    Todo es seguro

    Pero ojo:

    NLL NO extiende la vida de s
    NLL solo acorta la vida del borrow

    🧠 Regla mental correcta (afinada)

    ❌ “Una referencia no puede vivir más que su valor” → simplificación
    ✅ Una referencia no puede SER USADA después de que su valor muera

    Tu ejemplo cumple esta regla.

    📌 Resumen final

    ✅ Tu código sí compila

    ✅ Es válido porque r no se usa

    ✅ NLL permite que el borrow termine en la asignación

    ❌ En cuanto intentes usar r fuera, falla

    ❌ NLL nunca permite referencias colgantes usadas

    Este es un ejemplo muy bueno para entender el borrow checker moderno 👌

*/



fn main() {
    
    
    let r;

    //creamos codigo dentro de un bloque entre corchetes, esto implica que el ciclo de vida solo sera mientras transcurra 
    //el codigo dentro del bloque entre corchetes
    //en versiones anteriores el bloque de codigo daba error hasta 2018 (Ver imagenes video 7/lifeTimes) 
    //en 2018 se introduce e NLL(No lexical LifeTimes), antiguamente el compilador miraba las llaves(scopes)
   
    {
       let s=String::from("Hola");

       //usamos la variable creada antes de forma global r, donde toma la referencia a la variable s
       //tomamos un prestamo(borrowing)
       r=&s;      
    
       println!("{}",r);  //aqui no da error, ver abajo

       //2 FORMAS DE USAR STRING
       let s = String::from("Adios");  //de esta manera String es dueño, tipo propietario
       let t:&str="Adios majo"; //de esta manera &str tan solo una referencia a texto no un dueño

       //ambos con String y &str el texto Adios y Adios majo viven en el heap, 
       //las variables s es propietario y t referencia
       //la diferencia es que la variable s, al ser dueño puede crecer, modificarse, etc siguiendo las reglas del ownership
       //la variable t al no ser dueño solo apunta al texto Adios majo, y no se puede modificar es inmutable, no
       //reserva ni libera memoria cuando muere el texto muere la referencia
       //Rust lo hace de esta manera para separar de manera clara lo que es ser dueño a utilizar simplemente.
       //Esto da mas seguridad, menos copias, mas rendimientos y un control total de la memoria
       //ver (Ver imagenes video 7/String) 
    }

    //comento lo de abajo porque da error ya que queremos usar r fuera del bloque, como fura del bloque s no vive
    //ya no hay nadie que apunte a "Hola" en el heap,puntero colgante e c++ esto compilaria pero podria dar errores
    //println!("{}",r);
    
}
