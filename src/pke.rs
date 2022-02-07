/// Public-key encryption scheme
pub fn key_gen() {
	todo!("pke::key_gen()");
}

pub fn enc() {
	todo!("pke::enc()");
}

pub fn dec() {
	todo!("pke::dec()");
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
