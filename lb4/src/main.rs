fn liczba_wystapien(napis: &String, znak: char) -> usize {
    napis.matches(znak).count()
}

fn co_drugi_znak(napis: &String) -> String {
    napis.chars().step_by(2).collect()
}

fn szyfruj(napis: &String, klucz: usize) -> String {
    napis.chars().collect::<Vec<char>>().chunks(klucz).map(|c| {
        let mut rev = c.to_vec();
        rev.reverse();
        rev
    }).flatten().collect()
}

fn wizytowka(imie: &String, nazwisko: &String) -> String {
    let mut napis = String::new();
    napis.push(imie.chars().next().unwrap().to_ascii_uppercase());
    napis.push_str(". ");

    let mut nazwisko_chars = nazwisko.chars();
    napis.push(nazwisko_chars.next().unwrap().to_ascii_uppercase());

    for c in nazwisko_chars {
        napis.push(c.to_ascii_lowercase());
    }

    napis
}

fn rzymskie(napis: &String) -> u32 {
    let mut last_digit = 0;
    let mut value = 0;

    for c in napis.chars().rev() {
        let v = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _   => panic!(":o")
        };

        if last_digit > v {
            value -= v;
        } else {
            value += v;
        }

        last_digit = v;
    }

    value
}

// POPSUTE
fn na_rzymskie(liczba: i32) -> String {
    let mut result = String::new();
    let mut l = liczba;

    while l > 0 {
        if l - 1000 > 0 {
            result.push('M');
            l -= 1000;
        } else if l - 500 > 0 {
            result.push('D');
            l -= 500;
        } else if l - 100 > 0 {
            result.push('C');
            l -= 100;
        } else if l - 50 > 0 {
            result.push('L');
            l -= 50;
        } else if l - 10 > 0 {
            result.push('X');
            l -= 10;
        } else {
            result.push('I');
            l -= 1;
        }
    }

    result
}

// TEŻ POPSUTE
fn dodaj_pisemnie(a: &String, b: &String) -> String {
    let mut a_ = if a.len() > b.len() { b.chars().rev() } else { a.chars().rev() };
    let mut b_ = if a.len() > b.len() { a.chars().rev() } else { b.chars().rev() };
    let mut result = String::new();

    let mut remainder = 0;
    for c in b_ {
        let x = c.to_digit(10).unwrap();
        let y = a_.next().unwrap().to_digit(10).unwrap();
    }

    result.chars().rev().collect()
}

fn main() {
    let napis: String = "ala ma kota".to_string();

    println!("ala ma kota (a) = {}", liczba_wystapien(&napis, 'a'));
    println!("{}", co_drugi_znak(&napis));
    println!("{}", szyfruj(&napis, 2));

    println!("{}", wizytowka(&"jan".to_string(), &"KOWALSKI".to_string()));
    println!("III = {}", rzymskie(&"III".to_string()));
    println!("IX = {}", rzymskie(&"IX".to_string()));
    println!("XIX = {}", rzymskie(&"XIX".to_string()));
    println!("MCMX = {}", rzymskie(&"MCMX".to_string()));

    println!("1910 = {}", na_rzymskie(1910));
}
