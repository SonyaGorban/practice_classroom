fn invert_the_case(s: String) -> String {
    s.chars() //ітерація по кожному символу рядка
        .map(|c| {
            if c.is_uppercase() {        //символ у верхньому регістрі- змінюємо його на нижній
                c.to_lowercase().collect::<String>()
            } else {
                c.to_uppercase().collect::<String>()
            }
        })
        .collect() // збир змінені символи в новий рядок
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invert_the_case() {
        let data = [
            ("Hello", "hELLO"),
            ("Привет", "пРИВЕТ"),
        ];

        data.iter()
            .for_each(|(a, b)| {
            assert_eq!(invert_the_case(a.to_string()),  //перевір, що результат для a відповід очікуваному b
                       b.to_string());
            assert_eq!(invert_the_case(b.to_string()),
                       a.to_string());
        });
    }
}
