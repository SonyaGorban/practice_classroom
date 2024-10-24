#[test]
fn test1() {
    const DIM: u8 = 11; // Розмір квадрата 11x11
    let mid = DIM / 2;  //  використовується для перевірок

    // ф повертає дзеркальне значення відносно правої межі
    fn mirror(val: u8) -> u8 {
        DIM - 1 - val
    }

    for y in 0..DIM {   // прох по коорд Y
        for x in 0..DIM {   // Прох по коорд X

            //  чи координати в межах ромба
            let cond1 = x + y < mid;
            let cond2 = mirror(x) + y < mid;
            let cond3 = x + mirror(y) < mid;
            let cond4 = mirror(x) + mirror(y) < mid;

            //  або пробіл, або зірку
            let symbol = if cond1 || cond2 || cond3 || cond4 { ' ' } else { '*' };

            print!("{symbol}");
        }
        println!();
    }
}
