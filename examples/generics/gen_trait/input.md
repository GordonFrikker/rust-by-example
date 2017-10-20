Конечно `traits` тоже могут быть обобщёнными. Здесь мы определяем, тот
который повторно реализует `trait` `Drop` как обобщённый метод, чтобы
удалить себя и входные данные.

{trait.play}

### Смотрите также:

[`Drop`][Drop], [`Структуры`][structs], и [`trait`][traits]

[Drop]: https://doc.rust-lang.org/std/ops/trait.Drop.html
[structs]: ../custom_types/structs.html
[traits]: ../trait.html
