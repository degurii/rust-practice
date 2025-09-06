use std::borrow::Cow;

const LINES: [&str; 12] = [
    "A partridge in a pear tree",
    "Two turtle doves and",
    "Three French hens",
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

const ORDINALS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

fn carol_lines_up_to(month: u8) -> Option<&'static [&'static str]> {
    match month {
        1..=12 => Some(&LINES[..month.into()]),
        _ => None,
    }
}

fn verse_for_month(month: u8) -> Option<impl Iterator<Item = Cow<'static, str>>> {
    if !(1..=12).contains(&month) {
        return None;
    }

    let lines = carol_lines_up_to(month)?.iter().rev();

    let opening = format!(
        "On the {} day of Christmas, my true love sent to me",
        ORDINALS[month as usize - 1],
    );

    Some(std::iter::once(Cow::Owned(opening)).chain(lines.map(|&s| Cow::Borrowed(s))))
}

fn main() {
    (1..=12).filter_map(verse_for_month).for_each(|lines| {
        lines.for_each(|s| println!("{s}"));
        println!();
    });
}
