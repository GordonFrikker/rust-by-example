Компилятор способен предоставить основные реализации для некоторых типажей
с помощью [атрибута][attribute] `#[derive]`. Эти типажи могут быть
реализованы вручную, если необходимо более сложное поведение.

Ниже приводится список выводимых типажей:
* Типажи сравнения:
  [`Eq`][eq], [`PartialEq`][partial-eq], [`Ord`][ord], [`PartialOrd`][partial-ord]
* [`Clone`][clone], для создания `T` из `&T` с помощью копии.
* [`Copy`][copy], чтобы создать тип семантикой копирования, вместо семантики перемещения.
* [`Hash`][hash], чтобы вычислить хэш из `&T`.
* [`Default`][default], чтобы создать пустой экземпляр типа данных.
* `Zero`, для создания нулевого экземпляра числового типа данных.
* [`Debug`][debug], чтобы отформатировать значение с помощью `{:?}`.

{derive.play}

### Смотрите также:
[`derive`][derive]

[attribute]: ../attribute.html
[eq]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
[partial-eq]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
[ord]: https://doc.rust-lang.org/std/cmp/trait.Ord.html
[partial-ord]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
[clone]: https://doc.rust-lang.org/std/clone/trait.Clone.html
[copy]: https://doc.rust-lang.org/core/marker/trait.Copy.html
[hash]: https://doc.rust-lang.org/std/hash/trait.Hash.html
[default]: https://doc.rust-lang.org/std/default/trait.Default.html
[debug]: https://doc.rust-lang.org/std/fmt/trait.Debug.html
[derive]: https://doc.rust-lang.org/reference/attributes.html#derive
