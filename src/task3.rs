#[test]

/*

// Question: how many heap allocations are happening here?
// Your answer:
fn main() {
    // Create a String type based on `&str`
    // The type of string literals is `&str`
   let s: String = String::from("hello, world!");

   // Create a slice point to String `s`
   let slice: &str = &s;

   // Create a String type based on the recently created slice
   let s: String = slice.to_string();

   assert_eq!(s, "hello, world!");

   println!("Success!");
}
*/


// Question: how many heap allocations are happening here?
// Your answer: в этом коде происходит 2 выделения памяти в куче.
fn main() {
    // Create a String type based on `&str`
    // The type of string literals is `&str`
    let s: String = String::from("hello, world!");


    // Create a slice point to String `s`
    let slice: &str = &s;

    // Create a String type based on the recently created slice
    let s: String = slice.to_string();

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

/*
В данном коде происходит две выделения памяти в куче (heap allocations):
Первое выделение:
let s: String = String::from("hello, world!");
Здесь создается новый объект типа String на основе строкового литерала "hello, world!".
Эта строка копируется в новую область памяти в куче, что требует выделения.
Второе выделение:
let s: String = slice.to_string();
В этом случае метод to_string() вызывается на срезе slice, который ссылается на ранее созданный String.
Метод to_string() создает новый объект String, который также требует выделения памяти в куче для хранения копии строки.
*/