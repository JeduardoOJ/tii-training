pub fn the_twelve_days_of_christmas() {
    let song: [&str; 12] = [
        "Twelve drummers drumming,",
        "Eleven pipers piping,",
        "Ten lords a-leaping,",
        "Nine ladies dancing,",
        "Eight maids a-milking,",
        "Seven swans a-swimming,",
        "Six geese a-laying,",
        "Five golden rings,",
        "Four calling birds,",
        "Three French hens,",
        "Two turtle doves,",
        "",
    ];
    let days: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for i in 0..12 {
        println!("On the {} day of Christmas, ", days[i]);
        println!("my true love gave to me,");

        for j in 11 - i..11 {
            println!("{}", song[j]);
        }

        if i == 0 {
            println!("A partridge in a pear tree\n");
        } else {
            println!("And a partridge in a pear tree.\n");
        }
    }
}
