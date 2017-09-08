struct Droppable {
    name: &'static str,
}

// Это простая реализация `drop`, которая добавляет вывод в консоль.
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a" };

    // блок A
    {
        let _b = Droppable { name: "b" };

        // блок B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting block B");
        }
        println!("Just exited block B");

        println!("Exiting block A");
    }
    println!("Just exited block A");

    // Переменную можно сбросить вручную с помощью функции `drop`.
    drop(_a);
    // ЗАДАНИЕ ^ Попробуйте закомментировать эту строку

    println!("end of the main function");

    // `_a` *не можем* сбросить здесь снова, потому что переменная уже
    // (вручную) сброшена.
}
