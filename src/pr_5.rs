#[test]
fn draw_shape() {
    const WIDTH: u32 = 25;
    const HEIGHT: u32 = 10;
    let slope_ratio = WIDTH as f32 / HEIGHT as f32;  // обчисл співвідношення ширини до висоти

    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            let top_bottom_border = row == 0 || row == HEIGHT - 1;  // перевірка на верхню або нижню межу
            let side_border = col == 0 || col == WIDTH - 1;         // Перевірка на ліву або праву межу
            let primary_diagonal = col == (row as f32 * slope_ratio) as u32;  // Перевірка на головну діагональ
            let secondary_diagonal = col == WIDTH - 1 - (row as f32 * slope_ratio) as u32;  // Перевірка на побічну діагональ

            //  точка на межі або на одній із діагоналей- виводимо зірочку, інакше — пробіл
            let symbol = if top_bottom_border || side_border || primary_diagonal || secondary_diagonal {
                '*'
            } else {
                ' '
            };
            print!("{}", symbol);
        }
        println!();
    }
}

