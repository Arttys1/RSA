#[allow(dead_code)]
pub mod rsa {
    use crate::prime_generator::gen::{ generator, generator_max };
    use crate::tools::rsa_tools::{ modular_inverse, power_modulo };

    //trait to regroup multiple traits
    pub trait RsaInt:
        Copy +
        Clone +
        From<u8> +
        std::ops::Shr<u8,Output = Self> +
        std::ops::Shl<u8,Output = Self> +
        std::ops::BitAnd<Output = Self> +
        std::fmt::Display +
        num::Integer +
        num::ToPrimitive +
        num::traits::Pow<u32, Output = Self> +
        rand::distributions::uniform::SampleUniform {}

    impl<T:
        Copy +
        Clone +
        From<u8> +
        std::ops::Shr<u8,Output = Self> +
        std::ops::Shl<u8,Output = Self> +
        std::ops::BitAnd<Output = Self> +
        std::fmt::Display +
        num::Integer +
        num::ToPrimitive +
        num::traits::Pow<u32, Output = Self> +
        rand::distributions::uniform::SampleUniform>
    RsaInt for T {}

    /**
     * The public key use in the RSA algorithm.
     */
    pub struct PublicKey<T>
    where T: RsaInt
    {
        n: T,    //crypto modulus
        e: T,    //crypto exponent
    }

    /**
     * The private key use in the RSA algorithm.
     * !! this key must be keep secret !!
     */
    pub struct PrivateKey<T>
    where T: RsaInt
    {
        d: T,    //decryption exponent
    }

    /**
     * Method to generate public and privates keys.
     * Public key contains the crypto exponent and the crypto modulus.
     * Private key contains the decryption exponent.
     *
     * !! Private key must be keep secret !!
     * @return the public and the private key
     */
    pub fn generate<T>(size: u8) -> (PublicKey<T>, PrivateKey<T>)
    where T: RsaInt
    {
        let p: T = generator(size);
        let mut q: T = generator(size);
        while p == q {
            q = generator(size);
        }
        let n: T = p * q;
        let phi: T = (p - T::one()) * (q - T::one());
        let e: T = generator_max(phi);
        let d: T = modular_inverse(e, phi);
        (PublicKey {e, n}, PrivateKey { d })
    }

    impl<T> PublicKey<T>
    where T: RsaInt
    {
        pub fn construct(e: T, n: T) -> PublicKey<T> {
            PublicKey {e, n}
        }

        pub fn n(&self) -> T {
            self.n
        }

        pub fn e(&self) -> T {
            self.e
        }

        pub fn encrypt(&self, message: T) -> T {
            power_modulo(message, self.e, self.n)
        }

        pub fn to_string(&self) -> String {
            format!("--BEGIN_PUBLIC_KEY--||{}||{}||--END_PUBLIC_KEY--", self.n, self.e)
        }
    }

    impl<T> PrivateKey<T>
    where T: RsaInt
    {
        pub fn construct(d: T) -> PrivateKey<T> {
            PrivateKey{d}
        }

        pub fn d(&self) -> T {
            self.d
        }

        pub fn decrypt(&self, message: T, public: &PublicKey<T>) -> T {
            power_modulo(message, self.d, public.n)
        }

        pub fn to_string(&self) -> String {
            format!("--BEGIN_PRIVATE_KEY--||{}||--END_PRIVATE_KEY--", self.d)
        }
    }

    /**
     * Encrypt a message with the public key.
     * @param message a 128 bits signed integer to encrypt.
     * @param public_key the public key use to encrypt the message.
     *
     * @return the encrypted message.
     */
    pub fn encrypt<T>(message: T, key: &PublicKey<T>) -> T
    where T: RsaInt
    {
        power_modulo(message, key.e, key.n)
    }

    /**
     * Decrypt a message with the private key.
     * @param cipher_message a 128 bits signed integer encrypted to decrypt.
     * @param public_key the private key use to decrypt the message.
     * @param private_key the private key use to decrypt the message.
     *
     * @return the decrypted message.
     */
    pub fn decrypt<T>(cipher_message: T, public_key: &PublicKey<T>, private_key: &PrivateKey<T>) -> T
    where T: RsaInt
    {
        power_modulo(cipher_message, private_key.d, public_key.n)
    }

    pub fn encrypt_tab<T>(m : &[u8], key : &PublicKey<T>) -> Vec<T>
    where T: RsaInt
    {
        let mut c = Vec::with_capacity(m.len());
        for i in 0..m.len() {
            c.push(encrypt(T::from(m[i]), key));
        }
        c
    }

    pub fn decrypt_tab<T>(c : &[T], key : &PublicKey<T>, private_key : &PrivateKey<T>) -> Vec<T>
    where T: RsaInt
    {
        let mut m = Vec::with_capacity(c.len());
        for i in 0..c.len() {
            m.push(decrypt(c[i], key, private_key));
        }
        m
    }

}
