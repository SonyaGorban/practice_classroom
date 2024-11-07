fn is_prime(n: &u32) -> bool {
    if *n <= 1 {
        return false;
    }
    if *n <= 3 {
        return true;
    }
    if *n % 2 == 0 || *n % 3 == 0 {
        return false; // відсікання парних чисел та кратних трьом
    }

    let mut i = 5;
    while i * i <= *n {
        if *n % i == 0 || *n % (i + 2) == 0 {
            return false; // перевіряємо кратність
        }
        i += 6; // перевіряємо тільки числа, які не кратні 2 і 3
    }
    true // якщо не знайшли дільників, то число просте
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        let test_data = [
            (0, false),
            (1, false),
            (2, true),
            (3, true),
            (4, false),
            (5, true),
            (100, false),
            (10007, true),
        ];

        test_data
            .iter()
            .for_each(|(n, prime)| assert_eq!(is_prime(n), *prime));
    }
}