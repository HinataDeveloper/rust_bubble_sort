fn main() {
    println!("\n");

    let mut number_list = vec![
        10, 56, 34, 75, 82, 21, 45, 62, 86, 95, 74, 91, 203, 58, 401, 301,
    ];

    let sort_style = Sorting::Ascending;
    // let sort_style = Sorting::Descending;

    let should_swap = match sort_style {
        Sorting::Ascending => |a: i32, b: i32| a > b,
        Sorting::Descending => |a: i32, b: i32| a < b,
    };

    let len = number_list.len() - 1;

    'outer: loop {
        let mut swapped = false;

        for index in 0..len {
            if should_swap(number_list[index], number_list[index + 1]) {
                number_list.swap(index, index + 1);
                swapped = true;
            }
        }

        if !swapped {
            break 'outer;
        }
    }

    for item in &number_list {
        println!(" -> {}", item);
    }

    println!("\n The End ...\n")
}

enum Sorting {
    Ascending,
    Descending,
}
