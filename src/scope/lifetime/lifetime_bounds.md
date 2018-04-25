# Ограничения

Так же как и обобщённые типы, время жизни (обобщённое само по себе) могут быть ограничены.
Знак `:` имеет немного другое значение,
но  знак `+` такое же. Прочитайте следующую заметку:

1. `T: 'a`: *Все* ссылки в `T` должны пережить время жизни `'a`.
2. `T: Trait + 'a`: Тип `T` должен реализовать типаж `Trait` и *все* ссылки
в `T` должны пережить `'a`.

Пример ниже демонстрирует синтаксис в действии:

```rust,editable
use std::fmt::Debug; // Типаж с ограничениями.

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// `Ref` содержит ссылки на обобщённый тип `T` который имеет
// неизвестное время жизни `'a`. `T` ограничен так, что любые
// *ссылки* в `T` должны пережить `'a`.
// Кроме того, время жизни `Ref` не может превышать `'a`.

// Обобщённая функция, которая показывает использование типажа `Debug`.
fn print<T>(t: T) where
    T: Debug {
    println!("`print`: t это {:?}", t);
}

// Здесь приводится ссылка на `T`, где `T` реализует
// `Debug` и все *ссылки* в `T` переживут `'a`.
// К тому же, `'a` должен пережить функцию.
fn print_ref<'a, T>(t: &'a T) where
    T: Debug + 'a {
    println!("`print_ref`: t это {:?}", t);
}

fn main() {
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}
```

### Смотрите также:

[generics][generics], [bounds in generics][bounds], and 
[multiple bounds in generics][multibounds]

[generics]: generics.html
[bounds]: generics/bounds.html
[multibounds]: generics/multi_bounds.html