use std::io;
use std::time::Instant;

pub fn choose_algorithm() -> u32 {

    println!("Wybierz algorytm, który chcesz użyć:");
    println!("\t1. Sortowanie przez wstawianie (insertion sort).");
    println!("\t2. Sortowanie przez kopcowanie (heapsort).");
    println!("\t3. Sortowanie Shella.");
    println!("\t4. Sortowanie szybkie (quicksort).");
    println!("\t5. Powrót.");

    let mut choice = read_user_choice();
    choice
}

// MATCHING ALGORITHMS CAUSE GENERIC TYPES IN RUST SUCK

pub fn match_algorithm_i32(choice: u32, data_type: u32, data: Vec<i32>) -> (Vec<i32>, std::time::Duration) {
    match choice {
        1 => {
            // Insertion sort
            let start_time = Instant::now();
            let mut tmp_vec = insertion_sort_i32(data);
            let elapsed_time = start_time.elapsed();
            (tmp_vec, elapsed_time)
        },

        2 => {
            // Heapsort
            let start_time = Instant::now();
            let mut tmp_vec = heapsort_i32(data);
            let elapsed_time = start_time.elapsed();
            (tmp_vec, elapsed_time)
        },

        3 => {
            // Shell
            let start_time = Instant::now();
            let mut tmp_vec = shell_sort_i32(data);
            let elapsed_time = start_time.elapsed();
            (tmp_vec, elapsed_time)
        },

        4 => {
            // Quicksort
            let start_time = Instant::now();
            let mut tmp_vec = quicksort_i32(data);
            let elapsed_time = start_time.elapsed();
            (tmp_vec, elapsed_time)
        },

        _ => {
            println!("[-] Niepoprawny wybor, powrot do menu...");
            return (Vec::new(), std::time::Duration::new(10, 10));
        },
    }
}

pub fn match_algorithm_f32(choice: u32, data_type: u32, data: Vec<f32>) -> (Vec<f32>, std::time::Duration) {
    match choice {
        1 => {
            // Insertion sort
            let start_time = Instant::now();
            let mut tmp_vec = insertion_sort_f32(data);
            let elapsed_time = start_time.elapsed();
            (tmp_vec, elapsed_time)
        },

        2 => {
            // Heapsort
            let start_time = Instant::now();
            let mut tmp_vec = heapsort_f32(data);
            let elapsed_time = start_time.elapsed();
            (tmp_vec, elapsed_time)
        },

        3 => {
            // Shell
            let start_time = Instant::now();
            let mut tmp_vec = shell_sort_f32(data);
            let elapsed_time = start_time.elapsed();
            (tmp_vec, elapsed_time)
        },

        4 => {
            // Quicksort
            let start_time = Instant::now();
            let mut tmp_vec = quicksort_f32(data);
            let elapsed_time = start_time.elapsed();
            (tmp_vec, elapsed_time)
        },

        _ => {
            println!("[-] Niepoprawny wybor, powrot do menu...");
            return (Vec::new(), std::time::Duration::new(10, 10));
        },
    }
}

pub fn match_algorithm_f64(choice: u32, data_type: u32, data: Vec<f64>) -> (Vec<f64>, std::time::Duration) {
    match choice {
        1 => {
            // Insertion sort
            let start_time = Instant::now();
            let mut tmp_vec = insertion_sort_f64(data);
            let elapsed_time = start_time.elapsed();
            (tmp_vec, elapsed_time)
        },

        2 => {
            // Heapsort
            let start_time = Instant::now();
            let mut tmp_vec = heapsort_f64(data);
            let elapsed_time = start_time.elapsed();
            (tmp_vec, elapsed_time)
        },

        3 => {
            // Shell
            let start_time = Instant::now();
            let mut tmp_vec = shell_sort_f64(data);
            let elapsed_time = start_time.elapsed();
            (tmp_vec, elapsed_time)
        },

        4 => {
            // Quicksort
            let start_time = Instant::now();
            let mut tmp_vec = quicksort_f64(data);
            let elapsed_time = start_time.elapsed();
            (tmp_vec, elapsed_time)
        },

        _ => {
            println!("[-] Niepoprawny wybor, powrot do menu...");
            return (Vec::new(), std::time::Duration::new(10, 10));
        },
    }
}


// ALGORITHMS

// ################################################################################
// i32
// ################################################################################

fn insertion_sort_i32(data: Vec<i32>) -> Vec<i32> {
    let mut data_mut_copy: Vec<i32> = data.clone();
    for i in 1..data_mut_copy.len() {
        let key = data_mut_copy[i];
        let mut j = i-1;
        while j > 0 && key < data_mut_copy[j as usize] {
            data_mut_copy[j as usize + 1] = data_mut_copy[j as usize];
            j-=1;
        }
        data_mut_copy[(j+1) as usize] = key;
    }
    data_mut_copy
}

fn heapify_i32(data: &mut [i32], n: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && data[left] > data[largest] {
        largest = left;
    }

    if right < n && data[right] > data[largest] {
        largest = right;
    }

    if largest != i {
        data.swap(i, largest);
        heapify_i32(data, n, largest);
    }
}

fn heapsort_i32(data_1: Vec<i32>) -> Vec<i32> {
    let mut data = data_1.clone();
    let n = data.len();

    if n == 0 {
        return data;
    }

    for i in (0..n / 2).rev() {
        heapify_i32(&mut data, n, i);
    }

    for i in (1..n).rev() {
        data.swap(0, i);
        heapify_i32(&mut data, i, 0);
    }

    data
}

fn shell_sort_i32(data_1: Vec<i32>) -> Vec<i32> {
    let mut data = data_1.clone();
    let n = data.len();
    let mut gap = n / 2;

    while gap > 0 {
        for i in gap..n {
            let mut j = i;
            let temp = data[i];

            while j >= gap && data[j - gap] > temp {
                data[j] = data[j - gap];
                j -= gap;
            }

            data[j] = temp;
        }

        gap /= 2;
    }

    data
}

fn partition_i32(data: &mut [i32]) -> usize {
    let pivot_index = data.len() - 1;
    let mut i = 0;

    for j in 0..pivot_index {
        if data[j] <= data[pivot_index] {
            data.swap(i, j);
            i += 1;
        }
    }

    data.swap(i, pivot_index);
    i
}

fn quicksort_i32(data_1: Vec<i32>) -> Vec<i32> {
    let mut data = data_1.clone();
    if data.len() <= 1 {
        return data;
    }

    let pivot_index = partition_i32(&mut data);

    let (left, right) = data.split_at_mut(pivot_index);

    let mut sorted_left = quicksort_i32(left.to_vec());
    let sorted_right = quicksort_i32(right[1..].to_vec());

    sorted_left.push(data[pivot_index]);
    sorted_left.extend(sorted_right);

    sorted_left
}

// ################################################################################
// f32
// ################################################################################

fn insertion_sort_f32(data: Vec<f32>) -> Vec<f32> {
    let mut data_mut_copy: Vec<f32> = data.clone();
    for i in 1..data_mut_copy.len() {
        let key = data_mut_copy[i];
        let mut j = i-1;
        while j > 0 && key < data_mut_copy[j as usize] {
            data_mut_copy[j as usize + 1] = data_mut_copy[j as usize];
            j-=1;
        }
        data_mut_copy[(j+1) as usize] = key;
    }
    data_mut_copy
}

fn heapify_f32(data: &mut [f32], n: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && data[left] > data[largest] {
        largest = left;
    }

    if right < n && data[right] > data[largest] {
        largest = right;
    }

    if largest != i {
        data.swap(i, largest);
        heapify_f32(data, n, largest);
    }
}

fn heapsort_f32(data_1: Vec<f32>) -> Vec<f32> {
    let mut data = data_1.clone();
    let n = data.len();

    if n == 0 {
        return data;
    }

    for i in (0..n / 2).rev() {
        heapify_f32(&mut data, n, i);
    }

    for i in (1..n).rev() {
        data.swap(0, i);
        heapify_f32(&mut data, i, 0);
    }

    data
}

fn shell_sort_f32(data_1: Vec<f32>) -> Vec<f32> {
    let mut data = data_1.clone();
    let n = data.len();
    let mut gap = n / 2;

    while gap > 0 {
        for i in gap..n {
            let mut j = i;
            let temp = data[i];

            while j >= gap && data[j - gap] > temp {
                data[j] = data[j - gap];
                j -= gap;
            }

            data[j] = temp;
        }

        gap /= 2;
    }

    data
}

fn partition_f32(data: &mut [f32]) -> usize {
    let pivot_index = data.len() - 1;
    let mut i = 0;

    for j in 0..pivot_index {
        if data[j] <= data[pivot_index] {
            data.swap(i, j);
            i += 1;
        }
    }

    data.swap(i, pivot_index);
    i
}

fn quicksort_f32(data_1: Vec<f32>) -> Vec<f32> {
    let mut data = data_1.clone();
    if data.len() <= 1 {
        return data;
    }

    let pivot_index = partition_f32(&mut data);

    let (left, right) = data.split_at_mut(pivot_index);

    let mut sorted_left = quicksort_f32(left.to_vec());
    let sorted_right = quicksort_f32(right[1..].to_vec());

    sorted_left.push(data[pivot_index]);
    sorted_left.extend(sorted_right);

    sorted_left
}

// ################################################################################
// f64
// ################################################################################

fn insertion_sort_f64(data: Vec<f64>) -> Vec<f64> {
    let mut data_mut_copy: Vec<f64> = data.clone();
    for i in 1..data_mut_copy.len() {
        let key = data_mut_copy[i];
        let mut j = i-1;
        while j > 0 && key < data_mut_copy[j as usize] {
            data_mut_copy[j as usize + 1] = data_mut_copy[j as usize];
            j -= 1;
        }
        data_mut_copy[(j+1) as usize] = key;
    }
    data_mut_copy
}

fn heapify_f64(data: &mut [f64], n: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && data[left] > data[largest] {
        largest = left;
    }

    if right < n && data[right] > data[largest] {
        largest = right;
    }

    if largest != i {
        data.swap(i, largest);
        heapify_f64(data, n, largest);
    }
}

fn heapsort_f64(data_1: Vec<f64>) -> Vec<f64> {
    let mut data = data_1.clone();
    let n = data.len();

    if n == 0 {
        return data;
    }

    for i in (0..n / 2).rev() {
        heapify_f64(&mut data, n, i);
    }

    for i in (1..n).rev() {
        data.swap(0, i);
        heapify_f64(&mut data, i, 0);
    }

    data
}

fn shell_sort_f64(data_1: Vec<f64>) -> Vec<f64> {
    let mut data = data_1.clone();
    let n = data.len();
    let mut gap = n / 2;

    while gap > 0 {
        for i in gap..n {
            let mut j = i;
            let temp = data[i];

            while j >= gap && data[j - gap] > temp {
                data[j] = data[j - gap];
                j -= gap;
            }

            data[j] = temp;
        }

        gap /= 2;
    }

    data
}

fn partition_f64(data: &mut [f64]) -> usize {
    let pivot_index = data.len() - 1;
    let mut i = 0;

    for j in 0..pivot_index {
        if data[j] <= data[pivot_index] {
            data.swap(i, j);
            i += 1;
        }
    }

    data.swap(i, pivot_index);
    i
}

fn quicksort_f64(data_1: Vec<f64>) -> Vec<f64> {
    let mut data = data_1.clone();
    if data.len() <= 1 {
        return data;
    }

    let pivot_index = partition_f64(&mut data);

    let (left, right) = data.split_at_mut(pivot_index);

    let mut sorted_left = quicksort_f64(left.to_vec());
    let sorted_right = quicksort_f64(right[1..].to_vec());

    sorted_left.push(data[pivot_index]);
    sorted_left.extend(sorted_right);

    sorted_left
}

// ####################################################
// Sortowanie w danym procencie
// ####################################################

pub fn sort_vec_percent_i32(vec_1: Vec<i32>, percent: f64) -> Vec<i32> {
    let mut vec = vec_1.clone();
    let len = vec.len();
    let mut sorted_vec = Vec::with_capacity(len);
    let mut chunk_size = (len as f64 * percent).ceil() as usize;


    vec.sort();

    let mut chunk = vec.drain(..chunk_size).collect::<Vec<i32>>();

    for &num in &vec {
        if let Some(idx) = chunk.iter().position(|&x| x <= num) {
            chunk.insert(idx, num);
        } else {
            chunk.push(num);
        }
    }

    sorted_vec.extend(chunk);

    sorted_vec
}

pub fn sort_vec_percent_f32(vec_1: Vec<f32>, percent: f64) -> Vec<f32> {
    let mut vec = vec_1.clone();
    let len = vec.len();
    let mut sorted_vec = Vec::with_capacity(len);
    let mut chunk_size = (len as f64 * percent).ceil() as usize;


    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut chunk = vec.drain(..chunk_size).collect::<Vec<f32>>();

    for &num in &vec {
        if let Some(idx) = chunk.iter().position(|&x| x <= num) {
            chunk.insert(idx, num);
        } else {
            chunk.push(num);
        }
    }

    sorted_vec.extend(chunk);

    sorted_vec
}

pub fn sort_vec_percent_f64(vec_1: Vec<f64>, percent: f64) -> Vec<f64> {
    let mut vec = vec_1.clone();
    let len = vec.len();
    let mut sorted_vec = Vec::with_capacity(len);
    let mut chunk_size = (len as f64 * percent).ceil() as usize;


    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut chunk = vec.drain(..chunk_size).collect::<Vec<f64>>();

    for &num in &vec {
        if let Some(idx) = chunk.iter().position(|&x| x <= num) {
            chunk.insert(idx, num);
        } else {
            chunk.push(num);
        }
    }

    sorted_vec.extend(chunk);

    sorted_vec
}


pub fn read_user_choice() -> u32 {
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

