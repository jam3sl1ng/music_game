use std::io;
use std::process;

fn main() {
    init_page();
}

fn print_ascii() {
    println!(r#"
    $$$$$$$\                        $$\           $$\      $$\                     $$\           
    $$  __$$\                       $$ |          $$$\    $$$ |                    \__|          
    $$ |  $$ |$$\   $$\  $$$$$$$\ $$$$$$\         $$$$\  $$$$ |$$\   $$\  $$$$$$$\ $$\  $$$$$$$\ 
    $$$$$$$  |$$ |  $$ |$$  _____|\_$$  _|        $$\$$\$$ $$ |$$ |  $$ |$$  _____|$$ |$$  _____|
    $$  __$$< $$ |  $$ |\$$$$$$\    $$ |          $$ \$$$  $$ |$$ |  $$ |\$$$$$$\  $$ |$$ /      
    $$ |  $$ |$$ |  $$ | \____$$\   $$ |$$\       $$ |\$  /$$ |$$ |  $$ | \____$$\ $$ |$$ |      
    $$ |  $$ |\$$$$$$  |$$$$$$$  |  \$$$$  |      $$ | \_/ $$ |\$$$$$$  |$$$$$$$  |$$ |\$$$$$$$\
    \__|  \__| \______/ \_______/    \____/       \__|     \__| \______/ \_______/ \__| \_______|"#);
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn init_page() {
    clear_screen();

    let mut page_selection: String = String::new();

    print_ascii();

    println!(r#"
    [1] Login
    [2] Quit"#);
    io::stdin()
        .read_line(&mut page_selection)
        .expect("Please select a valid option (1 - 3).");

    match page_selection.trim() {
        "1" => print_menu(),
        "2" => process::exit(0x100),
        _ => println!("Invalid selection. Please try again."),
    }
}

fn print_menu() {
    clear_screen();

    let mut page_selection: String = String::new();

    print_ascii();

    println!(r#"
    [1] Play
    [2] Leader Bosard
    [3] Logout"#);
    io::stdin()
        .read_line(&mut page_selection)
        .expect("Please select a valid option (1 - 3).");

    match page_selection.trim() {
        "1" => print_menu(),
        "2" => print_menu(),
        "3" => init_page(),
        _ => println!("Invalid selection. Please try again."),
    }
}
