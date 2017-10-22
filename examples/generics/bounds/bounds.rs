// Типаж, который реализует маркер печати: `{:?}`.
use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 { self.length * self.height }
}

#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }
#[allow(dead_code)]
struct Triangle  { length: f64, height: f64 }

// Обобщенный тип `T` должен реализовать `Debug`. Независимо
// от типа, это будет работать правильно.
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// `T` должен реализовать `HasArea`. Любая функция, которая встречается
// в ограничении может получить доступ к функции `area` из `HasArea`.
fn area<T: HasArea>(t: &T) -> f64 { t.area() }

fn main() {
    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    let _triangle = Triangle  { length: 3.0, height: 4.0 };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    //print_debug(&_triangle);
    //println!("Area: {}", area(&_triangle));
    // ^ TODO: Попробуйте раскомментировать эти строки.
    // | Ошибка: Не реализует `Debug` или `HasArea`.
}
