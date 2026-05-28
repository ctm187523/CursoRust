//programa de ejemplo que a partir de un vector con notas, decimos si esta aprobado, la media y la nota mas alta


fn main() {
    
    let notas = vec![7.5,4.1,8.2,9.1,3.8,6.4];

    //iter nos devuelve referencias no consumimos el vector original, por eso podemos usarlo varias veces
    //si usaramos into_iter() consumiriamos el vector original, por eso no podríamos usarlo varias veces, porque el vector original ya no estaría disponible, con into_iter() el tipo de aprobados seria Vec<f64>,
    let aprobados:Vec<&f64> = notas             //iter nos devuelve una referencia a cada elemento del vector, como es decimal es de tipo f64, por eso el tipo de aprobados es Vec<&f64>
    .iter()
    .filter(|n:&&f64|**n >= 5.0)                //filtramos las calificaciones mayores que 5, usamos funciones anomimas usamos &&n porque el iterador nos devuelve una referencia a cada elemento del vector, con el primer & desreferenciamos la referencia que nos da el iterador para obtener el valor del elemento del vector, con el segundo & desreferenciamos la referencia que nos da el filtro para obtener el valor del elemento del vector, como queremos comparar el valor del elemento del vector con 5.0, necesitamos desreferenciar dos veces, una para obtener el valor del elemento del vector y otra para obtener el valor del elemento del filtro, como queremos comparar el valor del elemento del vector con 5.0, necesitamos desreferenciar dos veces, una para obtener el valor del elemento del vector y otra para obtener el valor del elemento del filtro
    .collect();                                 //comn ** hemos desreferenciado dos veces, el tipo de aprobados es Vec<&f64>, con collect() convertimos el iterador en un vector, en este caso un vector de referencias a los elementos del vector original que cumplen la condicion, en este caso las calificaciones mayores que 5.0

    println!("Los aprobados son: {:?}", aprobados);

    //ahora hayamos la nota media, itereamos todos los elementos los sumamos y dividimos por la longitud del vector
    let media:f64 = notas.iter().sum::<f64>() / notas.len() as f64; //especificacmos as f64 

    println!("La nota media es: {:?}", media);

    //ahora hayamos la nota maxima, usamos con max_by una comparacion parcial porque podria aver un valor naN en el vector, con max_by() podemos especificar una funcion de comparacion, en este caso usamos partial_cmp() para comparar los valores de tipo f64, como partial_cmp() devuelve un Option<Ordering>, usamos unwrap() para obtener el valor de tipo Ordering, como el vector no esta vacio, el resultado seria Some(9.1)
    let nota_maxima:Option<&f64> = notas.iter().max_by(|a, b| a.partial_cmp(b).unwrap()); //usamos max_by() porque el tipo de dato es f64, con max_by() podemos especificar una funcion de comparacion, en este caso usamos partial_cmp() para comparar los valores de tipo f64, como partial_cmp() devuelve un Option<Ordering>, usamos unwrap() para obtener el valor de tipo Ordering, como el vector no esta vacio, el resultado seria Some(9.1)
    println!("La nota maxima es: {:?}", nota_maxima);

}
