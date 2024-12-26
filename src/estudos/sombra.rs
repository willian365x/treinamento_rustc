pub fn sombra() {
    let x: i8 = 5;
    println!("Resultado: {} - {:p}", x, &x);

    let x: i8 = x + 5;
    println!("Resultado: {} - {:p}", x, &x);

    let x: i8 = x * 2;
    println!("Resultado: {} - {:p}", x, &x);
}
