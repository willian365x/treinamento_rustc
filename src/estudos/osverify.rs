pub fn os_verify() {
    if cfg!(target_os = "windows") {
        println!("Este programa está sendo executado no Window");
    } else if cfg!(target_os = "linux") {
        println!("Este programa está sendo executado no Linux");
    } else {
        println!("Este programa está sendo executado em outro sistema operacional");
    }
}
