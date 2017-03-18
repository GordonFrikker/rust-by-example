`Iterator::any` is a function which when passed an iterator, will return
`true` if any element satisfies the predicate. Otherwise `false`. Its
signature:
`Iterator::any` - это функция, которая принимает итератор и возвращает `true`,
если любой элемент удовлетворяет предикату. Иначе возвращает `false`. Ее
объявление:

```rust
pub trait Iterator {
    // The type being iterated over.
    // Тип, по которому выполняется итерирование
    type Item;

    // `any` takes `&mut self` meaning the caller may be borrowed
    // and modified, but not consumed.
    // `any` принимает `&mut self`, что означает заимствование
    // и изменение, но не поглощение
    fn any<F>(&mut self, f: F) -> bool where
        // `FnMut` meaning any captured variable may at most be
        // modified, not consumed. `Self::Item` states it takes
        // arguments to the closure by value.
        // `FnMut` означает, что любая захваченная переменная
        // может быть изменена, но не поглощена. `Self::Item`
        // указывает на захват аргументов замыкания по значению
        F: FnMut(Self::Item) -> bool {}
}
```

{iter_any.play}

### Смотрите также:

[`std::iter::Iterator::any`][any]

[any]: http://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any
