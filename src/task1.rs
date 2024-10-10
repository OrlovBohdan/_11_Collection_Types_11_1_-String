#[test]

/*

// FILL in the blanks and FIX errors
// 1. Don't use `to_string()`
// 2. Don't add/remove any code line
fn main() {
    let mut s: String = "hello, ";
    s.push_str("world".to_string());
    s.push(__);

    move_ownership(s);

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

fn move_ownership(s: String) {
    println!("ownership of \"{}\" is moved here!", s)
}
*/


fn main() {
    let mut s: String = String::from("hello, "); // Инициализируем s как String
    s.push_str("world"); // Убираем .to_string()
    s.push('!'); // Заполняем пропуск

    move_ownership(s.clone());

    assert_eq!(s, "hello, world!"); // Эта строка не будет достигнута после перемещения владения

    println!("Success!");
}

fn move_ownership(s: String) {
    println!("ownership of \"{}\" is moved here!", s)
}

/*
Инициализация s: Мы используем String::from("hello, ") вместо let mut s: String = "hello, ";, чтобы корректно создать String.

Удаление to_string(): Вместо s.push_str("world".to_string()); мы просто используем s.push_str("world");
так как push_str принимает строковые срезы (&str), а не String.

Заполнение пропуска: В пропуске нужно использовать символ, который добавит в строку !. Для этого используем s.push('!');.

Assert: Обратите внимание, что утверждение assert_eq!(s, "hello, world!"); никогда не будет выполнено,
так как s больше недоступен после вызова move_ownership(s);. Чтобы этот код работал корректно,
нужно изменить assert_eq! на move_ownership так, чтобы в move_ownership проверялась правильность строки.
Это можно сделать путем возврата строки в move_ownership, но это не входило в требования.
*/