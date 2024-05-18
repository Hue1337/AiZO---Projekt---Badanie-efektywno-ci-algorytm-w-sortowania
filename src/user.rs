use std::io;

use crate::menu;
use crate::data_handler;
use crate::algorithms;
use crate::generator::Generator;


pub struct UserService {
    pub amount_numbers: u32, 
    pub size_data: u32,
    pub user_action: u32,
    pub data_type: u32,         // Wybrany typ danych
    pub algorithm_choice: u32,  // Wybrany algorytm
    pub generator: Generator,
    pub vec_data_i32: Vec<i32>,
    pub vec_data_i32_copy: Vec<i32>,
    pub vec_data_f32: Vec<f32>,
    pub vec_data_f32_copy: Vec<f32>,
    pub vec_data_f64: Vec<f64>,
    pub vec_data_f64_copy: Vec<f64>,
    pub filename: String,
}
impl UserService {
    pub fn new() -> UserService {
        UserService {
            amount_numbers: 0,
            size_data: 0,
            user_action: 1,
            data_type: 1, 
            algorithm_choice: 1,
            generator: Generator::new(),
            vec_data_i32: Vec::new(),
            vec_data_i32_copy: Vec::new(),
            vec_data_f32: Vec::new(),
            vec_data_f32_copy: Vec::new(),
            vec_data_f64: Vec::new(),
            vec_data_f64_copy: Vec::new(),
            filename: String::from("./data/default.csv"),
        }
    }

    pub fn start_service(&mut self) {
        // Pierwsze uruchomienie menu
        let mut data_type: u32;
        let mut user_action: u32;
        (self.data_type, self.user_action) = menu::run();
        loop {
            // Zapetlenie drugiego poziomu menu
            self.filename = String::from("./data/default.csv");
            match self.user_action {
                1 => {
                    // Wczytanie tablicy z pliku o zadanej nazwie.
                    self.filename = String::from("./data/");
                    let mut filename_tmp = String::new(); 

                    println!("\n\n1. Wczytanie tablicy z pliku o zadanej nazwie.\nPodaj nazwe pliku:");
                    io::stdin().read_line(&mut filename_tmp).expect("[-] Failed to read a line ...");
                    

                    self.filename += &filename_tmp;
                    self.remove_newline();
                    println!("{}", self.filename);
                    
                    match self.data_type {
                        1 => {
                            self.vec_data_i32 = data_handler::read_data_i32(self.filename.clone());
                        },

                        2 => {
                            self.vec_data_f32 = data_handler::read_data_f32(self.filename.clone());
                        },

                        3 => {
                            self.vec_data_f64 = data_handler::read_data_f64(self.filename.clone());
                        },

                        _ => {

                        }
                    }
                },

                2 => {
                    // Wygenerowanie tablicy o zadanym rozmiarze zawierającej losowe dane.
                    println!("\nPodaj rozmiar tablicy:");
                    self.generator.set_arr_size(algorithms::read_user_choice());


                    self.generator.run(self.data_type);
                    self.remove_newline();

                    match self.data_type {
                        1 => {
                            data_handler::write_data_i32(self.generator.get_arr_i32(), self.filename.clone());
                        },

                        2 => {
                            data_handler::write_data_f32(self.generator.get_arr_f32(), self.filename.clone());
                        },

                        3 => {
                            data_handler::write_data_f64(self.generator.get_arr_f64(), self.filename.clone());
                        },

                        _ => {
                            println!("[-] Failed to write data!");
                        }
                    }                    
                },

                3 => {
                    // Wyświetlenie ostatnio utworzonej tablicy
                    match self.data_type {
                        1 => {
                            println!("Ostatnio wygenerowana tablica:\n{:?}", self.generator.get_arr_i32());
                        },

                        2 => {
                            println!("Ostatnio wygenerowana tablica:\n{:?}", self.generator.get_arr_f32());
                        },

                        3 => {
                            println!("Ostatnio wygenerowana tablica:\n{:?}", self.generator.get_arr_f64());
                        },

                        _ => {
                            println!("[-] Error occurred! Failed to generate an array!");
                        }
                    }
                },

                4 => {
                    // Wybór algorytmu
                    self.algorithm_choice = algorithms::choose_algorithm();
                    
                },

                5 => {
                    // Wyświetl obecną konfigurację symulacji oraz liste dostepnych tablic
                    match self.data_type {
                        1 => {
                            println!("Typ danych: Integer");
                        },

                        2 => {
                            println!("Typ danych: Float");
                        },

                        3 => {
                            println!("Typ danych: Double");
                        },
                        
                        _ => {
                            println!("Error occured idk how you did that!");
                        }
                    }
                    
                    match self.algorithm_choice {
                        1 => {
                            println!("Wybrany algorytm: Sortowanie przez wstawianie (insertion sort).");
                        },

                        2 => {
                            println!("Wybrany algorytm: Sortowanie przez kopcowanie (heapsort).");
                        },

                        3 => {
                            println!("Wybrany algorytm: Sortowanie Shella.");
                        },

                        4 => {
                            println!("Wybrany algorytm: Sortowanie szybkie (Quicksort).");
                        },

                        _ => {
                            println!("Error occurred! Nie wiem jak to zrobiles xdddd.");
                        }
                    }

                    data_handler::list_avaible_arrays();
                },
                
                6 => {
                    // Uruchomienie symulacji
                    /*
                    To do:
                    */
                    match self.data_type {
                        1 => {
                            // Integer
                            let mut elapsed_time: std::time::Duration;
                            (self.vec_data_i32_copy, elapsed_time) = algorithms::match_algorithm_i32(self.algorithm_choice, self.data_type, self.generator.get_arr_i32().clone());
                            println!("Duration time of sorting: {:?}", elapsed_time);
                        },

                        2 => {
                            // Float
                            let mut elapsed_time: std::time::Duration;
                            (self.vec_data_f32_copy, elapsed_time) = algorithms::match_algorithm_f32(self.algorithm_choice, self.data_type, self.generator.get_arr_f32().clone());
                            println!("Duration time of sorting: {:?}", elapsed_time);                        
                        },

                        3 => {
                            // Double
                            let mut elapsed_time: std::time::Duration;
                            (self.vec_data_f64_copy, elapsed_time) = algorithms::match_algorithm_f64(self.algorithm_choice, self.data_type, self.generator.get_arr_f64().clone());
                            println!("Duration time of sorting: {:?}", elapsed_time);
                        },

                        _ => {
                            println!("[-] Failed to choose sorting algorithm!")
                        }
                    }
                },

                7 => {
                    // Zapisanie obecnie uzywanej tablicy do pliku o nazwie podanej przez uzytkownika
                    self.filename = String::from("./data/");
                    let mut filename_tmp: String = String::new();
                    
                    println!("Podaj nazwe pliku:");
                    io::stdin().read_line(&mut filename_tmp).expect("Failed to read a filename ...");
                    self.filename += &filename_tmp;
                    
                    println!("Plik zostanie zapisany w folderze ./data/");
                    self.remove_newline();

                    // Based on Data type:
                    match self.data_type {
                        1 => {
                            data_handler::write_data_i32(self.generator.get_arr_i32(), self.filename.clone());
                        },

                        2 => {
                            data_handler::write_data_f32(self.generator.get_arr_f32(), self.filename.clone());
                        },

                        3 => {
                            data_handler::write_data_f64(self.generator.get_arr_f64(), self.filename.clone());
                        },

                        _ => {
                            println!("[-] Error occurred! Failed to write data!")
                        }
                    }
                },  

                8 => {
                    // Powrót fo pierwszego poziomu menu
                    // menu::run();
                    self.data_type = menu::first_level_menu();
                    // continue;
                },

                9 => {
                    // Wyświetlenie posortowanej tablicy - jezeli zostala posortowana. Jezeli nie to zostanie wyswietlona
                    // nieposortowana.
                    match self.data_type {
                        1 => {
                            // Integer
                            println!("Posortowana tablica:\n{:?}", self.vec_data_i32_copy);
                        },

                        2 => {
                            // Float
                            println!("Posortowana tablica:\n{:?}", self.vec_data_f32_copy);
                        },

                        3 => {
                            // Double
                            println!("Posortowana tablica:\n{:?}", self.vec_data_f64_copy);
                        },

                        _ => {
                            println!("[-] Failed to print the array!");
                        }
                    }
                },

                10 => {
                    // Edge case'y
                    println!("Dostepne edge case'y:");
                    println!("\t1. Tablica posortowana rosnąco.");
                    println!("\t2. Tablica posortowana malejąco.");
                    println!("\t3. Tablica posortowana w 33%.");
                    println!("\t4. Tablica posortowana w 67%.");

                    let mut tmp_choice = algorithms::read_user_choice();

                    match tmp_choice {
                        1 => {
                            // Posortowana rosnąco.
                            match self.data_type {
                                1 => {
                                    // Integer
                                    self.generator.arr_i32.sort();
                                },

                                2 => {
                                    // Float
                                    self.generator.arr_f32.sort_by(|a, b| a.partial_cmp(b).unwrap());
                                },

                                3 => {
                                    // Double
                                    self.generator.arr_f64.sort_by(|a, b| a.partial_cmp(b).unwrap());
                                },

                                _ => {
                                    println!("Komunikat ktory nigdy sie nie wyswietli pozdro.");
                                }
                            }
                        },

                        2 => {
                            // Posortowana malejąco.
                            match self.data_type {
                                1 => {
                                    // Integer
                                    self.generator.arr_i32.sort();
                                    self.generator.arr_i32.reverse();
                                },

                                2 => {
                                    // Float
                                    self.generator.arr_f32.sort_by(|a, b| a.partial_cmp(b).unwrap());
                                    self.generator.arr_f32.reverse();
                                },

                                3 => {
                                    // Double
                                    self.generator.arr_f64.sort_by(|a, b| a.partial_cmp(b).unwrap());
                                    self.generator.arr_f64.reverse();
                                },

                                _ => {
                                    println!("Komunikat ktory nigdy sie nie wyswietli pozdro.");
                                }
                            }
                        },

                        3 => {
                            // Posortowana w 33%.
                            match self.data_type {
                                1 => {
                                    // Integer
                                    self.generator.arr_i32 = algorithms::sort_vec_percent_i32(self.generator.arr_i32.clone(), 0.33);
                                },

                                2 => {
                                    // Float
                                    self.generator.arr_f32 = algorithms::sort_vec_percent_f32(self.generator.arr_f32.clone(), 0.33);
                                },

                                3 => {
                                    // Double
                                    self.generator.arr_f64 = algorithms::sort_vec_percent_f64(self.generator.arr_f64.clone(), 0.33);
                                },

                                _ => {
                                    println!("Komunikat ktory nigdy sie nie wyswietli pozdro.");
                                }
                            }
                        },

                        4 => {
                            // Posortowana w 67%.
                            match self.data_type {
                                1 => {
                                    // Integer
                                    self.generator.arr_i32 = algorithms::sort_vec_percent_i32(self.generator.arr_i32.clone(), 0.67);
                                },

                                2 => {
                                    // Float
                                    self.generator.arr_f32 = algorithms::sort_vec_percent_f32(self.generator.arr_f32.clone(), 0.67);
                                },

                                3 => {
                                    // Double
                                    self.generator.arr_f64 = algorithms::sort_vec_percent_f64(self.generator.arr_f64.clone(), 0.67);
                                },

                                _ => {
                                    println!("Komunikat ktory nigdy sie nie wyswietli pozdro.");
                                }
                            }
                        },

                        _ => {
                            println!("[-] Failed to read a user's choice!");
                        }
                    }
                }

                11 => {
                    // Wyjscie z programu
                    return;
                },

                _ => {
                    // Default option
                    println!("[-] Niepoprawny wybor...");
                },
            }
            self.user_action = menu::second_level_menu();
        }
    }
    // Data handler
    fn remove_newline(&mut self) {
        if self.filename.ends_with('\n') {
            self.filename.pop();
            println!("Filename after pop: {}", self.filename);
        }
    }



    // Getters and setters
    pub fn set_data_type(&mut self, val: u32) {
        self.data_type = val;
    }

    pub fn set_user_action(&mut self, val: u32) {
        self.user_action = val;
    }

    pub fn set_amount_numbers(&mut self, val: u32) {
        self.amount_numbers = val;
    }

    pub fn set_size_data(&mut self, val: u32) {
        self.size_data = val;
    }

    pub fn get_data_type(&self) -> u32 {
        self.data_type
    }

    pub fn get_user_action(&self) -> u32 {
        self.user_action
    }

    pub fn get_amount_numbers(&self) -> u32 {
        self.amount_numbers
    }

    pub fn get_size_data(&self) -> u32 {
        self.size_data
    }
}