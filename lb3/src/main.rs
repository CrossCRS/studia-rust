fn swp(a: &mut i32, b: &mut i32) {
    let tmp = *a;
    *a = *b;
    *b = tmp;
}

fn sort(a: &mut i32, b: &mut i32, c: &mut i32) {
    if a > c {
        swp(a, c);
    }
    if a > b {
        swp(a, b);
    }
    if b > c {
        swp(b, c);
    }
}

fn rand(seed: &mut u64, max_rand: u64) -> u64 {
    let mut i = 20;
    let mut x = *seed;

    while i > 0 {
        x *= x;
        x %= max_rand;
        i -= 1;
    }
    *seed += 1;
    x
}

fn random_arr(arr: &mut [i32]) {
    let mut seed: u64 = 1456;

    for i in 0..arr.len() {
        let rand_i = rand(&mut seed, arr.len() as u64);
        swp(arr[i], arr[rand_i as usize]);
    }
}

fn main() { 
    /*let mut a = 1;
    let mut b = 4;
    println!("a = {}, b = {}", a, b);
    swp(&mut a, &mut b);
    println!("a = {}, b = {}", a, b);

    let mut a = 2;
    let mut b = 1;
    let mut c = 0;

    sort(&mut a, &mut b, &mut c);
    println!("{} {} {}", a, b, c);*/
    
    /*let mut seed: u64 = 4343;
    for i in 1..10 {
        println!("{}", rand(&mut seed, 10));
    }*/

    let mut arr = [1, 2, 3, 4, 5];
    random_arr(&mut arr);

    for i in arr {
        print!("{} ", i);
    }
    println!("");
}
