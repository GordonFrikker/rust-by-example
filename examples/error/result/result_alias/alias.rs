use std::num::ParseIntError;

// Объявим обобщённый псевдоним для `Result` с типом ошибки `ParseIntError`.
type AliasedResult<T> = Result<T, ParseIntError>;

// Используем вышеуказанный псевдоним для обозначения
// нашего конкретного типа `Result`.
fn double_number(number_str: &str) -> AliasedResult<i32> {
    number_str.parse::<i32>().map(|n| 2 * n)
}

// Здесь псевдоним снова позволяет нам сэкономить место.
fn print(result: AliasedResult<i32>) {
    match result {
        Ok(n) => println!("n это {}", n),
        Err(e) => println!("Ошибка: {}", e),
    }
}

fn main() {
    print(double_number("10"));
    print(double_number("t"));
}
