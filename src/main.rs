use std::io;
use unicode_segmentation::UnicodeSegmentation; // испольуем крейт для разбиения строки на графемы

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap(); // считываем строку
    let string = input.as_str(); // преобразуем в строковый срез
    let graphemes = UnicodeSegmentation::graphemes(string, true).collect::<Vec<&str>>(); // разбиваем строку на графемы и добавляем в вектор

    let mut result = String::new();
    for &grapheme in graphemes.iter().rev() {
        // записываем полученные графемы в результирующую строку в обратном порядке
        result.push_str(grapheme);
    }
    println!("{}", result);
}
