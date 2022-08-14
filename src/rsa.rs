#[allow(dead_code)]
pub mod rsa {
    use crate::prime_generator::gen::{ generator, generator_max };
    use crate::tools::rsa_tools::{ modular_inverse, power_modulo };

    /**
     * The public key use in the RSA algorithm.
     */
    pub struct PublicKey {
        n: i128,    //crypto modulus
        e: i128,    //crypto exponent
    }

    /**
     * The private key use in the RSA algorithm.
     * !! this key must be keep secret !!
     */
    pub struct PrivateKey {
        d: i128,    //decryption exponent
    }

    /**
     * Method to generate public and privates keys.
     * Public key contains the crypto exponent and the crypto modulus.
     * Private key contains the decryption exponent.
     * 
     * !! Private key must be keep secret !!
     * @return the public and the private key
     */
    pub fn generate() -> (PublicKey, PrivateKey) {
        const N : u8 = 9;
        let p: i128 = generator(N);
        let mut q: i128 = generator(N);
        while p == q {
            q = generator(N);
        }

        let n: i128 = p * q;
        let phi: i128 = (p - 1) * (q - 1);
        let e: i128 = generator_max(phi);
        let d: i128 = modular_inverse(e, phi);
        (PublicKey {e, n}, PrivateKey { d })
    }

    impl PublicKey {
        pub fn n(&self) -> i128 {
            self.n
        }

        pub fn e(&self) -> i128 {
            self.e
        }

        pub fn encrypt(&self, message: i128) -> i128 {
            power_modulo(message, self.e, self.n)
        }
    }    

    impl PrivateKey {
        pub fn d(&self) -> i128 {
            self.d
        }

        pub fn decrypt(&self, message: i128, public: &PublicKey) -> i128 {
            power_modulo(message, self.d, public.n)
        }
    }

    /**
     * Encrypt a message with the public key.
     * @param message a 128 bits signed integer to encrypt.
     * @param public_key the public key use to encrypt the message.
     * 
     * @return the encrypted message.
     */
    pub fn encrypt(message: u128, key: &PublicKey) -> u128 {
        power_modulo(message, key.e as u128, key.n as u128)
    }

    /**
     * Decrypt a message with the private key.
     * @param cipher_message a 128 bits signed integer encrypted to decrypt.
     * @param public_key the private key use to decrypt the message.
     * @param private_key the private key use to decrypt the message.
     * 
     * @return the decrypted message.
     */
    pub fn decrypt(cipher_message: u128, public_key: &PublicKey, private_key: &PrivateKey) -> u128 {
        power_modulo(cipher_message, private_key.d as u128, public_key.n as u128)
    }

    pub fn encrypt_tab(m : &[u8], key : &PublicKey) -> Vec<u128> {
        let mut c = Vec::with_capacity(m.len());
        for i in 0..m.len() {
            c.push(encrypt(m[i] as u128, key));
        }
        c
    }

    pub fn decrypt_tab(c : &[u128], key : &PublicKey, private_key : &PrivateKey) -> Vec<u128> {
        let mut m = Vec::with_capacity(c.len());
        for i in 0..c.len() {
            m.push(decrypt(c[i], key, private_key));
        }
        m
    }

}