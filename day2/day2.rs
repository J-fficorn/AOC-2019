fn main() {
    let input = include_str!("i.txt");
    let mut intcode : Vec<i64> = input.split(",").map(|x| x.parse().unwrap()).collect();
    let frozen : Vec<i64> = intcode.clone();
    let p1 = runIntCode(intcode);
    print!("PART 1: {}", p1);
    print!("\n"); //p1
    let p2 = findNV(19690720, frozen);
    print!("\nPART 2: {}", p2);
}

fn operate(op: i64, a: i64, b: i64) -> i64 {
    match op {
        1  => return a + b,
        2  => return a * b,
        _  => return op
    }
}

fn runIntCode(ic: Vec<i64>) -> i64 {
    let mut intcode = ic;
    let length : usize = intcode.len();
    let mut i : usize = 0;
    while i < length {
        let mut newIntcode : Vec<i64> = Vec::new();
        let op : i64 = intcode[i];
        if op != 99i64 {
            let a  : usize = intcode[i + 1] as usize;
            let b  : usize = intcode[i + 2] as usize;
            //print!("\nOP: {} = {} + {} = {}", op, a, b, a + b);
            let result : i64 = operate(op, intcode[a] as i64, intcode[b] as i64);
            for j in 0..length {
                if j != intcode[i + 3] as usize {
                    newIntcode.push(intcode[j]);
                } else {
                    newIntcode.push(result);
                }
            }
            intcode = newIntcode;
        } else {
            //print!("\nBROKEN AT {}", i);
            break;
        }
        i += 4;
    }
    return intcode[0]
}

fn findNV(target: i64, ic: Vec<i64>) -> i64 {
    for n in 0..100 {
        for v in 0..100 {
            let mut newIC = ic.clone();
            newIC[1] = n; newIC[2] = v;
            let result = runIntCode(newIC);
            if result == target {
                return 100 * n + v
            }
        }
    }
    0
}
