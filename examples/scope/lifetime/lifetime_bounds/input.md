Так же как и обобщённые типы, время жизни (обобщённое само по себе) могут быть ограничены.
Знак `:` имеет немного другое значение,
но  знак `+` такое же. Прочитайте следующую заметку:

1. `T: 'a`: *Все* ссылки в `T` должны пережить время жизни `'a`.
2. `T: Trait + 'a`: Тип `T` должен реализовать типаж `Trait` и *все* ссылки
в `T` должны пережить `'a`.

Пример ниже демонстрирует синтаксис в действии:

{bounds.play}

### Смотрите также:

[`Обобщения`][generics], [bounds in generics][bounds], and
[multiple bounds in generics][multibounds]

[generics]: ../../generics.html
[bounds]: ../../generics/bounds.html
[multibounds]: ../../generics/multi_bounds.html
