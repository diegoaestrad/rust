fn main() {
    println!("Hello, world!");
    let x = 5;          // variable inmutable
    println!("x = {}", x);

    let mut y = 10;     // variable mutable
    println!("y = {}", y);
    y = 15;
    println!("y ahora es = {}", y);

    let nam:char = 'p';
    println!("nam = {} ", nam);

    let numero = 7;

    if numero < 5 {
        println!("Es menor que 5");
    } else {
        println!("Es mayor o igual a 5");
    }

    // Bucle for
    for i in 0..3 {
        println!("i = {}", i);
    }

    // Bucle while
    let mut contador = 0;
    while contador < 3 {
        println!("contador = {}", contador);
        contador += 1;
    }

    let resultado = sumar(2, 3);
    println!("2 + 3 = {}", resultado);

    let s = String::from("Hola");
    println!("{}", s); // OK

    validar(resultado);
    println!("{}",resultado);

    println!("{}",validar(resultado));

    let s1 = String::from("Hola");
    let s2 = s1.clone(); // ahora s1 y s2 son independientes

    // let s2 = s1;

    // move
    println!("{}", s1); // ❌ ERROR: `s1` ya no es válido
    println!("{}", s2); // ✅ OK

    // borrowing 
    let s3= String::from("Hola");

    let len = calcular_longitud(&s3); // prestamos s1
    println!("La longitud de '{}' es {}", s3, len); // ✅ OK


    let mut s = String::from("Hola");
    cambiar(&mut s);
    println!("{}", s); // ✅ Muestra "Hola, mundo"
    

}

fn sumar(a: i32, b: i32) -> i32 {
    a + b  // la última expresión es el valor retornado sin usar `return`
}

fn validar(a: i32) -> i32 {
    a + 1 
}

fn calcular_longitud(s: &String) -> usize {
    s.len()
}

fn cambiar(s: &mut String) {
    s.push_str(", mundo");
}

//i32 — entero con signo de 32 bits
//u32 — entero sin signo
//f64 — número flotante
//bool — booleano (true/false)
//char — carácter Unicode