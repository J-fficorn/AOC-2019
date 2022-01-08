use std::time::Instant;

fn main() {
    let time = Instant::now();
    let (mut i,  mut end) = (Num::digits(236491), Num::digits(713787));
    let (mut p1, mut p2)  = (0i16, 0i16);
    loop {
        if uphill(&i) {
            if double(&i) { p1 += 1; }
            if exclusive(&i) { p2 += 1; }
        }
        i.inc();
        if i > end { break; }
    }
    print!("PART 1: {}\nPART 2: {}\n", p1, p2);
    print!("TIME: {} ms", time.elapsed().as_micros() as f32 / 1000 as f32);
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Num([u8; 6]);
    
fn uphill(i: &Num) -> bool { (0..5).all(|x| i.0[x] <= i.0[x + 1]) }
    
fn double(i: &Num) -> bool { (0..5).any(|x| i.0[x] == i.0[x + 1]) }
    
fn exclusive(i: &Num) -> bool {
    (0..5).any(|x| i.0[x] == i.0[x + 1] && //double
                   (if x > 0 { i.0[x - 1] != i.0[x] } else { true }) && //exclusive in front
                   (if x < 4 { i.0[x + 2] != i.0[x] } else { true }) //exclusive in back
              )
}
    
impl Num {
    fn digits(i: i32) -> Self {
        Self {
            0: [(i / 100000 % 10) as u8,
                (i / 10000  % 10) as u8,
                (i / 1000   % 10) as u8,
                (i / 100    % 10) as u8,
                (i / 10     % 10) as u8,
                (i          % 10) as u8],
        }
    }
    
    fn inc(&mut self) {
        for i in (0..6).into_iter().rev() { //6..0
            self.0[i] += 1; //last digit
            if self.0[i] != 10 { return; } //0-9 only
            self.0[i] = self.0[i - 1]; //set to uphill
        }
    }
}
