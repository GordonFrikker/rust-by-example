Макросы позволяют писать DRY код , путем разделения общих частей функций
и/или набор тестов. Вот пример, который реализует и тестирует операторы
`+=`, `*=` и `-=` на `Vec<T>`:

{dry.rs}

```
$ rustc --test dry.rs && ./dry
running 3 tests
test test::mul_assign ... ok
test test::add_assign ... ok
test test::sub_assign ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured
```
