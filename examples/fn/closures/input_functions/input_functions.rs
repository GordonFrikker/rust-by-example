// Объявляем функцию, которая принимает обобщенный тип `F`,
// ограниченный типажом `Fn`, и вызывает его.
fn call_me<F: Fn()>(f: F) {
    f();
}

// Объявляем функцию-обертку, удовлетворяющую ограничению `Fn`
fn function() {
    println!("I'm a function!");
}

fn main() {
    // Определяем замыкание, удовлетворяющее ограничению `Fn`
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}
