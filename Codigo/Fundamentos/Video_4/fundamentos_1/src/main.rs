fn main() {
    //println! con el signo de admiracion es una macro sustituye codigo en tiempo de compilacion no en tiempo de ejecucion
    //Se expande en tiempo de compilación: el compilador sustituye la macro por el código que genera
    //Se invocan con !
    //println!("Hello, world!");   

    let edad = 30;   //declaramos una variable con let, Rust tiene inferencia de datos no es necesario poner el tipo de variable al declararla por defecto un numero es i32(32 bits con signo)
    let edad2: u8 = 25; //para tipar la variable usamos los dos puntos y el tipo de dato en estecado de 8 bits unsigned(sin signo)

    let mensaje = "Hola";       //String no es primitivo

    let precio = 19.99;         //decimales por defecto f64

    let activa=true;           //booleanos

    //LOS VALORES EN RUST SON INMUTABLES SI QUEREMOS CAMBIAR EL VALOR DE UNA VARIABLE NOS DA ERROR
    //POR EJEMPLO SI HEMOS DECLARADO let edad = 30; no podemos poner edad = 50; no dara error no se puede coambiar el valor
    //PARA TRANFORMAR UNA VARIABLE EN MUTABLE SE UTILIZA LA PALABRA RESERVADA mut, se usara mas adelante

    //RUST TRABAJA A UN NIVEL BAJO CON EL HARDWARE PROCESADOR Y MEMOIRA
    //CUANDO USAMOS LET NO SOLO RESERVAMOS UN ESPACIO EN LA MEMORIA RAM DEL ORDENADOR, CREAMOS ADEMAS UN VINCULO
    //ENTRE EL NOMBRE DE LA VARIABLE Y EL VALOR ALMACENADO, VINCULO LLAMADO BINDING, EN JAVA POR EJEMPLO 
    //EN LA MEMORIA RAM TIENE DOS ZONAS EL STACK(PILA) Y EL HEAP(MONTON), CUANDO USAMOS UN STRING YA QUE STRING ES UN
    //OBJETO CREAMOS UNA VARIABLE POR EJEMPLO NOMBRE, ESTA VARIABLE SE ALMACENA EN EL STACK Y SU VALOR POR EJEMPLO
    //JUAN EN EL HEAP HAY UNA REFERENCIA DEL NOMBRE DE LA VARIABLE AL VALOR, SI CREAMOS OTRA VARIABLE DE TIPO STRING
    //Y USAMOS UNA VARIABLE YA CREADA EN EL EJEMPLO NOMBRE Y HACEMOS String dato=nombre; TENEMOS AHORA EN EL SATCK LAS 
    //VARIABLES NOMBRE Y DATO QUE TIENE COMO REFERENCIA EN EL HEAP EL MISMO VALOR JUAN, ESTO QUE IMPLICA QUE SI QUEREMOS
    //IMPRIMIR TANTO NOMBRE COMO DATO -> system.out.println(dato) o system.out.println(nombre) tendremos el mismo
    //RESULTADO QUE SERA JUAN(VER EN CARPETA VIDEO 4 DE ImagenesVideos)
    //
    //EN RUST FUNCIONA DIFERENTE CON EL BINDING, COMO EN JAVA SI DECLARAMOS UNA VARIABLE CON LET EJ:
    //let nombre: String=String::from("Juan"); EL NOMBRE DE LA VARIABLE NOMBRE PASA AL STACK Y EL VALOR JUAN AL HEAP
    //LA DIFRENCIA ES QUE EL LET CREA ESE BINDING Y CONVIERTE A LA VARIABLE NOMBRE DE DUEÑA DEL VALOR JUAN
    //SI AHORA COMO VIMOS EN EL EJEMPLO DE JAVA CREAMOS CON LET LA VARIABLE DATO Y LE ASIGNAMOS EL VALOR DE NOMBRE
    //let dato: String = nombre; APUNTA A LA POSICION DE MEMORIA DEL HEAP DONDE SE CREO EL VALOR JUAN PERO IMPLICA
    //QUE AHORA CAMBIA EL DUEÑO LA VARIABLE NOMBRE YA NO ES EL DUEÑO DEL VALOR JUAN SI NO QUE AHORA ES LA VARIABLE DATO
    //ESTO IMPLICA QUE NOMBRE YA NO PUEDE ACCEDER A ESE VALOR Y SI QUEREMOS IMPRIMIR EL VALOR DE NOMBRE NO LO VA A IMPRIMIR
    //YA QUE AHORA NO ES EL DUENO DEL VALOR DE LA VARIABLE(VER EN CARPETA VIDEO 4 DE ImagenesVideos) 
    //ESTO SOLO PASA CUANDO TRABAJAMOS CON OBJETOS, NO CON PRIMITIVOS, CON PRIMITIVOS SE COMPORTA COMO CON OTROS LENGUAJES
    //ESTO LO HACE PORQUE COMO RUST TRABAJA A NIVEL BAJO CON PUNTEROS, ETC Y DEBEMOS SABER COMO FUNCIONA LA MEMORIA

    /*
        El ownership (propiedad) es el concepto central de Rust para manejar la memoria sin garbage collector y sin fugas. Es la base del sistema de seguridad de Rust.
        Te lo explico de forma clara y con ejemplos:

        🎯 ¿Qué es el ownership?

        Rust tiene tres reglas:

        Cada valor tiene un único owner (dueño).

        Solo puede haber un owner a la vez.

        Cuando el owner sale de scope, el valor se libera automáticamente.

        Esto evita fugas de memoria y condiciones de carrera sin necesidad de GC y sin free() manual
     */

    //vemos el ejemplo de la explicacion anterior
    let nombre = String::from("Juan");
    let dato = nombre;
    println!("{}", dato);
}
