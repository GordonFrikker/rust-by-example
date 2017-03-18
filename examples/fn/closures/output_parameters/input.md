Closures as input parameters are possible, so returning closures as
output parameters should also be possible. However, returning closure types
are problematic because Rust currently only supports returning concrete
(non-generic) types. Anonymous closure types are, by definition, unknown
and so returning a closure is only possible by making it concrete. This
can be done via boxing.
Замыкания могут использоваться как входные параметры, значит и возврат замыканий
как выходных параметров также должен быть возможен. Однако, возврат замыканий
проблематичен из-за того, что Rust в настоящее время поддерживает только возврат
конкретных (не обобщенных) типов. Типы анонимных замыканий, по определению,
неизвестны. И поэтому возвращение замыканий возможно только путем конкретизации
их типов. Это может быть сделано через использование кучи.

The valid traits for returns are slightly different than before:
Возможные типажи для возвращаемых значений немного отличаются от прежних:

* `Fn`: как раньше
* `FnMut`: как раньше
* `FnOnce`: There are some unusual things at play here, so the [`FnBox`][fnbox]
  type is currently needed, and is unstable. This is expected to change in
  the future.
* `FnOnce`: здесь присутствуют некоторые неожиданности, поэтому тип [`FnBox`][fnbox]
  необходим, но он нестабилен в настоящее время. В будущем ожидаются изменения
  этого.

Beyond this, the `move` keyword must be used, which signals that all captures
occur by value. This is required because any captures by reference would be
dropped as soon as the function exited, leaving invalid references in the
closure.
Помимо этого, ключевое слово `move` должно быть использовано, чтобы
сигнализировать о том, что все переменные захватываются по значению. Это
необходимо, так как любые захваченные по ссылке значения будут удалены после
выхода из функции, оставляя недопустимые ссылки в замыкании.

{output_parameters.play}

### Смотрите также:

[Boxing][box], [`Fn`][fn], [`FnMut`][fnmut], and [Generics][generics].

[box]: ../../std/box.html
[fn]: http://doc.rust-lang.org/std/ops/trait.Fn.html
[fnmut]: http://doc.rust-lang.org/std/ops/trait.FnMut.html
[fnbox]: http://doc.rust-lang.org/std/boxed/trait.FnBox.html
[generics]: ../../generics.html
