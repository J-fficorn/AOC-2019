fn main() {
    let input = include_str!("i.txt");
    let masses : Vec<i64> = input.lines().map(|x| x.trim().parse().unwrap()).collect();
    let mut p1 : i64 = 0;
    let mut p2 : i64 = 0;
    for mass in masses {
        //print!("{} ", mass);
        p1 += mass / 3 - 2;
        let mut s : i64 = mass / 3 - 2;
        //print!("S: {} ", s);
        let mut r : i64 = mass / 3 - 2;
        while r > 0 {
            //print!("{} {} ", r, s);
            r = if r / 3 - 2 > 0 { r / 3 - 2 } else { 0 };
            s = s + r;
        }
        //print!("S: {} ", s);
        p2 += s;
    }
    print!("PART 1: {}\nPART 2: {}", p1, p2);
}
