/*
    ENUM
    - ¿Qué es un tipo enumerado?   
      un tipo enumerado (enum) es un tipo de dato definido por el usuario que limita una
      variable a un conjunto fijo y predefinido de valores constantes.

      -ejemplo:
        Enum EstadoCurso{
            Activo,
            Inactivo,
            Finalizado
        }

    - En este ejemplo la variable estado solo puede tomar uno de los tres valores definidos en el
      enum EstadoCurso: Activo, Inactivo o Finalizado. Esto ayuda a garantizar que la variable solo
      pueda tener valores válidos y predefinidos, evitando errores y mejorando la legibilidad del 
      código.
*/


//podemos crear un enum fuera de la funcion main, para que sea accesible desde cualquier parte del código,
// y podemos usarlo en cualquier parte del código, los enums pueden recibir parametros
enum Resultado{
        Aprobado(f64), //el parametro de Aprobado es un float, que representa la nota del estudiante, es el parametro que introducimos
        Suspenso(f64), //el parametro de Suspenso es un float, que representa la nota del estudiante, es el parametro que introducimos
}
fn main() {

    enum EstadoCurso {
        NoIniciado,
        EnProceso,
        Completado,
    }
    
    //le damos uno de los valores del enum a la variable estado, en este caso EnProceso
    let estado: EstadoCurso = EstadoCurso::EnProceso;

    //podemos usar el metodo match para evaluar el valor de la variable estado y ejecutar diferentes bloques 
    //de código según el valor del enum, como si fuera un switch case o un if else
    match estado {
        EstadoCurso::NoIniciado => println!("El curso no ha iniciado"),
        EstadoCurso::EnProceso => println!("El curso está en proceso"),
        EstadoCurso::Completado => println!("El curso ha sido completado"),
    }

    //ahora trabajamos con el enum Resultado, que es un enum definido fuera de la funcion main,
    let nota = 7.5;    //variable de una nota
    
    //creamos una variable llamada resultado, recibe paramatros este enum en concreto
    let resultado= if nota >= 5.0 {
        Resultado::Aprobado(nota)  //recibe el valor de la nota como parametro, que es un float
    } else {
        Resultado::Suspenso(nota)  //recibe el valor de la nota como parametro, que es un float
    };

    match resultado {
        Resultado::Aprobado(nota) => println!("El estudiante ha aprobado con una nota de {}", nota),
        Resultado::Suspenso(nota) => println!("El estudiante ha suspendido con una nota de {}", nota),
    }

}