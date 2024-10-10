#[test]

/*

// FILL in the blanks
fn main() {
    let mut s = String::new();
    __;

    // Some bytes, in a vector
    let v = vec![104, 101, 108, 108, 111];

    // Turn a byte's vector into a String
    let s1 = __;


    assert_eq!(s, s1);

    println!("Success!");
}
*/

fn main() {
    let mut s = String::new();
    s.push_str("hello"); // Заполняем строку значением "hello"

    // Some bytes, in a vector
    let v = vec![104, 101, 108, 108, 111]; // Это ASCII-коды для "hello"

    // Turn a byte's vector into a String
    let s1 = String::from_utf8(v).expect("Invalid UTF-8"); // Преобразуем вектор байтов в строку

    assert_eq!(s, s1); // Проверяем, что строки равны

    println!("Success!");
}

/*
Заполнение строки s: В первом пропуске мы используем метод push_str для заполнения строки значением "hello".
Преобразование вектора байтов в строку: Во втором пропуске используется String::from_utf8(v) для преобразования
вектора байтов в строку. Этот метод возвращает Result, поэтому необходимо обработать возможную ошибку с помощью expect.
*/