fn main() {
    let year1993 = 1993;
    print_message(year1993, year1993 - 10);

    let year2021 = 2021;
    print_message(year2021, year2021 - 10);
}

fn print_message(first_year: i32, second_year: i32 ) {
    println!("{}: ten years ago was {}", first_year, second_year);
}