Closures are inherently flexible and will do what the functionality requires
to make the closure work without annotation. This allows capturing to
flexibly adapt to the use case, sometimes moving and sometimes borrowing.
Closures can capture variables:
Замыкания по своей сути достаточно гибкие и предоставляют возможность работы с
замыканиями без дополнительного объяснения. Это позволяет адаптироваться в
зависимости от конкретной необходимости, иногда перемещая значения, а иногда
заимствуя их.
Замыкания могут захватывать переменные:

* по ссылке: `&T`
* по изменяемой ссылке: `&mut T`
* по значению: `T`

They preferentially capture variables by reference and only go lower when
required.
Они преимущественно захватывают переменные по ссылке, если не указано обратное.

{capture.play}

### Смотрите также:

[`Box`][box] и [`std::mem::drop`][drop]

[box]: ../../std/box.html
[drop]: http://doc.rust-lang.org/std/mem/fn.drop.html
