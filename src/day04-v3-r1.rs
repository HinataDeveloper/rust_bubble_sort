fn main() {
    println!("\n\n");

    let args: Vec<String> = std::env::args().collect();
    let mut ss: Sorting = Sorting::Unknown;

    if args.len() == 4 {
        ss = check_sort_style(&args[3]);
    }

    let args_result = check_args(args);

    let mut number_vector = match args_result {
        Err(error_message) => {
            usage();
            panic!("Error: {error_message}");
        }
        Ok(vec) => vec,
    };

    sort(&mut number_vector, ss);
    print_vector(&number_vector);

    println!("\n The End ... (0.0.1)\n");
}

enum Sorting {
    Ascending,
    Descending,
    Unknown,
}

fn get_number_list(str_num: &str) -> Vec<i32> {
    let part1 = str_num.replace("[", "");
    let part2 = part1.replace("]", "");
    let str_num_list = part2.split(",");
    let mut number_list: Vec<i32> = Vec::new();
    for item in str_num_list {
        let num: i32 = item.parse().expect("invalid alpha number ...");
        number_list.push(num);
    }
    number_list
}

/// check the first command line parameter
/// should start with '[' and ends with ']'
fn check_number_list(num_list: &str) -> bool {
    let mut char_group = num_list.chars();
    char_group.next().unwrap() == '[' && char_group.last().unwrap() == ']'
}

/// check the second command line parameter
/// should be "sort"
fn check_sort_command(command: &str) -> bool {
    command == "sort"
}

/// check sort style should be "asc" or "desc"
fn check_sort_style(sort_style: &str) -> Sorting {
    if sort_style == "asc" {
        Sorting::Ascending
    } else if sort_style == "desc" {
        Sorting::Descending
    } else {
        Sorting::Unknown
    }
}

fn usage() {
    println!(
        r"
    usage:
        bubble_sort [n1,n2,n3,...] sort [asc, desc]
        "
    );
}

/// validate command line parameters
fn check_args(input_args: Vec<String>) -> Result<Vec<i32>, String> {
    // check the number of command line parameters
    if input_args.len() != 4 {
        return Err(String::from("invalid command line parameters ..."));
    }

    // start with '[' and ends with ']'
    if !check_number_list(&input_args[1]) {
        return Err(String::from("invalid number list ..."));
    }

    // second command line parameter should be "sort"
    if !check_sort_command(&input_args[2]) {
        return Err(String::from("invalid command sort ..."));
    }

    // return sort style from Sorting enumeration
    let _ = match check_sort_style(&input_args[3]) {
        Sorting::Unknown => return Err(String::from("invalid sort style ...")),
        _ => true,
    };

    // create a vector of number
    let num_list = get_number_list(&input_args[1]);
    Ok(num_list)
}

/// print all numbers of input vector
fn print_vector(vec_num: &Vec<i32>) {
    for item in vec_num {
        println!(" -> {}", item);
    }
}

/// sort the array of numbers
fn sort(data: &mut [i32], sort_mode: Sorting) {
    let should_swap = match sort_mode {
        Sorting::Ascending => |a: i32, b: i32| a > b,
        Sorting::Descending => |a: i32, b: i32| a < b,
        Sorting::Unknown => |a: i32, b: i32| a > b,
    };

    let length = data.len() - 1;
    loop {
        let mut swapped = false;

        for index in 0..length {
            if should_swap(data[index], data[index + 1]) {
                data.swap(index, index + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}
