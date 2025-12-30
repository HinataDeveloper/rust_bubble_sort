fn main() {
    println!("\n");

    let mut number_list = vec![101, 220, 301, 14, 50, 16, 72, 89, 19];
    let len = number_list.len() - 1;

    'outer: loop {
        let mut swapped = false;

        for index in 0..len {
            if number_list[index] < number_list[index + 1] {
                number_list.swap(index, index + 1);
                swapped = true;
            }
        }

        if !swapped {
            break 'outer;
        }
    }

    for item in number_list {
        println!(" -> {}", item);
    }

    println!("\n The End ...\n");
}
