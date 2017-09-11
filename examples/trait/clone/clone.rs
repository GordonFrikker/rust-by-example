// Единичная структура без ресурсов
#[derive(Debug, Clone, Copy)]
struct Nil;

// Кортежная структура с ресурсами, которая реализует типаж `Clone`
#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    // Объявим экземпляр `Nil`
    let nil = Nil;
    // Скопируем `Nil`, который не имеет ресурсов для перемещения
    let copied_nil = nil;

    // Оба `Nil`s могут быть использованы независимо
    println!("original: {:?}", nil);
    println!("copy: {:?}", copied_nil);

    // Объявим экземпляр `Pair`
    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    // Скопируем `pair` в `moved_pair`, перенаправляя ресурсы
    let moved_pair = pair;
    println!("copy: {:?}", moved_pair);

    // Ошибка! `pair` потеряла свои ресурсы
    //println!("original: {:?}", pair);
    // ЗАДАНИЕ ^ Попробуйте раскомментировать эту строку

    // Скопируем `moved_pair` в `cloned_pair` (включая ресурсы)
    let cloned_pair = moved_pair.clone();
    // Сбросим оригинальную пару используя std::mem::drop
    drop(moved_pair);

    // Ошибка! `moved_pair` была сброшена
    //println!("copy: {:?}", moved_pair);
    // ЗАДАНИЕ ^ Попробуйте раскомментировать эту строку

    // Полученный результат из .clone() все ещё можно использовать!
    println!("clone: {:?}", cloned_pair);
}
