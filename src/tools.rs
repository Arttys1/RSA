#[allow(dead_code)]

pub mod rsa_tools {
    /**
     * Modular exponentiation by square algorithm.
     * This function return the Modular exponentiation in a O(log(n)) complexity.
     * It can takes all primivites integers
     *
     * @param nb the base.
     * @param pow the exponent.
     * @param modulo the modulo.
     *
     * @return nb^pow % modulo
     */
    pub fn power_modulo<T>(mut nb: T, mut pow :T, modulo: T) -> T
    where
        T :  num::PrimInt + std::ops::Shr<T, Output = T>
    {
        let mut result: T = T::one();

        while pow > T::zero() {
            if pow & T::one() > T::zero() {
                result = (result * nb) % modulo;
            }
            pow = pow >> T::one();
            nb = (nb * nb) % modulo;
        }

        result
    }

    /**
     * Extended Euclidean algorithm.
     * Returns a tuple (r, u, v) where r is the greatest common divisor of a and b.
     * If a and b are coprime then u and v are the multiplicative inverses of a and b.
     * It can takes all primivites signed integers
     */
    pub fn extended_euclidean_algorithm<T>(a: T, b: T) -> (T, T, T)
    where
        T : num::PrimInt + num::Signed
    {
        let mut r: T = a;
        let mut u: T = T::one();
        let mut v: T = T::zero();
        let mut s: T = b;
        let mut t: T = T::zero();
        let mut p: T = T::one();

        while s != T::zero() {
            let  q: T = r / s;
            let rs: T = r;
            let us: T = u;
            let vs: T = v;
            r = s;
            u = t;
            v = p;
            s = rs - q*s;
            t = us - q*t;
            p = vs - q*p;
        }

        (r, u, v)
    }

    /**
     * Extended Euclidean algorithm.
     * Returns a tuple (r, u, v) where r is the greatest common divisor of a and b.
     * If a and b are coprime then u and v are the multiplicative inverses of a and b.
     */
    pub fn extended_euclidean_algorithm_unsigned<T>(a: T, b: T) -> (T, T, T)
    where
        T : num::PrimInt
    {
        let two = T::one() + T::one();
        let mut r0 = a;
        let mut s0 = T::one();
        let mut t0 = T::zero();
        let mut r1 = b;
        let mut s1 = T::zero();
        let mut t1 = T::one();
        let mut n = T::zero();
        while r1 != T::zero() {
            let q = r0 / r1;
            r0 = if r0 > q * r1 { r0-q *r1 } else { q*r1-r0 };
            std::mem::swap(&mut r0, &mut r1);
            s0 = s0+q*s1;
            std::mem::swap(&mut s0, &mut s1);
            t0 = t0+q*t1;
            std::mem::swap(&mut t0, &mut t1);
            n = n + T::one();
        }
        // gcd = r0
        if n % two != T::zero() {
            s0 = b - s0;
        }
        else {
            t0 = a - t0;
        }
        (r0, s0, t0)
    }

    /**
     * Returns the greatest common divisor of a and b.
     * It implements the Euclidean algorithm.
     * It can takes all primivites integers
     */
    pub fn gcd<T>(mut a: T, mut b: T) -> T
    where
        T : num::PrimInt
    {
        while b != T::zero() {
            let t: T = b;
            b = a % b;
            a = t;
        }

        a
    }

    /**
     * Returns the modular inverse of a and b.
     * !! a and b must be coprime !!
     * Panic if there is no modular inverse (a and b no coprime).
     */
    pub fn modular_inverse<T>(a: T, b: T) -> T
    where
        T : num::PrimInt
    {
        let (r, _, v): (T, T, T) = extended_euclidean_algorithm_unsigned(b, a);
        if r != T::one() {
            panic!("No modular inverse");
        }
        if v < T::zero() {          //correct v if the sign is negative
            return (v + b) % b;     //custom modulo with negative sign
        }                           //v mod b (with v negatif)
        v
    }
}
