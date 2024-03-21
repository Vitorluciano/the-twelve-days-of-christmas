fn main() {
    let ordinals = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let verses = [
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,"
    ]; 

    for i in 0..ordinals.len() {
        let ordinal = ordinals[i];

        println!("On the {ordinal} day of Christmas,");
        println!("my true love gave to me");

        if i == 0 {
            println!("A partridge in a pear tree.\n");
        } else {
            for j in (0..i).rev() {
                let verse = verses[j];
                println!("{verse}");         
            }
            println!("And a partridge in a pear tree.\n");
        }
    }
}
