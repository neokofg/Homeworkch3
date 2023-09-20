fn main() {

    let mut input = String::new();

    println!("Choose your function: fahrenheit, fibbonachi, twelvedays");

    std::io::stdin().read_line(&mut input).expect("Failed to read a line!");

    if input.trim() == "fahrenheit" {
        fahrenheit()
    } else if input.trim() == "fibbonachi" {
        fibbonachi()
    } else if input.trim() == "twelvedays" {
        twelvedays()
    } else {
        panic!("Invalid function");
    }
}

fn fahrenheit() {

    let mut fahrenheit = String::new();

    println!("Input temperature by fahrenheit!");

    std::io::stdin().read_line(&mut fahrenheit).expect("Failed to read a line!");

    let fahrenheit: f32 = match fahrenheit.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Input is not a number!"),
    };

    let result = (fahrenheit - 32.0) * 5.0 / 9.0;

    println!("Temperature in Celsius is {}", result);

}

fn fibbonachi() {

    let mut fibbonachi = String::new();

    println!("Input fibbonachi number!");

    std::io::stdin().read_line(&mut fibbonachi).expect("Failed to read a line!");

    let fibbonachi: i128 = match fibbonachi.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Input is not a solid positive number!"),
    };

    let mut result: i128 = 2;

    let mut a:i128 = 1;

    let mut b:i128 = 1;

    while result != fibbonachi {
        let c:i128 = a + b;
        a = b;
        b = c;
        result += 1;
    }

    println!("your fibbonachi is {}", b)
}

fn twelvedays() {
    const DAYS: [i32;12] = [1,2,3,4,5,6,7,8,9,10,11,12];
    const TEXT: [&str;12] = [
        "A partridge in a pear tree",
        "2 turtle doves",
        "3 french hens",
        "4 calling birds",
        "5 golden rings",
        "6 geese a-laying",
        "7 swans a swimming",
        "8 maids a milking",
        "9 ladies dancing",
        "10 lords a leaping",
        "11 pipers piping",
        "12 drummers drumming",
    ];
    for day in DAYS {
        let mut count = 0;
        println!("On the {} day of christmas", day);
        println!("My true lover sent to me:");
        while count != day {
            println!("{}", TEXT[count as usize]);
            count += 1;
        }
    }
}