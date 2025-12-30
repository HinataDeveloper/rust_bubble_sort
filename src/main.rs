use std::env;

fn main() {
    println!("\n");

    const SORT: &str = "sort";
    const ASC: &str = "asc";
    const DESC: &str = "desc";

    let order: Vec<String> = env::args().collect();

    if order.len() != 3 {
        error_message(" -> invalid number of parameters parameter ...");
        return;
    }

    if order[1].trim() != SORT {
        error_message(" -> sort param not found ...");
        return;
    }

    if order[2].trim() != ASC && order[2].trim() != DESC {
        error_message(" -> sorting style param not found ...");
        return;
    }

    let order = order[2].as_str().trim();

    let should_swap = match order {
        ASC => |a: i32, b: i32| a > b,
        DESC => |a: i32, b: i32| a < b,
        _ => {
            error_message("invalid ordering style ");
            return;
        }
    };

    let mut number_list = vec![
        10, 56, 34, 75, 82, 21, 45, 62, 86, 95, 74, 91, 203, 58, 401, 301,
    ];

    let length = number_list.len() - 1;

    'outer: loop {
        let mut swapped = false;

        for index in 0..length {
            if should_swap(number_list[index], number_list[index + 1]) {
                number_list.swap(index, index + 1);
                swapped = true;
            }
        }

        if !swapped {
            break 'outer;
        }
    }

    print_vector(&number_list);

    println!("\n The End ...\n")
}

fn error_message(msg: &str) {
    eprintln!("{msg}");
    println!(
        r"
    $ bubble-sort sort asc
    $ bubble-sort sort desc"
    );
    println!();
}

fn print_vector(vec: &Vec<i32>) {
    for item in vec {
        println!(" ->> {item}");
    }
}
