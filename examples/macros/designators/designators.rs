macro_rules! create_function {
    // Этот макрос принимает аргумент указателя `ident` и
    // создает функцию с именем `$func_name`.
    // Указатель `ident` используют для обозначения переменной/имени функции.
    ($func_name:ident) => (
        fn $func_name() {
            // Макрос `stringify!` преобразует `ident` в строку.
            println!("You called {:?}()",
                     stringify!($func_name))
        }
    )
}

// Создадим функции с именами `foo` и `bar` используя макрос, указанный выше.
create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // Этот макрос принимает выражение типа `expr` и напечатает
    // его как строку вместе с результатом.
    // Указатель `expr` используют для обозначения выражений.
    ($expression:expr) => (
        // `stringify!` преобразует выражение *as it is* в строку.
        println!("{:?} = {:?}",
                 stringify!($expression),
                 $expression)
    )
}

fn main() {
    foo();
    bar();

    print_result!(1u32 + 1);

    // Напомним, что блоки тоже являются выражениями!
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });
}
