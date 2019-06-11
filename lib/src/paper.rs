
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
        let (addr, pk, path) = get_address(testnet, &seed, i);
        ans.push(object!{
                "num"           => i,
                "address"       => addr,
                "private_key"   => pk,
                "seed"          => path
        }).unwrap(); 
    }      

    return json::stringify_pretty(ans, 2);
}

fn get_address(testnet: bool, seed: &[u8; 32], index: u32) -> (String, String, String) {
    let addr_prefix = if testnet {"ztestsapling"} else {"zs"};
    let pk_prefix   = if testnet {"secret-extended-key-test"} else {"secret-extended-key-main"};
    
    let spk: ExtendedSpendingKey = ExtendedSpendingKey::from_path(
            &ExtendedSpendingKey::master(seed),
            &[
                ChildIndex::Hardened(32),
                ChildIndex::Hardened(44),
                ChildIndex::Hardened(0),
                ChildIndex::from_index(index)
            ],
        );
    let path = format!("HDSeed: {}, Path: 32'/44'/0'/{}", hex::encode(seed), index);

    let (_d, addr) = spk.default_address().expect("Cannot get result");

    // Address    
    let mut v = vec![0; 43];
    v.get_mut(..11).unwrap().copy_from_slice(&addr.diversifier.0);
    addr.pk_d.write(v.get_mut(11..).unwrap()).expect("Cannot write!");


    let checked_data: Vec<u5> = v.to_base32();
    let encoded = Bech32::new(addr_prefix.into(), checked_data).expect("bech32 failed").to_string();

    // Private Key
    let mut vp = Vec::new();
    spk.write(&mut vp).expect("Can't write private key");
    
    let c_d: Vec<u5> = vp.to_base32();
    let encoded_pk = Bech32::new(pk_prefix.into(), c_d).expect("bech32 failed").to_string();

    return (encoded.to_string(), encoded_pk.to_string(), path);
}


#[cfg(test)]
mod tests {
    
    #[test]
    fn test_wallet_generation() {
        use crate::paper::generate_wallet;
        use std::collections::HashSet;
        
        // Testnet wallet
        let w = generate_wallet(true, 1);
        let j = json::parse(&w).unwrap();
        assert_eq!(j.len(), 1);
        assert!(j[0]["address"].as_str().unwrap().starts_with("ztestsapling"));
        assert!(j[0]["private_key"].as_str().unwrap().starts_with("secret-extended-key-test"));
        assert!(j[0]["seed"].as_str().unwrap().contains("32'/44'/0'/0"));


        // Mainnet wallet
        let w = generate_wallet(false, 1);
        let j = json::parse(&w).unwrap();
        assert_eq!(j.len(), 1);
        assert!(j[0]["address"].as_str().unwrap().starts_with("zs"));
        assert!(j[0]["private_key"].as_str().unwrap().starts_with("secret-extended-key-main"));
        assert!(j[0]["seed"].as_str().unwrap().contains("32'/44'/0'/0"));

        // Check if all the addresses are the same
        let w = generate_wallet(true, 3);
        let j = json::parse(&w).unwrap();
        assert_eq!(j.len(), 3);
        let mut s = HashSet::new();
        for i in 0..3 {
            assert!(j[i]["address"].as_str().unwrap().starts_with("ztestsapling"));
            assert!(j[i]["seed"].as_str().unwrap().contains(format!("32'/44'/0'/{}", i).as_str()));

            s.insert(j[i]["address"].as_str().unwrap());
            s.insert(j[i]["private_key"].as_str().unwrap());
        }
        // There should be 3 + 3 distinct addresses and private keys
        assert_eq!(s.len(), 6);
    }

}
