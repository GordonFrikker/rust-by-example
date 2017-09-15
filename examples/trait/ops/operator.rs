use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

// Типаж `std::ops::Add` используется для указания функциональности `+`.
// Здесь мы объявим `Add<Bar>` - типаж сложения, со вторым
// операндом типа `Bar`.
// Следующий блок реализует операцию: Foo + Bar = FooBar
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Вызвали Foo.add(Bar)");

        FooBar
    }
}

// Если мы поменяем местами типы, то получим реализацию некоммутативного сложения.
// Здесь мы объявим `Add<Foo>` - типаж сложения, со вторым
// операндом типа `Foo`.
// Этот блок реализует операцию: Bar + Foo = BarFoo
impl ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Вызвали Bar.add(Foo)");

        BarFoo
    }
}

fn main() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}
