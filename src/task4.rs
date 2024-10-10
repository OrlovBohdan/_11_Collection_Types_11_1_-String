#[test]

/*
// FILL in the blank and FIX errors
fn main() {
    let s = String::from("hello, 世界");
    let slice1 = s[0]; //tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(slice1, "h");

    let slice2 = &s[3..5]; // Tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(slice2, "世");

    // Iterate through all chars in s
    for (i, c) in s.__ {
        if i == 7 {
            assert_eq!(c, '世')
        }
    }

    println!("Success!");
}
*/


fn main() {
    let s = String::from("hello, 世界");

    let slice1 = &s[0..1]; // Получаем "h"
    assert_eq!(slice1, "h");

    let slice2 = &s[7..10]; // Получаем "世"
    assert_eq!(slice2, "世");

    // Итерация по всем символам в строке
    for (i, c) in s.chars().enumerate() {
        if i == 7 {
            assert_eq!(c, '世'); // Проверяем, что символ на индексе 7 - "世"
        }
    }

    println!("Success!");
}



/*
let slice1 = &s[0..1];: Здесь мы создаем срез, который включает только первый байт, соответствующий символу 'h'.
Для правильной работы с срезами строк в Rust необходимо использовать диапазоны.

let slice2 = &s[7..10];: В этом срезе мы указываем диапазон, который включает байты, соответствующие символу '世'.
Мы знаем, что символ "世" занимает 3 байта, так что правильный диапазон — от 7 до 10.

for (i, c) in s.chars().enumerate(): Этот код итерирует по всем символам в строке, возвращая индекс и символ. chars()
возвращает итератор по символам, а enumerate() предоставляет индексы.
*/