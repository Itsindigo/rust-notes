fn main() {
    twelve_days_of_christmas();
}


fn twelve_days_of_christmas() {
    let gifts = ["And a Partridge in a Pear Tree", "Two Turtle Doves", "Three French Hens", "Four Calling Birds", "Five Golden Rings", "Six Geese a-Laying", "Seven Swans a-Swimming", "Eight Maids a-Milking", "Nine Ladies Dancing", "Ten Lords a-Leaping", "Eleven Pipers Piping", "Twelve Drummers Drumming"];

    let date_labels = [
        "First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh", "Eighth", "Ninth", "Tenth", "Eleventh", "Twelth"
    ];
    let mut nth_day = 0;

    while nth_day < 12 {
        println!("On the {date_label} day of Christmas", date_label=date_labels[nth_day]);
        println!("My true love gave to me");

        for i in (0..nth_day + 1).rev() {
            if nth_day == 0 {
                println!("A Partridge in a pear tree");
            } else {
                println!("{gift}", gift=gifts[i])
            }
        }
        println!("\n");
        nth_day += 1;
    }
}