#[test]

/*
// FILL in the blanks
use std::mem;

fn main() {
    let story = String::from("Rust By Practice");

    // Prevent automatically dropping of the String's data
    let mut story = mem::ManuallyDrop::new(story);

    let ptr = story.__();
    let len = story.__();
    let capacity = story.__();

    assert_eq!(16, len);

    // We can rebuild a String out of ptr, len, and capacity. This is all
    // unsafe because we are responsible for making sure the components are
    // valid:
    let s = unsafe { String::from_raw_parts(ptr, len, capacity) };

    assert_eq!(*story, s);

    println!("Success!");
}
*/


// FILL in the blanks


fn main() {
    let story = String::from("Rust By Practice");

    // Prevent automatically dropping of the String's data
    let mut story = mem::ManuallyDrop::new(story);

    let ptr = story.as_mut_ptr(); // Получаем указатель на данные строки
    let len = story.len();        // Получаем длину строки
    let capacity = story.capacity(); // Получаем емкость строки

    assert_eq!(16, len);

    // We can rebuild a String out of ptr, len, and capacity. This is all
    // unsafe because we are responsible for making sure the components are
    // valid:
    let s = unsafe { String::from_raw_parts(ptr, len, capacity) };

    assert_eq!(*story, s);

    println!("Success!");
}
use std::mem;

/*
as_mut_ptr(): Этот метод возвращает изменяемый указатель на данные внутри story. Он необходим для передачи в String::from_raw_parts.
len(): Возвращает количество элементов (байтов) в строке, что эквивалентно длине строки.
capacity(): Возвращает количество байтов, которые могут быть выделены для хранения данных строки, не выполняя повторное выделение памяти.
*/