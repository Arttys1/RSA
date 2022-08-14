#[allow(dead_code)]
pub mod primality {
    use num::PrimInt;
    use rand::Rng;

    use crate::tools::rsa_tools::power_modulo;

    const K : u8 = 7;   

    /**
     * Generics implementation of the Miller-Rabin test.
     * @param n the number to test.
     * @return true if n is prime, false otherwise.
     */
    pub fn is_prime_<T>(n: T) -> bool 
    where 
        T : PrimInt + rand::distributions::uniform::SampleUniform 
        + std::ops::Shr<T, Output = T>
     {
        let two: T = T::one() + T::one();
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

        for _ in 0..K {
            let a:T= rng.gen_range(two ..= n - two);     
            if miller_rabin_test_(n, a) {
                return false;
            }
        }
        true
    }

    
    fn miller_rabin_test_<T>(n: T, a: T) -> bool
    where 
        T: PrimInt + std::ops::Shr<T, Output = T>
    {
        let tab : (T, T) = s_and_d_(n);
        let s : T = tab.0;
        let d : T = tab.1;
        let mut x: T = power_modulo::<T>(a, d, n);

        if x == T::one() || x == n - T::one() {
            return false;
        }
        
        let stop = if s > T::zero() { s - T::one() } else { T::zero() };
        for _ in num::iter::range(T::zero(), stop) {
            x = power_modulo::<T>(x, T::one() + T::one(), n);
            if x == n - T::one() {
                return false;
            }  
        }

        true
    }

    fn s_and_d_<T>(n: T) -> (T, T)
    where
        T : PrimInt
    {
        let two: T = T::one() + T::one();
        let mut d: T = n - T::one();
        let mut s: T = T::zero();
        while d % two == T::zero() {
            d = d / two;
            s = s + T::one();
        }
        (s, d)
    }

    //DEAD_CODE//

    /**
     * function who check if a number is prime, thanks to the Miller-Rabin test
     */
    fn is_prime(n: u128) -> bool {
        let mut rng = rand::thread_rng();
        for _ in 0..K {
            let a : u128 = rng.gen_range(2..=n-2);     
            if miller_rabin_test_(n, a) {
                return false;
            }
        }
        true
    }

    fn miller_rabin_test(n: u128, a: u128) -> bool {
        let tab : (u128, u128) = s_and_d(n);
        let s : u128 = tab.0;
        let d : u128 = tab.1;
        let mut x: u128 = power_modulo::<u128>(a, d, n);

        if x == 1 || x == n - 1 {
            return false;
        }

        for _ in 0.. (if s > 0 { s - 1 } else { 0 }) {
            x = power_modulo::<u128>(x, 2, n);
            if x == n - 1 {
                return false;
            }      
        }

        true
    }

    fn s_and_d(n: u128) -> (u128, u128) {
        let mut d: u128 = n - 1;
        let mut s: u128 = 0;
        while d % 2 == 0{
            d /= 2;
            s += 1;
        }
        (s, d)
    }



}