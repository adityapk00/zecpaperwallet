
use zip32::{ChildIndex, ExtendedSpendingKey};
use bech32::{Bech32, u5, ToBase32};
use rand::{OsRng, Rng};

pub fn get_address() -> (String, String) {
    
    let mut rng = OsRng::new().expect("Error opening random number generator");
    let mut seed:[u8; 32] = [0; 32]; 
    rng.fill_bytes(&mut seed);

    let spk: ExtendedSpendingKey = ExtendedSpendingKey::from_path(
            &ExtendedSpendingKey::master(&seed),
            &[
                ChildIndex::Hardened(32),
                ChildIndex::Hardened(44),
                ChildIndex::Hardened(0),
            ],
        );

    let (_d, addr) = spk.default_address().expect("Cannot get result");
    //println!("{:?}", d.0);
    //println!("{:?}", addr.diversifier.0);

    // Address    
    let mut v = vec![0; 43];
    v.get_mut(..11).unwrap().copy_from_slice(&addr.diversifier.0);
    addr.pk_d.write(v.get_mut(11..).unwrap()).expect("Cannot write!");


    let checked_data: Vec<u5> = v.to_base32();
    let encoded = Bech32::new("zs".into(), checked_data).expect("bech32 failed").to_string();
    //println!("{}", encoded);

    // Private Key
    let mut vp = Vec::new();
    spk.write(&mut vp).expect("Can't write private key");
    //println!("Len {:?}", vp.len());
    
    let c_d: Vec<u5> = vp.to_base32();
    let encoded_pk = Bech32::new("secret-extended-key-main".into(), c_d).expect("bech32 failed").to_string();
    //println!("{}", encoded_pk);
    

    //println!("Hello, {}, {}!", encoded, encoded_pk);
    return (encoded.to_string(), encoded_pk.to_string());
}