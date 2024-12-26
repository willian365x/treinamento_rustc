mod estudos {
    pub mod constantes;
    pub mod sombra;
    pub mod variaveis;
}

use estudos::constantes::constante;
use estudos::sombra::sombra;
use estudos::variaveis::variavel;

fn main() {
    variavel();
    constante();
    sombra();
}
