`Iterator::find` - это функция, которая принимает итератор и возращает первый
элемент, который удовлетворяет предикату, как `Option`. Ее объявление:

```rust
pub trait Iterator {
    // Тип, по которому выполняется итерирование
    type Item;

    // `find` принимает `&mut self`, что означает заимствование и
    // изменение, но не поглощение `self`.
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
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
