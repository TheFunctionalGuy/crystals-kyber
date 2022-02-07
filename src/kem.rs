/// Key-encapsulation mechanism
pub fn key_gen() {
	todo!("kem::key_gen()");
}

pub fn enc() {
	todo!("kem::enc()");
}

pub fn dec() {
	todo!("kem::dec()");
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_enc_dec_equals_plaintext() {
		// TODO: Create real plaintext
		let plaintext = ();

		//assert_eq!(enc(dec(plaintext)), plaintext);
		enc();
		dec();
	}

	#[test]
	fn test_dec_enc_equals_ciphertext() {
		// TODO: Create real ciphertext
		let ciphertext = ();

		//assert_eq!(dec(enc(ciphertext)), ciphertext);
		dec();
		enc();
	}
}
