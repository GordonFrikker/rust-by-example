// `Centimeters`, кортежная структура, которую можно сравнить
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches`, кортежная структура, которую можно напечатать
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

// `Seconds`, кортежная структура без дополнительных атрибутов
struct Seconds(i32);

fn main() {
    let _one_second = Seconds(1);

    // Ошибка: `Seconds` не может быть напечатана; не реализован типаж `Debug`
    //println!("One second looks like: {:?}", _one_second);
    // ЗАДАНИЕ ^ Попробуйте раскомментировать эту строку

    // Ошибка: `Seconds` нельзя сравнить; не реализован типаж `PartialEq`
    //let _this_is_true = (_one_second == _one_second);
    // ЗАДАНИЕ ^ Попробуйте раскомментировать эту строку

    let foot = Inches(12);

    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

    println!("One foot is {} than one meter.", cmp);
}
