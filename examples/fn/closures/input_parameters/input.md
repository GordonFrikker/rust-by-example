While Rust chooses how to capture variables on the fly mostly without type
annotation, this ambiguity is not allowed when writing functions. When
taking a closure as an input parameter, the closure's complete type must be
annotated using one of a few `traits`. In order of decreasing restriction,
they are:
В то время как замыкания Rust выбирают, как захватывать переменные на лету, по
большей части без указания типов, эта двусмысленность не допустима при написании
функций. При использовании замыкания в качестве аргумента, его тип должен быть
указан с использованием одного из типажей. Вот они, в порядке уменьшения
ограниченности:

* `Fn`: замыкание захватывает по ссылке (`&T`)
* `FnMut`: замыкание захватывает по изменяемой ссылке (`&mut T`)
* `FnOnce`: замыкание захватывает по значению (`T`)

On a variable-by-variable basis, the compiler will capture variables in the
least restrictive manner possible.


For instance, consider a parameter annotated as `FnOnce`. This specifies
that the closure *may* capture by `&T`, `&mut T`, or `T`, but the compiler
will ultimately choose based on how the captured variables are used in the
closure.
Для примера, рассмотрим аргумент, отмеченныей как `FnOnce`. Это определяет, что
замыкание *может* захватывать `&T`, `&mut T`, или `T`, но компилятор в итоге
будет выбирать в зависимости от того, как захваченные переменные используются
в замыкании.

This is because if a move is possible, then any type of borrow should also
be possible. Note that the reverse is not true. If the parameter is
annotated as `Fn`, then capturing variables by `&mut T` or `T` are not
allowed.
Это связано с тем, что если перемещение возможно, тогда любой тип заимствования
также должен быть возможен. Отметим, что обратное не верно. Если параметр
указан как `Fn`, то захват переменных как `&mut T` или `T` недопустим.

In the following example, try swapping the usage of `Fn`, `FnMut`, and
`FnOnce` to see what happens:
В следующем примере попробуйте поменять местами использование `Fn`, `FnMut`, и
`FnOnce`, чтобы увидеть результат.

{input_parameters.play}

### Смотрите также:

[`std::mem::drop`][drop], [`Fn`][fn], [`FnMut`][fnmut], and [`FnOnce`][fnonce]

[drop]: http://doc.rust-lang.org/std/mem/fn.drop.html
[fn]: http://doc.rust-lang.org/std/ops/trait.Fn.html
[fnmut]: http://doc.rust-lang.org/std/ops/trait.FnMut.html
[fnonce]: http://doc.rust-lang.org/std/ops/trait.FnOnce.html
