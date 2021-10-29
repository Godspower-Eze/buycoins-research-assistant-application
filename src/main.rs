use modinverse::egcd;


fn main() {
    find_egcd(17, 3);
}

/**
 * The multiplicative inverse of any number is that number that when that when 
 * multiplied by the number gives 1.
 * 
 * Example:
 * The multiplicative inverse of 5 is 1/5 because 5 * 1/5 = 1
 * 
 * Furthermore, A multiplicative inverse of a mod n is ā such that a * ā ≣ 1(mod n)
 * 
 * The multiplicative inverse of a number when dealing modulus. This is the number that
 * when multiplied by the number that when multiplied it should give 1 in the modulus. In this case, the
 * number is 6 because 3 * 6 is 18 and 18 in modulus 17 is 1.
 * 
 * Examples:
 * 
 * 3 mod 17. Here, we would be looking for a number that when multiplied by 3, we would get 1 in modulus 17
 * 
 * Solving the Multiplicative Inverse using the Extented Euclidean Algorithm involves the following steps:
 * 
 * 1. Check that the GCD(Greatest common Divisor) is equals 1. If yes, the we can move further else we say that
 * the number doesn't have a multiplicative inverse in that modulus.
 * 
 * That is:
 * 
 * GCD(a, n) == 1
 * 
 * 2. Then, find the modular inverse using the Extented Euclidean Algorithm
 */

fn compute_gcd(a: i64, b: i64) -> i64 {

    // Sets a mutable variable of max to a
    let mut max = a;
    // Sets a mutable variable of min to a
    let mut min = b;
    
    // Checks if the min value is greater than max, then switches
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    // Infinite loop through till it finds the GCD  
    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

// A function that checks that the gcd is 1, then uses the extended euclidean algorithm
// to compute the multiplicative inverse. 
// Pass the bigger value as "a" and the smaller value as "b"
fn find_egcd(a: i64, b: i64){

    let gcd = compute_gcd(a, b);

    if gcd == 1{
        // egcd(a, b) is a function that computes the multiplicative inverse using extended euclidean algorithm
        // g is the value of GCD and y is the multiplicative inverse
        let (g, x, y) = egcd(a, b);
        println!("GCD: {}, Multiplicative Inverse: {}", gcd, y);
    }else{
        println!("The multiplicative inverse does not exist");
    }
}