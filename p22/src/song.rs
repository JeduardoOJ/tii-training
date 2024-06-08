pub fn the_twelve_days_of_christmas() {
    let song = [
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
    ];
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    println!("\n\nThe Twelve Days of Christmas Song\n");

    for (i, day) in days.iter().enumerate() {
        println!("On the {} day of Christmas, ", day);
        println!("my true love gave to me,");
        for song_part in &song[11 - i..] {
            println!("{}", *song_part);
        }

        if i == 0 {
            println!("A partridge in a pear tree\n");
        } else {
            println!("And a partridge in a pear tree.\n");
        }
    }
}
