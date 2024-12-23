const NUMERO_DIAS: i8 = 30;
static mut NUMERO_CARTAO_CREDITO: i32 = 1452369854;
static NUMERO_PI: f64 = 3.14150;

pub fn constante() {
    println!("Números de dias {}", NUMERO_DIAS);

    unsafe {
        NUMERO_CARTAO_CREDITO = 125654321;
        println!("Número do cartão de crédito: {}", NUMERO_CARTAO_CREDITO);
    }

    println!("Número PI: {NUMERO_PI}");
}
