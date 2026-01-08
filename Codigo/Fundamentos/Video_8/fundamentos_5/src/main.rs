// ¿Que es un slice?
// referencia a parte de una coleccion de datos(ej un String, array, etc) que ya existe
// Permite ver parte de los datos sin copiarlos en memoria y sin ser dueño, importantes en rendimiento y eficiencia
// El slice tiene borowing y lifetime

// ¿Para que sirve un slice?
// Para tener mayor eficiencia y seguridad.¿Cómo?
// -Evitando copias innecesarias en memoria
// -Pasando datos a funciones de forma segura
// -Permitiendo APIs más flexibles.

//En resumen: Un slice es una forma segura y eficiente de trabajar con partes de datos sin ser dueño de ellos.

fn main() {
    
    //creamos un string para usar un slice en el
    let s=String::from("Hola alumnos");
    
    //queremos crear un slice a la palabra hola del string s
    //para ello creamos un puntero & al string s y ponemos el rango de lo que queremos obtener
    //ver imagen en Video_8 (slice)
    //saludo no apunta a s apunta a una parte de su contenido en el heap concretamente a la palabra hola
    //sin hacer copias en el heap de hola, el dueño es s de hola alumnos y saludo apunta a una parte del heap donde 
    //es apuntado por s, guarda saludo un puntero y una longitud
    let saludo=&s[0..4];

    //Imprimimos la parte seleccionado en el slice creado
    println!("Es un slice de s {}", saludo);

    //creamos un array
    let numeros = [1,2,3,4,5];

    //creamos un slice que apunte a 2,3,4 de numeros
    let parte = &numeros[1..4];

    // trait es una definicion de comportamiento, dice que puede hacer ese dato(tipo de dato) y no lo que es
    //un array no podemos imprimirlo en consola como hicimos con el string de arriba con {}->   println!("Es un slice de s {}", saludo);
    // tenemos que especificar ese tipo con un trait
    // los String, los int implementan un comportamiento llamado Display pero los array no implementan
    //este comportamiento Display
    //los arrays no son capaces de mostrarse en consola porque no tienen el comportamiento Display
    //en lugar de {} debemos usar {:?}, esto no le da un comportamiento Display da un comportamiento Debug
    //que nos sirve para que un array pueda ser imprimido, le damos un trait(un comportamiento)
    println!("{:?}", parte);

}
