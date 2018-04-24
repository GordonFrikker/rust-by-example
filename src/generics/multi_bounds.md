# Множественные ограничения

Множественные ограничения по типажу могут быть применены с помощью `+`.
Разные типы разделяются с помощью `,`.

```rust,editable
use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: `{:?}", t);
    println!("u: `{:?}", u);
}

fn main() {
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    //compare_prints(&array);
    // ЗАДАНИЕ ^ Попробуйте удалить комментарий.

    compare_types(&array, &vec);
}
```

### Смотрите также:

[`std::fmt`][fmt] и [`типажи`][traits]

[fmt]: hello/print.html
[traits]: trait.html
