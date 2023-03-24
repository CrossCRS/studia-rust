fn is_leap(year: i32) -> bool {
    year % 4 == 0 && year % 100 != 0 || year % 400 == 0
}

fn zad2() {
    let month = 6;
    let year = 2023;

    let days = if month == 1 || month == 3 || month == 5 || month == 7 || month == 8 || month == 10 || month == 12 {
        31
    } else if month == 2 {
        if is_leap(year) {
            29
        } else {
            28
        }
    } else {
        30
    };

    println!("{}.{} has {} days", month, year, days);
}

fn zad3(temperature: f32) -> f32 {
    32.0 + (9.0/5.0) * temperature
}

fn zad4(temperature: f32) -> f32 {
    (temperature - 32.0) / (9.0/5.0)
}

fn zad5(time1: (i32, i32, i32), time2: (i32, i32, i32)) -> (i32, i32, i32) {
    let s1 = time1.0 * 60 * 60 + time1.1 * 60 + time1.2;
    let s2 = time2.0 * 60 * 60 + time2.1 * 60 + time2.2;

    let mut res = (s1 - s2).abs();

    let res_h = res / 3600;
    res -= res_h * 3600;
    let res_m = res / 60;
    res -= res_m * 60;

    (res_h, res_m, res)
}

fn zad6(n: i64) -> i64 {
    let mut s = 1;
    for i in 2..n+1 {
        s *= i;
    }
    s
}

fn zad7(n: i32) {
    let mut i = n;
    while i > 0 {
        print!("{} ", i % 10);
        i /= 10;
    }
    println!();
}

fn zad8(n: i32) -> i32 {
    let mut i = n;
    let mut sum = 0;
    while i > 0 {
        sum += i % 10;
        i /= 10;
    }
    sum
}

fn zad9(x: i32) {
    for a in 1..x {
        for b in a..x {
            for c in 1..x {
                if a*a + b*b == c*c {
                    println!("{}^2 + {}^2 = {}^2", a, b, c);
                }
            }
        }
    }
}

fn zad_dod(a: i32) {
    let mut step = 2.0;
}

fn main() {
    println!("2023 leap = {}", is_leap(2023));
    zad2();
    println!("6*C = {}*F", zad3(6.0));
    println!("42.8*F = {}*C", zad4(42.8));
    let time = zad5((22, 13, 30), (16,24,12));
    println!("22:13:30 i 16:24:12 = {}:{}:{}", time.0, time.1, time.2);
    println!("8! = {}", zad6(8));
    zad7(1234);
    println!("Suma cyfr 1234 = {}", zad8(1234));
    zad9(25);
}
