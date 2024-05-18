use std::io;

pub fn run() -> (u32, u32) {
    /*
        Funkcja wywolujaca dwupoziomowe menu.
    */
    let choice_1: u32 = first_level_menu();
    let choice_2: u32 = second_level_menu();
    (choice_1, choice_2)
}

pub fn first_level_menu() -> u32{
    /*
        Pierwszy poziom menu.
    */
    let mut user_choice: u32;
    loop {
        print_first_level_menu();
        user_choice = read_user_choice();
        if user_choice < 4 && user_choice > 0 {
            println!("User choice: {}", user_choice);
            break;
        }
        println!("[-] Wrong choice!")
    }
    user_choice
}

fn print_first_level_menu() {
    println!("Wybor typu danych:");
    println!("\t1. Integer.");
    println!("\t2. Float.");
    println!("\t3. Double.");
}

pub fn second_level_menu() -> u32 {
    /*
        Drugi poziom menu.
    */
    let mut user_choice: u32;
    loop {
        print_second_level_menu();
        user_choice = read_user_choice();
        break;
    }
    user_choice
}  

fn print_second_level_menu() {
    println!("Menu:");
    println!("\t1. Wczytanie tablice z pliku o zadanej nazwie.");
    println!("\t2. Wygenerowanie tablicy o zadanych rozmiarze zawierającej losowe dane.");
    println!("\t3. Wyświetlenie ostatnio utworzonej tablicy.");
    println!("\t4. Wybór algorytmu.");
    println!("\t5. Wyswielt obecna konfiguracje symulacji.");
    println!("\t6. Uruchomienie symulacji.");
    println!("\t7. Zapisz obecnie uzywana tablice do pliku.");
    println!("\t8. Powrot do pierwszego poziomu menu (wybor typu danych).");
    println!("\t9. Wyświetlenie posortowanej tablicy. Jezeli nie zostala posortowana przez uzytkownika - zostanie wyswietlona nieposortowana.");
    println!("\t10. Wybor edge case'ow.");
    println!("\t11. Wyjście z progamu.");
}

fn read_user_choice() -> u32 {
    let mut tmp_val: String = String::new();
    io::stdin().read_line(&mut tmp_val).expect("Failed to read a line!");
    let val: u32 = match tmp_val.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Wrong number!");
            99
        }
    };
    val
}

