const NUMERO_DIAS: i8 = 30;
static NUMERO_CARTAO_CREDITO: i32 = 1452369854;
static NUMERO_PI: f64 = 3.14150;
const NOME_USUARIO: &str = "Willian";

pub fn constante() {
    println!("Usuário vigente: {NOME_USUARIO}");
    println!("Números de dias {}", NUMERO_DIAS);
    println!("Número do cartão de crédito: {}", NUMERO_CARTAO_CREDITO);
    println!("Número PI: {NUMERO_PI}");
}
