use num::{
    bigint::{BigInt, RandBigInt, ToBigInt},
    pow,
};
use num_traits::ToPrimitive;
use rand::{SeedableRng, StdRng};
use wasm_bindgen::prelude::*;

static SMALL_PRIMES: &'static [i32] = &[
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547,
    557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659,
    661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797,
    809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929,
    937, 941, 947, 953, 967, 971, 977, 983, 991, 997,
];

static BASES: &'static [i32] = &[2, 3, 5, 7, 11];

// Why lazy_static you may ask? Well, for one, try to compile this without lazy_static. You will
// get an error saying statics can't be the result of an executed function. So, as per the crate
// docs, with lazy_static we get Using this macro, it is possible to have statics that require
// code to be executed at runtime in order to be initialized.
lazy_static! {
    static ref ZERO: BigInt = string_to_number("0");
    static ref ONE: BigInt = string_to_number("1");
    static ref TWO: BigInt = string_to_number("2");
}

pub fn string_to_number(s: &str) -> BigInt {
     BigInt::parse_bytes(s.as_bytes(), 10).unwrap()
}

pub fn number_to_string(num: &BigInt) -> String {
     format!("{}", num)
}

#[cfg(test)]
mod test_string_to_number_macro {
    use super::*;

    #[test]
    fn negative_small() {
        let a = string_to_number("-5");
        let b = BigInt::parse_bytes(b"-5", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn negative_large() {
        let a = string_to_number("-523892389328392");
        let b = BigInt::parse_bytes(b"-523892389328392", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn miniscule() {
        let a = string_to_number("0");
        let b = BigInt::parse_bytes(b"0", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn tiny() {
        let a = string_to_number("10");
        let b = BigInt::parse_bytes(b"10", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn small() {
        let a = string_to_number("123");
        let b = BigInt::parse_bytes(b"123", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn medium() {
        let a = string_to_number("123456789");
        let b = BigInt::parse_bytes(b"123456789", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn large() {
        let a = string_to_number("123456789123456789");
        let b = BigInt::parse_bytes(b"123456789123456789", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn x_large() {
        let a = string_to_number("123456789123456789123456789123456789123456789123456789");
        let b = BigInt::parse_bytes(
            b"123456789123456789123456789123456789123456789123456789",
            10,
        ).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn xx_large() {
        let a = string_to_number("123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789");
        let b = BigInt::parse_bytes(b"123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789", 10).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn xxx_large() {
        let a = string_to_number("123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789");
        let b = BigInt::parse_bytes(b"123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789", 10).unwrap();
        assert_eq!(a, b);
    }
}

#[cfg(test)]
mod test_number_to_string_macro {
    use super::*;

    #[test]
    fn negative_small() {
        let num = BigInt::parse_bytes(b"-5", 10).unwrap();
        let a = number_to_string(&num);
        let b = "-5".to_string();
        assert_eq!(a, b);
    }

    #[test]
    fn negative_large() {
        let num = BigInt::parse_bytes(b"-523892389328392", 10).unwrap();
        let a = number_to_string(&num);
        let b = "-523892389328392".to_string();
        assert_eq!(a, b);
    }

    #[test]
    fn miniscule() {
        let num = BigInt::parse_bytes(b"0", 10).unwrap();
        let a = number_to_string(&num);
        let b = "0";
        assert_eq!(a, b);
    }

    #[test]
    fn tiny() {
        let num = BigInt::parse_bytes(b"10", 10).unwrap();
        let a = number_to_string(&num);
        let b = "10".to_string();
        assert_eq!(a, b);
    }

    #[test]
    fn small() {
        let num = BigInt::parse_bytes(b"123", 10).unwrap();
        let a = number_to_string(&num);
        let b = "123".to_string();
        assert_eq!(a, b);
    }

    #[test]
    fn medium() {
        let num = BigInt::parse_bytes(b"123456789", 10).unwrap();
        let a = number_to_string(&num);
        let b = "123456789".to_string();
        assert_eq!(a, b);
    }

    #[test]
    fn large() {
        let num = BigInt::parse_bytes(b"123456789123456789", 10).unwrap();
        let a = number_to_string(&num);
        let b = "123456789123456789".to_string();
        assert_eq!(a, b);
    }

    #[test]
    fn x_large() {
        let num = BigInt::parse_bytes(
            b"123456789123456789123456789123456789123456789123456789",
            10,
        ).unwrap();
        let a = number_to_string(&num);
        let b = "123456789123456789123456789123456789123456789123456789".to_string();
        assert_eq!(a, b);
    }

    #[test]
    fn xx_large() {
        let num = BigInt::parse_bytes(b"123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789", 10).unwrap();
        let a = number_to_string(&num);
        let b = "123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789".to_string();
        assert_eq!(a, b);
    }

    #[test]
    fn xxx_large() {
        let num = BigInt::parse_bytes(b"123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789", 10).unwrap();
        let a = number_to_string(&num);
        let b = "123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789123456789".to_string();
        assert_eq!(a, b);
    }
}

// Ported from: http://www.maths.dk/teaching/courses/math398-spring2017/code/cryptomath.txt
pub fn gcd(a: &str, b: &str) -> String {
    let mut a_num = string_to_number(a);
    let mut b_num = string_to_number(b);

    while b_num != *ZERO {
        let remainder = a_num % &b_num;
        a_num = b_num;
        b_num = remainder;
    }

    number_to_string(&a_num)
}

#[cfg(test)]
mod test_gcd {
    use super::*;

    #[test]
    fn miniscule() {
        let a = "10";
        let b = "5";
        let expected = "5";
        assert_eq!(gcd(a, b), expected);
    }

    #[test]
    fn tiny() {
        let a = "29943";
        let b = "29738";
        let expected = "1";
        assert_eq!(gcd(a, b), expected);
    }

    #[test]
    fn small() {
        let a = "299429203";
        let b = "827382738";
        let expected = "1";
        assert_eq!(gcd(a, b), expected);
    }

    #[test]
    fn medium() {
        let a = "1672976127961212891";
        let b = "3378278237328723873";
        let expected = "3";
        assert_eq!(gcd(a, b), expected);
    }

    #[test]
    fn large() {
        let a = "16729761279612128911672976127961212891";
        let b = "33782782373287238731672976127961212891";
        let expected = "3";
        assert_eq!(gcd(a, b), expected);
    }

    #[test]
    fn x_large() {
        let a = "1873817317893712873298173982173982173897128738912738217371897381374891378943789";
        let b = "9188937128738173912371837981739817238917246812647812678394619836281693618963297";
        let expected = "1";
        assert_eq!(gcd(a, b), expected);
    }

    #[test]
    fn xx_large() {
        let a = "18273781798371987398173891273871293762178362308763217863871263826817067830612083612876307916239721638916398216398613892168903681293610639120368219732891372189361287361986371863218763017236270362896319038213";
        let b = "82726226362376138712678923161327863279136912363261786391287273961273967239678123623623672369236872671268723672167267612727198623872637892632186267386219627823169783627819623761983627816378263178639821687326";
        let expected = "1";
        assert_eq!(gcd(a, b), expected);
    }

    #[test]
    fn xxx_large() {
        let a = "1827378179837198739817389127387129376217836230876321786387126382681706783061208361287630791623972163891639821639861389216890368129361063912036821973289137218936128736198637186321876301723627036289631903821318273781798371987398173891273871293762178362308763217863871263826817067830612083612876307916239721638916398216398613892168903681293610639120368219732891372189361287361986371863218763017236270362896319038213";
        let b = "8272622636237613871267892316132786327913691236326178639128727396127396723967812362362367236923687267126872367216726761272719862387263789263218626738621962782316978362781962376198362781637826317863982168732618273781798371987398173891273871293762178362308763217863871263826817067830612083612876307916239721638916398216398613892168903681293610639120368219732891372189361287361986371863218763017236270362896319038213";
        let expected = "1";
        assert_eq!(gcd(a, b), expected);
    }
}

pub fn lcm(a: &str, b: &str) -> String {
    let x = string_to_number(a);
    let y = string_to_number(b);

    let numerator = &x * &y;
    let denominator = string_to_number(&gcd(a, b));

    let lcm = numerator / denominator;

    number_to_string(&lcm)
}

#[cfg(test)]
mod test_lcm {
    use super::*;

    #[test]
    fn miniscule() {
        let a = "5";
        let b = "2";
        let expected = "10";
        assert_eq!(lcm(a, b), expected);
    }

    #[test]
    fn tiny() {
        let a = "15";
        let b = "20";
        let expected = "60";
        assert_eq!(lcm(a, b), expected);
    }

    #[test]
    fn small() {
        let a = "299429203";
        let b = "827382738";
        let expected = "247742553815297814";
        assert_eq!(lcm(a, b), expected);
    }

    #[test]
    fn medium() {
        let a = "1672976127961212891";
        let b = "3378278237328723873";
        let expected = "1883926281553946627336368887435682281";
        assert_eq!(lcm(a, b), expected);
    }
}

// Based on pseudocode from: https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
pub fn extended_gcd(a: &str, b: &str) -> (String, String) {
    let mut a_num = string_to_number(a);
    let mut b_num = string_to_number(b);

    let mut old_s: BigInt = ONE.clone();
    let mut s: BigInt = ZERO.clone();

    let mut old_t: BigInt = ZERO.clone();
    let mut t: BigInt = ONE.clone();

    while a_num != *ZERO {
        let quotient = &b_num / &a_num;

        let temp_r = a_num.clone();
        a_num = b_num - &quotient * a_num;
        b_num = temp_r;

        let temp_s = s.clone();
        s = old_s - &quotient * s;
        old_s = temp_s;

        let temp_t = t.clone();
        t = old_t - &quotient * t;
        old_t = temp_t;
    }

    (number_to_string(&old_t), number_to_string(&old_s))
}

#[cfg(test)]
mod test_extended_gcd {
    use super::*;

    #[test]
    fn miniscule() {
        let a = "12";
        let b = "17";
        let expected_a = "-7".to_string();
        let expected_b = "5".to_string();
        assert_eq!(extended_gcd(a, b), (expected_a, expected_b));
    }

    #[test]
    fn tiny() {
        let a = "180";
        let b = "150";

        let (x, y) = extended_gcd(a, b);
        let expected_x = "1";
        let expected_y = "-1".to_string();

        assert_eq!(x, expected_x);
        assert_eq!(y, expected_y);
    }

    #[test]
    fn small() {
        let a = "39392";
        let b = "9372";
        let expected_a = "950".to_string();
        let expected_b = "-3993".to_string();
        assert_eq!(extended_gcd(a, b), (expected_a, expected_b));
    }

    #[test]
    fn medium() {
        let a = "29837362344";
        let b = "20938934792";
        let expected_a = "70028498".to_string();
        let expected_b = "-99788537".to_string();
        assert_eq!(extended_gcd(a, b), (expected_a, expected_b));
    }

    #[test]
    fn large() {
        let a = "298373623442234243";
        let b = "224224424293284938";
        let expected_a = "29778059398942417".to_string();
        let expected_b = "-39625422207881285".to_string();
        assert_eq!(extended_gcd(a, b), (expected_a, expected_b));
    }

    #[test]
    fn x_large() {
        let a = "1873817317893712873298173982173982173897128738912738217371897381374891378943789";
        let b = "9188937128738173912371837981739817238917246812647812678394619836281693618963297";
        let expected_a =
            "-1486080468736810267748473400213603987572344688308283414544810257982300340964938"
                .to_string();
        let expected_b =
            "303043026531734365807887908508346161442903254640489390676789939813131430349539"
                .to_string();
        assert_eq!(extended_gcd(a, b), (expected_a, expected_b));
    }
}

// Ported from: http://www.maths.dk/teaching/courses/math398-spring2017/code/cryptomath.txt
pub fn mod_inverse(a: &str, m: &str) -> Option<String> {
    let gcd_num = string_to_number(&gcd(&a, &m));
    if gcd_num != *ONE {
        return None;
    }

    let (u, _) = extended_gcd(&a, &m);

    let u_num = string_to_number(&u);
    let m_num = string_to_number(m);

    Some(number_to_string(&(u_num % m_num)))
}

#[cfg(test)]
mod test_mod_inverse {
    use super::*;

    #[test]
    fn miniscule() {
        let a = "3";
        let m = "26";
        let expected = Some("9".to_string());
        assert_eq!(mod_inverse(a, m), expected);
    }

    #[test]
    fn tiny() {
        let a = "333";
        let m = "2613";
        let expected = None;
        assert_eq!(mod_inverse(a, m), expected);
    }

    #[test]
    fn small() {
        let a = "333213";
        let m = "261312334";
        let expected = Some("66480691".to_string());
        assert_eq!(mod_inverse(a, m), expected);
    }

    #[test]
    fn medium() {
        let a = "3332131312321";
        let m = "261312334131135465";
        let expected = Some("63643812378874741".to_string());
        assert_eq!(mod_inverse(a, m), expected);
    }

    #[test]
    fn large() {
        let a = "33321313123211923123812";
        let m = "261312334131135465912381278381238";
        let expected = None;
        assert_eq!(mod_inverse(a, m), expected);
    }

    #[test]
    fn x_large() {
        let a = "1873817317893712873298173982173982173897128738912738217371897381374891378";
        let m = "9188937128738173912371837981739817238917246812647812678394619836281693618963297";
        let expected = Some(
            "-996417904483222556354083958060155179719360472118047976841259037232297184027911"
                .to_string(),
        );
        assert_eq!(mod_inverse(a, m), expected);
    }
}

// Check out: https://rosettacode.org/wiki/Miller%E2%80%93Rabin_primality_test
pub fn miller_rabin(n: &str, seed: &[u8]) -> bool {
    let n_num: BigInt = string_to_number(n);
    let n_minus_one = &n_num - &*ONE;

    if n_num == *TWO {
        return true;
    }

    if n_num < *TWO || &n_num % &*TWO == *ZERO {
        return false;
    }

    let mut s: BigInt = ZERO.clone();
    let mut d: BigInt = &n_num - &*ONE;

    while &d % &*TWO == *ZERO {
        s += &*ONE;
        d /= &*TWO;
    }

    let mut rng: StdRng = SeedableRng::from_seed(from_slice(&seed));

    // 50 here is a parameter for accuracy
    for _ in 0..50 {
        let a_num = rng.gen_bigint_range(&*TWO, &n_minus_one);
        let a_str = number_to_string(&a_num);

        let gcd_str = gcd(&a_str, &n);
        let gcd_num = string_to_number(&gcd_str);

        if gcd_num != *ONE {
            return false;
        }

        let mut x_num = a_num.modpow(&d, &n_num);

        if x_num == *ONE || x_num == n_minus_one {
            continue;
        }

        let mut is_witness = true;
        let mut r = ONE.clone();

        while r < s && is_witness {
            x_num = x_num.modpow(&*TWO, &n_num);

            if x_num == n_minus_one {
                is_witness = false;
            }

            r += &*ONE;
        }

        if is_witness {
            return false;
        }
    }

    true
}

pub fn is_prime(n: &str, seed: &[u8]) -> bool {
    let n_num = string_to_number(n);

    let n_minus_one = &n_num - &*ONE;

    if n_num < *TWO {
        return false;
    }

    let small_primes_as_bigints: Vec<BigInt> = SMALL_PRIMES
        .iter()
        .map(|x| x.to_bigint().unwrap())
        .collect();
    let is_small_prime = small_primes_as_bigints.contains(&n_num);

    if is_small_prime {
        return true;
    }

    for prime in small_primes_as_bigints {
        if &n_num % &prime == *ZERO {
            return false;
        }
    }

    let bases_as_bigints: Vec<BigInt> = BASES.iter().map(|x| x.to_bigint().unwrap()).collect();

    for base in &bases_as_bigints {
        if base.modpow(&n_minus_one, &n_num) != *ONE {
            return false;
        }
    }

    miller_rabin(n, seed)
}

#[cfg(test)]
mod test_is_prime_and_rabin_miller {
    use super::*;

    #[test]
    fn miniscule_prime() {
        let a = "3";
        let expected = true;
        assert_eq!(is_prime(a, test_seed()), expected);
    }

    #[test]
    fn miniscule_not_prime() {
        let a = "4";
        let expected = false;
        assert_eq!(is_prime(a, test_seed()), expected);
    }

    #[test]
    fn tiny_prime() {
        let a = "1049";
        let expected = true;
        assert_eq!(is_prime(a, test_seed()), expected);
    }

    #[test]
    fn tiny_not_prime() {
        let a = "1050";
        let expected = false;
        assert_eq!(is_prime(a, test_seed()), expected);
    }

    #[test]
    fn small_prime() {
        let a = "100103";
        let expected = true;
        assert_eq!(is_prime(a, test_seed()), expected);
    }

    #[test]
    fn small_not_prime() {
        let a = "100105";
        let expected = false;
        assert_eq!(is_prime(a, test_seed()), expected);
    }

    #[test]
    fn medium_prime() {
        let a = "100000015333";
        let expected = true;
        assert_eq!(is_prime(a, test_seed()), expected);
    }

    #[test]
    fn medium_not_prime() {
        let a = "100000015334";
        let expected = false;
        assert_eq!(is_prime(a, test_seed()), expected);
    }

    #[test]
    fn large_prime() {
        let a = "335184372088831";
        let expected = true;
        assert_eq!(is_prime(a, test_seed()), expected);
    }

    #[test]
    fn large_not_prime() {
        let a = "335184372088832";
        let expected = false;
        assert_eq!(is_prime(a, test_seed()), expected);
    }
}

pub fn generate_prime(bits: usize, tries: usize, seed: &[u8]) -> Option<String> {
    let bits_minus_one = bits - 1;
    let x = pow(TWO.clone(), bits_minus_one);
    let y = &*TWO * &x;

    let mut rng: StdRng = SeedableRng::from_seed(from_slice(&seed));

    for _ in 0..tries {
        let mut n = rng.gen_bigint_range(&x, &y);

        if &n % &*TWO == *ZERO {
            n += 1;
        }

        let num_str = &number_to_string(&n);
        let q = is_prime(num_str, seed);

        if q {
            return Some(number_to_string(&n));
        }
    }

    None
}

#[cfg(test)]
mod test_generate_prime {
    use super::*;

    #[test]
    fn miniscule_prime() {
        let prime = generate_prime(2, 1000, test_seed());
        assert_eq!(prime, Some("3".to_string()));
    }

    #[test]
    fn tiny_prime() {
        let prime = generate_prime(8, 1000, test_seed());
        assert_eq!(prime, Some("193".to_string()));
    }

    #[test]
    fn medium_prime() {
        let prime = generate_prime(64, 1000, test_seed());
        assert_eq!(prime, Some("10057321802802702503".to_string()));
    }

    #[test]
    fn large_prime() {
        let prime = generate_prime(256, 1000, test_seed());
        assert_eq!(
            prime,
            Some(
                "91585194753718779240055081770127290880143452499556598946529982336565467053363"
                    .to_string()
            )
        );
    }
}

// Ref: https://stackoverflow.com/questions/29570607/is-there-a-good-way-to-convert-a-vect-to-an-array
fn from_slice(bytes: &[u8]) -> [u8; 32] {
    let mut array = [0; 32];
    let bytes = &bytes[..array.len()]; // panics if not enough data
    array.copy_from_slice(bytes);
    array
}

// Fixes: https://github.com/ColbyCypherSociety/ChatDemo/issues/21
// Ref: https://stackoverflow.com/questions/46378637/how-to-make-a-variable-with-a-scope-lifecycle-for-all-test-functions-in-a-rust-t
#[allow(dead_code)]
fn test_seed<'a>() -> &'a[u8] {
    &[
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1,
    ]
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Keypair {
    // Public key
    e: String,
    // Private key
    d: String,
    // Modulo (both public and private)
    n: String,
}

#[wasm_bindgen]
impl Keypair {
    pub fn new(seed_one: &[u8], seed_two: &[u8]) -> Keypair {
        // Hardcoded to 256-bits with 1000 tries for now
        let q_str = generate_prime(256, 1000, &seed_one).unwrap();
        let q_num = string_to_number(&q_str);

        // Hardcoded to 256-bits with 1000 tries for now
        let p_str = generate_prime(256, 1000, &seed_two).unwrap();
        let p_num = string_to_number(&p_str);

        let n_num = &p_num * &q_num;
        let n_str = number_to_string(&n_num);

        let p_minus_one_str = number_to_string(&(&p_num - &*ONE));
        let q_minus_one_str = number_to_string(&(&q_num - &*ONE));

        let phi_str = lcm(&p_minus_one_str, &q_minus_one_str);
        let phi_num = string_to_number(&phi_str);

        let mut e_found = false;

        let mut rng: StdRng = SeedableRng::from_seed(from_slice(&seed_one));

        let mut e_str = String::default();

        while !e_found {
            let e_num = rng.gen_bigint_range(&*TWO, &(&phi_num - &*TWO));

            e_str = number_to_string(&e_num);
            if gcd(&e_str, &phi_str) == "1" {
                e_found = true;
            }
        }

        let mut d_str = mod_inverse(&e_str, &phi_str).unwrap();

        if &*d_str <  "0" {
            let d_num = &n_num + string_to_number(&d_str);
            d_str = number_to_string(&d_num);
        }

        Keypair {
            e: e_str,
            d: d_str,
            n: n_str,
        }
    }

    pub fn public_key_display_wasm(&self) -> String {
        format!("({}, {})", self.e, self.n)
    }

    pub fn decrypt(&self, ciphertext: &str) -> String {
        let private_key = string_to_number(&self.d);
        let modulus = string_to_number(&self.n);

        let mut decrypted_values: Vec<char> = Vec::new();

        for c in ciphertext.split(',') {
            let to_decrypt = string_to_number(c);
            let decrypted = to_decrypt.modpow(&private_key, &modulus);
            let decrypted_u8 = decrypted.to_u8();
            match decrypted_u8 {
                Some(d_u8) => decrypted_values.push(d_u8 as char),
                _ => (),
            }
        }

        decrypted_values.iter().collect()
    }
}

#[cfg(test)]
mod test_generate_key {
    use super::*;

    #[test]
    fn works_with_simple_encrypt_decrypt() {
        // You need two different seeds (p and q must be different)
        let seed_one = &[
            10, 16, 51, 42, 123, 31, 212, 31, 233, 15, 9, 7, 41, 32, 4, 3, 144, 122, 1, 35, 1, 13,
            55, 23, 1, 33, 1, 1, 1, 1, 2, 1,
        ];
        let seed_two = test_seed();

        // Generate a keypair
        let k = Keypair::new(seed_one, seed_two);

        // Capture all the variables for encryption and decryption
        let e = string_to_number(&k.e);
        let d = string_to_number(&k.d);
        let n = string_to_number(&k.n);

        // Message and ciphertext
        let plaintext = string_to_number("72");
        let ciphertext = plaintext.modpow(&e, &n);

        let decrypted = ciphertext.modpow(&d, &n);
        assert_eq!(plaintext, decrypted);
    }
}

#[wasm_bindgen]
pub fn encrypt(m: &str, e: &str, n: &str) -> String {
    let public_key = string_to_number(e);
    let modulus = string_to_number(n);

    let mut encrypted_values = String::default();

    for c in m.bytes() {
        let c_str = c.to_string();
        let to_encrypt = string_to_number(&c_str);
        let encrypted = to_encrypt.modpow(&public_key, &modulus);

        encrypted_values = format!("{},{}", encrypted_values, number_to_string(&encrypted));
    }

    encrypted_values
}

#[cfg(test)]
mod test_encrypt_decrypt {
    use super::*;

    #[test]
    fn complete_encrypt_and_decrypt() {
        // You need two different seeds (p and q must be different)
        let seed_one = &[
            10, 16, 51, 42, 123, 31, 212, 31, 233, 15, 9, 7, 41, 32, 4, 3, 144, 122, 1, 35, 1, 13,
            55, 23, 1, 33, 1, 1, 1, 1, 2, 1,
        ];
        let seed_two = test_seed();

        // Generate a keypair
        let k = Keypair::new(seed_one, seed_two);

        // Message and ciphertext
        let plaintext = "HelloWorld!";
        let ciphertext = encrypt(plaintext, &k.e, &k.n);
        let decrypted = k.decrypt(&ciphertext[1..]);

        assert_eq!(plaintext, decrypted);
    }
}
