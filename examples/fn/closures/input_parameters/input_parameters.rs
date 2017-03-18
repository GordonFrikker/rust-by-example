// A function which takes a closure as an argument and calls it.
// Функция, которая принимает замыкание в качестве аргумента и вызывает его.
fn apply<F>(f: F) where
    // The closure takes no input and returns nothing.
    // Замыкание ничего не принимает и не возвращает.
    F: FnOnce() {
    // ^ TODO: Try changing this to `Fn` or `FnMut`.
    // ^ TODO: Попробуйте изменить это на `Fn` или `FnMut`.

    f();
}

// A function which takes a closure and returns an `i32`.
// Функция, которая принимает замыкание и возвращает `i32`.
fn apply_to_3<F>(f: F) -> i32 where
    // The closure takes an `i32` and returns an `i32`.
    // Замыкание принимает `i32` и возвращает `i32`.
    F: Fn(i32) -> i32 {

    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    // Некопируемый тип.
    // `to_owned` преобразует заимствованные данные в собственные.
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    // Захват двух переменных: `greeting` по ссылке и
    // `farewell` по значению.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        // `greeting` захватывается по ссылке: требует `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        // Изменяемость требует от `farewell` быть захваченным
        // по изменяемой ссылке. Сейчаст требуется `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        // Ручной вызов удаления требуется от `farewell`
        // быть захваченным по значению. Сейчас требуется `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    // Вызов функции, которая выполняет замыкание.
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    // `double` удовлетворяет ограничениям типажа `apply_to_3`
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}
