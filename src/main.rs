use std::io;
use std::process;

fn main() {
    print_menu();
}

fn print_menu() {
    let mut page_selection: String = String::new();

    println!(r#"
    $$$$$$$\                        $$\           $$\      $$\                     $$\           
    $$  __$$\                       $$ |          $$$\    $$$ |                    \__|          
    $$ |  $$ |$$\   $$\  $$$$$$$\ $$$$$$\         $$$$\  $$$$ |$$\   $$\  $$$$$$$\ $$\  $$$$$$$\ 
    $$$$$$$  |$$ |  $$ |$$  _____|\_$$  _|        $$\$$\$$ $$ |$$ |  $$ |$$  _____|$$ |$$  _____|
    $$  __$$< $$ |  $$ |\$$$$$$\    $$ |          $$ \$$$  $$ |$$ |  $$ |\$$$$$$\  $$ |$$ /      
    $$ |  $$ |$$ |  $$ | \____$$\   $$ |$$\       $$ |\$  /$$ |$$ |  $$ | \____$$\ $$ |$$ |      
    $$ |  $$ |\$$$$$$  |$$$$$$$  |  \$$$$  |      $$ | \_/ $$ |\$$$$$$  |$$$$$$$  |$$ |\$$$$$$$\
    \__|  \__| \______/ \_______/    \____/       \__|     \__| \______/ \_______/ \__| \_______|"#);

    println!(r#"
    [1] Play
    [2] Leader Bosard
    [3] Quit"#);
    io::stdin()
        .read_line(&mut page_selection)
        .expect("Please select a valid option (1 - 3).");

    println!("You selected page: {}", page_selection);

    if page_selection == "3" {
        process::exit(0x100);
    }
}
