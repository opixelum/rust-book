fn number_th_to_string(number: u8) -> Result<String, String> {
    if number == 0 || number > 12 {
        return Err(format!("No ordinal string for number {}", number))
    }

    let number_th_strings = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",
    ];

    Ok(number_th_strings[number as usize - 1].to_string())
}

fn main() {
    let lyrics = [
        "A partridge in a pear tree",
        "Two turtle doves and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for index in 0..lyrics.len() {
        match number_th_to_string(index as u8 + 1) {
            Ok(ordinal_str) => println!(
                "On the {} day of Christmas, my true love sent to me",
                ordinal_str
            ),
            Err(e) => println!("Error: {}", e),
        }

        for sub_index in (0..=index).rev() {
            println!("{}", lyrics[sub_index])
        }

        println!()
    }
}
