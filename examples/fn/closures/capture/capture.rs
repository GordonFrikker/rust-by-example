fn main() {
    use std::mem;

    let color = "green";

    // A closure to print `color` which immediately borrows (`&`)
    // `color` and stores the borrow and closure in the `print`
    // variable. It will remain borrowed until `print` goes out of
    // scope. `println!` only requires `by reference` so it doesn't
    // impose anything more restrictive.
    // Замыкание для вывода `color`, которое немедленно заимствует (`&`)
    // `color` и сохраняет заимствование и замыкание в переменной `print`.
    // `color` будет оставаться заимствованным до выхода `print` из области
    // видимости. `println!` требует только ссылку, поэтому он не накладывает
    // никаких ограничений.
    let print = || println!("`color`: {}", color);

    // Call the closure using the borrow.
    // Вызываем замыкание, используя заимствование.
    print();
    print();

    let mut count = 0;

    // A closure to increment `count` could take either `&mut count`
    // or `count` but `&mut count` is less restrictive so it takes
    // that. Immediately borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside.
    // Thus, calling the closure mutates the closure which requires
    // a `mut`.
    // Замыкание для увеличения `count` может принимать как `&mut count`
    // так и `count`, но `&mut count` менее ограничивающее, так что оно
    // выбирает это. Немедленно заимствует `count`.
    //
    // `mut` требуется для `inc`, потому что `&mut` значение хранится внутри.
    // Таким образом, вызов замыкания изменяет его, что требует `mut`
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // Call the closure.
    // Вызываем замыкание.
    inc();
    inc();

    //let reborrow = &mut count;
    // ^ TODO: try uncommenting this line.
    // ^ TODO: попробуйте раскомментировать эту строку.

    // A non-copy type.
    // Тип без возможности копирования.
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    // `mem::drop` требует `T`, так что захват производится по значению.
    // Копируемый тип будет скопирован в замыкание, оставив оригинальное
    // значение без изменения. Некопируемый тип должен быть перемещен и
    // `movable` немедленно перемещается в замыкание.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` consumes the variable so this can only be called once.
    // `consume` поглощает переменную, так что оно может быть вызвано только раз.
    consume();
    //consume();
    // ^ TODO: Try uncommenting this line.
    // ^ TODO: Попробуйте раскомментировать эту строку.
}
