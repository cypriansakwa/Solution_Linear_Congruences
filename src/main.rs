use std::vec::Vec;

fn main() {
    let a = 507444444444339381111111111;
    let b = 183838383382222;
    let n = 71338888434334;

    match solve_linear_congruence(a, b, n) {
        Some(solutions) => {
            println!("Solutions to {}x \u{2261}  {} (mod {}): {:?}", a, b, n, solutions);
        }
        None => {
            println!("The congruence {}x \u{2261}  {} (mod {}) has no solutions", a, b, n);
        }
    }
}

fn gcd_extended(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        return (b, 0, 1);
    }
    let (g, x1, y1) = gcd_extended(b % a, a);
    let x = y1 - (b / a) * x1;
    let y = x1;
    (g, x, y)
}

fn mod_inverse(a: i128, n: i128) -> Option<i128> {
    let (g, x, _) = gcd_extended(a, n);
    if g != 1 {
        None
    } else {
        Some((x % n + n) % n)
    }
}

fn solve_linear_congruence(a: i128, b: i128, n: i128) -> Option<Vec<i128>> {
    let (g, _x, _) = gcd_extended(a, n); // Prefix `_x` to suppress warning

    if b % g != 0 {
        return None; // No solutions
    }

    let a_prime = a / g;
    let b_prime = b / g;
    let n_prime = n / g;

    let x0 = (b_prime * mod_inverse(a_prime, n_prime)?) % n_prime;
    let mut solutions = Vec::new();

    for i in 0..g {
        solutions.push((x0 + i * n_prime) % n);
    }

    Some(solutions)
}

