fn main() {
    let input = include_str!("i.txt");
    let wires : Vec<String> = input.lines().map(|x| x.trim().parse().unwrap()).collect();
    let wire_a : Vec<String> = wires[0].split(",").map(|x| x.trim().parse().unwrap()).collect();
    let wire_b : Vec<String> = wires[1].split(",").map(|x| x.trim().parse().unwrap()).collect();
    let p1 : i32 = solve(wire_a.clone(), wire_b.clone(), false);
    let p2 : i32 = solve(wire_a, wire_b, true);
    print!("\nPART 1: {}\nPART 2: {}", p1, p2);
}

#[derive(Debug, Clone, Copy)]
struct WireHori {
    start: i32,
    end: i32,
    y: i32,
    stepStart: i32,
    stepEnd: i32,
    op: bool,
}

#[derive(Debug, Clone, Copy)]
struct WireVert {
    start: i32,
    end: i32,
    x: i32,
    stepStart: i32,
    stepEnd: i32,
    op: bool,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    md: i32,
    step: i32,
}

fn pathFinder(wire: Vec<String>) -> (Vec<WireHori>, Vec<WireVert>) {
    let mut hori : Vec<WireHori> = Vec::new();
    let mut vert : Vec<WireVert> = Vec::new();
    let mut xC = 0i32;
    let mut yC = 0i32;
    let mut stepC = 0i32;
    for i in 0..wire.len() {
        let copy = wire.clone();
        let direction = &(copy[i])[0..1];
        let m : i32 = (&(copy[i])[1..]).trim().parse().unwrap();
        match direction {
            "R" => {
                let h = WireHori {
                    start: xC,
                    end:   xC + m,
                    y:     yC,
                    stepStart: stepC,
                    stepEnd: stepC + m,
                    op: false,
                };
                print!("{:?}\n", h);
                hori.push(h);
                xC += m;
                stepC += m;
                //hori add init x, + x, y; xC + m
            }
            "L" => {
                let h = WireHori {
                    start: xC - m,
                    end:   xC,
                    y:     yC,
                    stepStart: stepC,
                    stepEnd: stepC + m,
                    op: true,
                };
                print!("{:?}\n", h);
                hori.push(h);
                xC -= m;
                stepC += m;
                //hori add init x, - x, y; xC - m
            }
            "U" => {
                let v = WireVert {
                    start: yC,
                    end: yC + m,
                    x: xC,
                    stepStart: stepC,
                    stepEnd: stepC + m,
                    op: false,
                };
                print!("{:?}\n", v);
                vert.push(v);
                yC += m;
                stepC += m;
                //vert, copy R
            }
            "D" => {
                let v = WireVert {
                    start: yC - m,
                    end: yC,
                    x: xC,
                    stepStart: stepC,
                    stepEnd: stepC + m,
                    op: true,
                };
                print!("{:?}\n", v);
                vert.push(v);
                yC -= m;
                stepC += m;
                //vert, copy L
            }
            _   => print!("ERROR: UNKNOWN DIRECTION"),
        }
    }
    return (hori, vert)
}

fn findIntersections(ah: Vec<WireHori>, av: Vec<WireVert>, bh: Vec<WireHori>, bv: Vec<WireVert>) -> Vec<Point> {
    let mut intersections : Vec<Point> = Vec::new();
    for h in ah.clone() {
        for v in bv.clone() {
            if h.y >= v.start && h.y <= v.end && v.x >= h.start && v.x <= h.end {
                let p = Point {
                    x: v.x,
                    y: h.y,
                    md: v.x.abs() + h.y.abs(),
                    step: h.stepStart + (v.x - if h.op { h.end } else { h.start }).abs()
                        + v.stepStart + (h.y - if v.op { v.end } else { v.start }).abs(), //--.--. 0 1 2 3 4 5 6 = h end - v x = 
                };
                //print!("{} + {} - {}: {}", h.stepStart, v.x, h.start, v.x - h.start);
                //print!("\n{} + {} - {}: {}\n", v.stepStart, h.y, v.start, h.y - v.start);
                intersections.push(p);
            }
        }
    }
    for h in bh.clone() {
        for v in av.clone() {
            if  h.y >= v.start && h.y <= v.end && v.x >= h.start && v.x <= h.end {
                let p = Point {
                    x: v.x,
                    y: h.y,
                    md: v.x.abs() + h.y.abs(),
                    step: h.stepStart + (v.x - h.start).abs()
                        + v.stepStart + (h.y - v.start).abs(),
                };
                print!("{}, {}\n", h.stepStart + (v.x - h.start).abs(), v.stepStart + (h.y - v.start).abs());
                //print!("{} + {} - {}: {}", h.stepStart, v.x, h.start, v.x - h.start);
                //print!("\n{} + {} - {}: {}\n", v.stepStart, h.y, v.start, h.y - v.start);
                intersections.push(p);
            }
        }
    }
    return intersections
}

fn solve(wire_a: Vec<String>, wire_b: Vec<String>, part2: bool) -> i32 {
    let (segments_ah, segments_av) = pathFinder(wire_a);
    let (segments_bh, segments_bv) = pathFinder(wire_b);
    let intersections = findIntersections(segments_ah, segments_av, segments_bh, segments_bv);
    if !part2 {
        return leastMD(intersections)
    }
    return leastStep(intersections)
}

fn leastMD(intersections: Vec<Point>) -> i32 {
    let mut min : i32 = i32::MAX;
    for is in intersections {
        //print!("\n{:?}", is);
        if is.md < min && is.md != 0 {
            min = is.md;
        }
    }
    return min
}

fn leastStep(intersections: Vec<Point>) -> i32 {
    let mut min : i32 = i32::MAX;
    for is in intersections {
        print!("{:?}\n", is);
        if is.step < min && is.md != 0 {
            min = is.step;
        }
    }
    return min
}
