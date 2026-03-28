//ITERADORES
// FUNCIONES
// filter: filtra los elementos de una colección según una condición dada
// map: transforma los elementos de una colección aplicando una función a cada uno de ellos
// collect: convierte un iterador en una colección, como un vector o un conjunto, construye nuevas colecciones 
//          como un vector
// Se podria no usar estas funciones filter, map y collect y hacer lo mismo con bucles for, pero el uso de iteradores hace que el código sea más limpio y legible,
// facilita el codigo, simplicidad en el codigo si no las usamos el codigo sera de tipo
// IMPERATIVO ordenas directas, pero con el uso de iteradores el codigo es FUNCIONAL 


fn main( ){

    let numeros=vec![1,2,3,4,5,6];

    //forma IMPERATIVA queremos buscar los numeros pares usamos el modulo % para ver si es par o impar
    //extraemos los numeros pares y los multiplicamso por 2 almacenados en otroa vector mutable llamado resultado
    let mut resultado=Vec::new();
    for n in  &numeros {
        if n % 2 == 0 {
            resultado.push(n * 2);   //alamacenamos los valores pares multiplicados por 2
        }

    }
    
    println!("{:?}", resultado);

    println!("-----------------------------------------------------------------");
    println!("Ejemplo con iteradores: ");
    //Ahora usamos las funciones de iteradores, forma Funcional
    //VER IMAGEN EN C:\Users\cleme\Documents\Rust\CursoPildorasInformaticas\ImagenesVideos\VIDEO_13\Iteradores_Funciones
    //creamos un iterador sobre el vector numeros
    //Para filtrar usamos la funcion clousure o funcion anonima, seria equivalente en JavaScript con la funcion flecha
    //             o las funciones lambda de otros lengajes como Python, Java y C#. 
    //             En Rust seria |n|, esto seria una funcion anonima o arrow function
    //Para transformar usamos la funcion map y usamos tambien una funcion anonima o clousure como en el filter
    //             En Rust seria |n|, esto seria una funcion anonima o arrow function
    //No estamos modificando el vector original numeros, hacemos operacion con el, necesitamos el equivalente
    //A lo que hicimos anteriormente con el vector resultado, almacenar todas la operaciones en un vector
    //Para ello Usamos la funcion collect almacena los resultados en el ºVector llamado resultado2
    let resultado2:Vec<i32> = numeros.iter()
                            .filter(|n|*n%2==0)
                            .map(|n|n*2)
                            .collect();
    println!("{:?}", resultado2);   
}
