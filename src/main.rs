mod estudos {
    pub mod constantes;
    pub mod sombra;
    pub mod variaveis;
    pub mod osverify;
}

use estudos::constantes::constante;
use estudos::sombra::sombra;
use estudos::variaveis::variavel;
use estudos::osverify::os_verify;

fn main() {

    variavel();
    #[cfg(target_os = "linux")]
    constante();
    sombra();
    os_verify();
}
