// Функция, которая принимает замыкание в качестве аргумента и вызывает его.
fn apply<F>(f: F) where
    // Замыкание ничего не принимает и не возвращает.
    F: FnOnce() {
    // ^ TODO: Попробуйте изменить это на `Fn` или `FnMut`.

    f();
}

// Функция, которая принимает замыкание и возвращает `i32`.
fn apply_to_3<F>(f: F) -> i32 where
    // Замыкание принимает `i32` и возвращает `i32`.
    F: Fn(i32) -> i32 {

    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // Не копируемый тип.
    // `to_owned` преобразует заимствованные данные в собственные.
    let mut farewell = "goodbye".to_owned();

    // Захват двух переменных: `greeting` по ссылке и
    // `farewell` по значению.
    let diary = || {
        // `greeting` захватывается по ссылке: требует `Fn`.
        println!("I said {}.", greeting);

        // Изменяемость требует от `farewell` быть захваченным
        // по изменяемой ссылке. Сейчас требуется `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Ручной вызов удаления требуется от `farewell`
        // быть захваченным по значению. Теперь требуется `FnOnce`.
        mem::drop(farewell);
    };

    // Вызов функции, которая выполняет замыкание.
    apply(diary);

    // `double` удовлетворяет ограничениям типажа `apply_to_3`
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}
