#[allow(dead_code)]
pub mod gen {
    use rand::Rng;

    use crate::{primality::primality::is_prime_, rsa::rsa::RsaInt};

    /**
     * Generate a random prime number of n numbers.
     */
    pub fn generator<T>(n: u8) -> T
    where T: RsaInt
    {
        let mut rng: T = random_n_number_(n);
        while (rng & T::one()) == T::zero() || !is_prime_(rng) {
            rng = rng + T::one();
        }

        rng
    }

    pub fn generator_min<T>(min : T) -> T
    where T : RsaInt
    {
        let mut rng = rand::thread_rng();
        let twomin = min << 1u8; // min * 2
        let mut n: T = rng.gen_range(min..=twomin);

        while n & T::one() == T::zero() || !is_prime_(n) {
            n = n + T::one();
            if n > twomin {
                n = rng.gen_range(min..=twomin);
            }
        }

        n
    }

    pub fn generator_max<T>(max: T) -> T
    where T: RsaInt
    {
        let mut rng = rand::thread_rng();
        let halfmax = max >> 1u8; // max / 2;
        let mut n: T = rng.gen_range(halfmax..max);

        while n & T::one() == T::zero() || !is_prime_(n) {
            n = n + T::one();
            if n > max {
                n = rng.gen_range(halfmax..max);
            }
        }

        n
    }

    /**
     * Generate a random number of n numbers.
     */
    fn random_n_number_<T>(n: u8) -> T
    where T: RsaInt
    {
        let mut number : T = T::zero();
        let mut rng = rand::thread_rng();
        let ten: T = T::from(10);              //redefine 10 as a T type

        for i in 0..n {
            let a: T = rng.gen_range(T::zero()..ten);
            number = number + (a * ten.pow(i as u32));
        }
        number
    }
}
