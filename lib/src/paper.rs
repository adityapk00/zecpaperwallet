
use zip32::{ChildIndex, ExtendedSpendingKey};
use bech32::{Bech32, u5, ToBase32};
use rand::{Rng, ChaChaRng, FromEntropy};
use json::{array, object};

/**
 * Generate a series of `count` addresses and private keys. 
 */
pub fn generate_wallet(testnet: bool, count: u32) -> String {
    let mut rng = ChaChaRng::from_entropy();
    let mut seed:[u8; 32] = [0; 32]; 
    rng.fill(&mut seed);

    return gen_addresses_with_seed_as_json(testnet, count, &seed);
}

fn gen_addresses_with_seed_as_json(testnet: bool, count: u32, seed: &[u8; 32]) -> String {
    let mut ans = array![];

    for i in 0..count {
        let (addr, pk) = get_address(testnet, &seed);
        ans.push(object!{
                "num"           => i,
                "address"       => addr,
                "private_key"   => pk
        }).unwrap(); 
    }      

    return json::stringify_pretty(ans, 2);
}

fn get_address(testnet: bool, seed: &[u8; 32]) -> (String, String) {
    let addr_prefix = if testnet {"ztestsapling"} else {"zs"};
    let pk_prefix   = if testnet {"secret-extended-key-test"} else {"secret-extended-key-main"};
    
    let spk: ExtendedSpendingKey = ExtendedSpendingKey::from_path(
            &ExtendedSpendingKey::master(seed),
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
    let encoded = Bech32::new(addr_prefix.into(), checked_data).expect("bech32 failed").to_string();
    //println!("{}", encoded);

    // Private Key
    let mut vp = Vec::new();
    spk.write(&mut vp).expect("Can't write private key");
    //println!("Len {:?}", vp.len());
    
    let c_d: Vec<u5> = vp.to_base32();
    let encoded_pk = Bech32::new(pk_prefix.into(), c_d).expect("bech32 failed").to_string();
    //println!("{}", encoded_pk);
    

    //println!("Hello, {}, {}!", encoded, encoded_pk);
    return (encoded.to_string(), encoded_pk.to_string());
}