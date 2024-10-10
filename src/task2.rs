#[test]

/*
// FILL in the blanks
fn main() {
   let mut s = String::from("hello, world");

   let slice1: &str = __; // In two ways
   assert_eq!(slice1, "hello, world");

   let slice2 = __;
   assert_eq!(slice2, "hello");

   let slice3: __ = __;
   slice3.push('!');
   assert_eq!(slice3, "hello, world!");

   println!("Success!");
}
*/

fn main() {
    let mut s = String::from("hello, world");

    let slice1: &str = &s; // Первый способ: создаем срез всей строки
    assert_eq!(slice1, "hello, world");

    let slice2 = &s[0..5]; // Второй способ: создаем срез с "hello"
    assert_eq!(slice2, "hello");

    let slice3: &mut String = &mut s; // Заполняем пропуск для изменяемого среза
    slice3.push('!'); // Добавляем '!' к строке
    assert_eq!(slice3, "hello, world!");

    println!("Success!");
}
/*
let slice1: &str = &s;: Это создание неизменяемого среза, который ссылается на всю строку s.

let slice2 = &s[0..5];: Здесь создается срез, который включает символы от индекса 0 до 4, что соответствует строке "hello".

let slice3: &mut String = &mut s;: Мы создаем изменяемый срез, который ссылается на строку s. Обратите внимание, что это не совсем "срез" в традиционном смысле, а просто ссылка на s, которую можно модифицировать.

slice3.push('!');: Здесь мы добавляем символ '!' к строке, на которую ссылается slice3.
*/

