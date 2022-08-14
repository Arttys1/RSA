#[allow(dead_code)]
pub mod gen {
    use num::PrimInt;
    use rand::Rng;

    use crate::primality::primality::{ is_prime_ };

    /**
     * Generate a random prime number of n numbers.
     */
    pub fn generator<T>(n: u8) -> T 
    where
        T : PrimInt  + rand::distributions::uniform::SampleUniform + std::ops::Shr<T, Output = T>
    {
        let mut rng: T = random_n_number_(n);  
        let two : T = T::one() + T::one();  
                
        while (rng % two) == T::zero() || !is_prime_(rng) {
            rng = rng + T::one();
        }

        rng
    }

    pub fn generator_min<T>(min : T) -> T 
    where
        T : PrimInt + rand::distributions::uniform::SampleUniform + std::ops::Shr<T, Output = T>
    {
        let mut rng = rand::thread_rng();
        let two : T = T::one() + T::one();  
        
        let mut n: T = rng.gen_range(min..=min * two);
        
        while n % two == T::zero() || !is_prime_(n) {
            n = n + T::one();
        }
        
        n
    }

    pub fn generator_max<T>(max: T) -> T 
    where
        T : PrimInt + rand::distributions::uniform::SampleUniform + std::ops::Shr<T, Output = T>
    {
        let mut rng = rand::thread_rng();
        let two : T = T::one() + T::one();  
        
        let mut n: T = rng.gen_range((max / two) ..max);
        
        while n % two == T::zero() || !is_prime_(n) {
            n = n + T::one();
        }
        
        n
    }

    /**
     * Generate a random number of n numbers.
     */
    fn random_n_number_<T>(n: u8) -> T
    where
        T : PrimInt  + rand::distributions::uniform::SampleUniform
    {       
        let mut number : T = T::zero();
        let mut rng = rand::thread_rng();
        let ten: T = T::from(10).unwrap();              //redefine 10 as a T type

        for i in 0..n {
            let a: T = rng.gen_range(T::zero()..ten);
            number = number + (a * ten.pow(i as u32));
        }
        number
    }
}