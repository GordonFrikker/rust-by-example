`Iterator::find` is a function which when passed an iterator, will return
the first element which satisfies the predicate as an `Option`. Its
signature:
`Iterator::find` - это функция, которая принимает итератор и возращает первый
элемент, который удовлетворяет предикату, как `Option`. Ее объявление:

```rust
pub trait Iterator {
    // The type being iterated over.
    // Тип, по которому выполняется итерирование
    type Item;

    // `find` takes `&mut self` meaning the caller may be borrowed
    // and modified, but not consumed.
    // `find` принимает `&mut self`, что означает заимствование и
    // изменение, но не поглощение
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
        // `FnMut` meaning any captured variable may at most be
        // modified, not consumed. `&Self::Item` states it takes
        // arguments to the closure by reference.
        // `FnMut` означает, что любая захваченная переменная
        // может быть изменена, но не поглощена. `Self::Item`
        // указывает на захват аргументов замыкания по значению
        P: FnMut(&Self::Item) -> bool {}
}
```

{iter_find.play}

### Смотрите также:

[`std::iter::Iterator::find`][find]

[find]: http://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find
