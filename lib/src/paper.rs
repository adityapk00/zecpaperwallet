
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

fn gen_addresses_with_seed_as_json(testnet: bool, count: u32, seed: &[u8]) -> String {
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

// Generate a standard ZIP-32 address from the given seed at 32'/44'/0'/index
fn get_address(testnet: bool, seed: &[u8], index: u32) -> (String, String, String) {
    let addr_prefix = if testnet {"ztestsapling"} else {"zs"};
    let pk_prefix   = if testnet {"secret-extended-key-test"} else {"secret-extended-key-main"};
    let cointype    = if testnet {1} else {133};
    
    let spk: ExtendedSpendingKey = ExtendedSpendingKey::from_path(
            &ExtendedSpendingKey::master(seed),
            &[
                ChildIndex::Hardened(32),
                ChildIndex::Hardened(cointype),
                ChildIndex::Hardened(index)
            ],
        );
    let path = format!("HDSeed: {}, Path: m/32'/{}'/{}'", hex::encode(seed), cointype, index);

    let (_d, addr) = spk.default_address().expect("Cannot get result");

    // Address is encoded as a bech32 string
    let mut v = vec![0; 43];
    v.get_mut(..11).unwrap().copy_from_slice(&addr.diversifier.0);
    addr.pk_d.write(v.get_mut(11..).unwrap()).expect("Cannot write!");
    let checked_data: Vec<u5> = v.to_base32();
    let encoded = Bech32::new(addr_prefix.into(), checked_data).expect("bech32 failed").to_string();

    // Private Key is encoded as bech32 string
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
        assert!(j[0]["seed"].as_str().unwrap().contains("32'/1'/0'"));


        // Mainnet wallet
        let w = generate_wallet(false, 1);
        let j = json::parse(&w).unwrap();
        assert_eq!(j.len(), 1);
        assert!(j[0]["address"].as_str().unwrap().starts_with("zs"));
        assert!(j[0]["private_key"].as_str().unwrap().starts_with("secret-extended-key-main"));
        assert!(j[0]["seed"].as_str().unwrap().contains("32'/133'/0'"));

        // Check if all the addresses are the same
        let w = generate_wallet(true, 3);
        let j = json::parse(&w).unwrap();
        assert_eq!(j.len(), 3);
        let mut s = HashSet::new();
        for i in 0..3 {
            assert!(j[i]["address"].as_str().unwrap().starts_with("ztestsapling"));
            assert!(j[i]["seed"].as_str().unwrap().contains(format!("32'/1'/{}'", i).as_str()));

            s.insert(j[i]["address"].as_str().unwrap());
            s.insert(j[i]["private_key"].as_str().unwrap());
        }
        // There should be 3 + 3 distinct addresses and private keys
        assert_eq!(s.len(), 6);
    }

    // Test the address derivation against the test data.
    fn test_address_derivation(testdata: &str, testnet: bool) {
        use crate::paper::gen_addresses_with_seed_as_json;
        let td = json::parse(&testdata.replace("'", "\"")).unwrap();
        
        for i in td.members() {
            let seed = hex::decode(i["seed"].as_str().unwrap()).unwrap();
            let num  = i["num"].as_u32().unwrap();

            let addresses = gen_addresses_with_seed_as_json(testnet, num+1, &seed[0..32]);

            let j = json::parse(&addresses).unwrap();
            assert_eq!(j[num as usize]["address"], i["addr"]);
            assert_eq!(j[num as usize]["private_key"], i["pk"]);
        }
    }

    /*
        Test data was derived from zcashd. It cointains 3 sets of seeds, and for each seed, it contains 3 accounts that are derived for the testnet and mainnet. 
        We'll use the same seed and derive the same set of addresses here, and then make sure that both the address and private key matches up.

        To derive the test data, add something like this in test_wallet.cpp and run with
        ./src/zcash-gtest --gtest_filter=WalletTests.*

    ```
        void print_wallet(std::string seed, std::string pk, std::string addr, int num) {
            std::cout << "{'seed': '" << seed << "', 'pk': '" << pk << "', 'addr': '" << addr << "', 'num': " << num << "}," << std::endl;
        }

        void gen_addresses() {
            for (int i=0; i < 3; i++) {
                HDSeed seed = HDSeed::Random();
                for (int j=0; j < 3; j++) {
                    auto m = libzcash::SaplingExtendedSpendingKey::Master(seed);
                    auto xsk = m.Derive(32 | ZIP32_HARDENED_KEY_LIMIT)
                                .Derive(Params().BIP44CoinType() | ZIP32_HARDENED_KEY_LIMIT)
                                .Derive(j | ZIP32_HARDENED_KEY_LIMIT);

                    auto rawSeed = seed.RawSeed();
                    print_wallet(HexStr(rawSeed.begin(), rawSeed.end()), 
                                EncodeSpendingKey(xsk), EncodePaymentAddress(xsk.DefaultAddress()), j);
                }
            }
        }

        TEST(WalletTests, SaplingAddressTest) {
            SelectParams(CBaseChainParams::TESTNET);
            gen_addresses();
            
            SelectParams(CBaseChainParams::MAIN);
            gen_addresses();
        }
    ```
    */

    #[test]
    fn test_address_derivation_testnet() {
        let testdata = "[
            {'seed': 'ec310e73a6a428e70f9111c0c8b46a0f20435352700a8f0ccc7a02ff9dba21c1', 'pk': 'secret-extended-key-test1qv925grjqqqqpqrqkrdfagdnwsv6efchtt5kyergkeejzgw0jyf3mhu6ftvv3tf3qmflgzf8d77mlzvtfn9l456s0yj2qgef2qkgvge6l7meuh4ysjzqcyq5r2d7dqrqm4rg37xy0q773xmyayzdzmtzp582vqwxfhhmzccd3fgy6dyc8md8fw5phyqz38sraucytcdn45c8pzd5fgslry8s0sncvwcyasxauywej0k3kn3z9h8c2yuzn8d95pagkm3g48dt9rhhmkgh2jshh', 'addr': 'ztestsapling1amcnuqtqjqxa64xz4h9kcn4afffh6nrx6nfhulveexapk84tm6c9yrcdvhuzl0sh9g3eyx07qd0', 'num': 0},
            {'seed': 'ec310e73a6a428e70f9111c0c8b46a0f20435352700a8f0ccc7a02ff9dba21c1', 'pk': 'secret-extended-key-test1qv925grjqyqqpqrlhkxws3ptq2dcrluumvwrultgrmu83eckjw836ftghaal76nzfnjkt23q4782xhxlzqgk57hynzg80nvq5xwucvrmt2ewv50hym4spa5ja2perz09th2y27aa2quey38un0gsg8nr8td7aunxphuplzst0yvs257w24sudep2lsarmgeuyrlr59l9zw67072g8mtx4hy5wys2svtq7n68l9ne505vtcv7dlsxeh3h9uu549xd0ey5y47hnnl6q3sxx9w6c', 'addr': 'ztestsapling1ra4x3n357yhtykek3zx7zahw4t24095z7wr6w67a7y750vxk7mum0c7523mxcze9dr5gzhlc2jd', 'num': 1},
            {'seed': 'ec310e73a6a428e70f9111c0c8b46a0f20435352700a8f0ccc7a02ff9dba21c1', 'pk': 'secret-extended-key-test1qv925grjqgqqpqya3vq2uhkpr2yf7672cjrdlp6crtt0525xy7vcxsyw0cuwzq5c8k00azu4pxgalm4e2hcx0dqsylxgf2mhzaae2v947pz2uwr6z46q9zwcpvusf3v9dfzn9g7zxhxmtgwnt5ypxlj9e5fdcaz9xjxdpfgp0526g80vf9ym856xz9va480ssv4t38nqk3s7aa42pgx77urzjfq7m06t7lkcv9kjcjc6acghumyanezdduylpcagfdkew3yex7rwdzscaga8v', 'addr': 'ztestsapling19z33s3r7625cdh4xc8s5g85g6u3p0yzmkwy5plswcpt2cgh6vx4jp677z9j76a7esxldzjkfq9j', 'num': 2},
            {'seed': 'af534f57bd5df8f4a2f2750b65097d8804b9e7d01875428eb87eb59c2ab464a3', 'pk': 'secret-extended-key-test1qds00emdqqqqpqpugvy9k0u6zguk4gpxpts875ecjayvhlw9spz959ldk4xjcfjjzuhhujdr7yhyhfdmjv7ctccrx3tnumv8annx87kz97htymt0ttdq5yy379xr29wwcatwfs22qcfh9tk74j0x8nhqay9lqtu9xfn8rrgr4kj3wv94xuzz29q2kuzeq5tdefry2jr44m027nmx5h5n0dpxxvnd9qpvgjt5vy0zeh8dceys8wjuc459pshvrumpay7wxf82nfmvuesr3ta69', 'addr': 'ztestsapling16r7thvvfncl9ysltvufr8qaq74j9hg3frct3ad06hhgsgfa9e39uh0v2kl6tqdnrryftjcrg8gx', 'num': 0},
            {'seed': 'af534f57bd5df8f4a2f2750b65097d8804b9e7d01875428eb87eb59c2ab464a3', 'pk': 'secret-extended-key-test1qds00emdqyqqpq8st0767gujsrjf6eg3fkjwtadcwxgkdhlc3plyjhe6tdt9e2gnwzvpyn8j4p6uucm3fu930ctgk36y5e82vfth9m3qkpycshg6ph8qx3n30fl52h3dunr7wp6rrmsa7wg8m30mp4vktlsgwszr6pj675cdygtfqh27q33ztz8a84j5jz4felfxxw4e5cfc4kr2yga79s0cx24fcsz2ndxy7nwsz5rruh9z9lr5sne3wwp8g0mw3zsx5wrh3g56mxs4f3md7', 'addr': 'ztestsapling1lkgpqy52lee735prqpdke79plusx3thsl9xqxvg3y3czlgzp25j8lspjqxv9hxw2hfsrv02v3jw', 'num': 1},
            {'seed': 'af534f57bd5df8f4a2f2750b65097d8804b9e7d01875428eb87eb59c2ab464a3', 'pk': 'secret-extended-key-test1qds00emdqgqqpq9tcpdv6jenuentyquyglq5c2m7am7n56lqlat0wahypnzpyxkl2el3d7s4nj3f0q2gqjdt7weehf5zqm93n5uz5hnkfa8fqu7fm27qnkhrxz9leefauhmjm2fcc65x2q5tnz0e9hzzt84wk59yyckauzswy4r8zxv7cp4ktdejaqukxd9mykg4h985tekffnwllf23jw4e9zj670c7fe3ctjpu6rvm88xt0dqne60699xgf8chsxqcm8rmfd3u38qujvttu', 'addr': 'ztestsapling13v960lau60zqj2y3ty42wcvk6cpa6t4e37fueerqs8w2958stjf5vv035vs508cs47jmxfe3mjk', 'num': 2},
            {'seed': 'e5584c8f15cb22c1b876406457c5805620ff5a2d9dbb2c525b242a765a32f743', 'pk': 'secret-extended-key-test1qwcwln5yqqqqpqrxgmurndjq0yjl6gcvp97jly09rhe3sa4usduwg0g0ag3e79s0xkfmcjt8w3fkaa3rel2fkuvt83l2gvkvyjq4kmc5drwm0sthz7ps5pupyx6a97dafae0ap3nn2msvy3fguqm3f57cmxt9ghmd8eadcq8rskq66yx3u8lumkreek00x434jw7n9ulr24phjcpfg33mu6nue0adkggwhw0e847ppmvwh5hqt6tz3jsj8d73ux0rpne6u9xtmm7c9qgnynjk', 'addr': 'ztestsapling1l58ng46hw3p89h2vrjy4ymk4tvtgpv3nmvsetev0kt4hkew3aesvpx5ss4wwfj9700wkg4glfsq', 'num': 0},
            {'seed': 'e5584c8f15cb22c1b876406457c5805620ff5a2d9dbb2c525b242a765a32f743', 'pk': 'secret-extended-key-test1qwcwln5yqyqqpqqe6e0a5mhlhw7dewdqjdkuqspnmqmgs3s357rffruffjndkrcxxd0q0vsztc85kn2ljahchfkk28a2qnh40du6xn5vvljwztky9j2sfyu23hhf37n3g70fuydpdgr7rh8wm4gguz32xjvpu8n06pp5ypqf9u4s4hqcfqpxdw9khg8t0uzae62h9e42tqdjatu537a5s257lknl3z3euwkqz7se48wxtmzkd6p7hnz95hw585xpm854q5lckegcfyq4zyd46', 'addr': 'ztestsapling10z024sja27jjcxlykcxnpzswr3kzdyfpfgltwsrq2uclncdr3ruzpseqgqgk62uurz9agxq53wf', 'num': 1},
            {'seed': 'e5584c8f15cb22c1b876406457c5805620ff5a2d9dbb2c525b242a765a32f743', 'pk': 'secret-extended-key-test1qwcwln5yqgqqpqys9jav2aaflfucvpk4j2y5jh6sxjtwnhvtqw7j290ts2g4j95se4y0vktpkmv9c0xhf3tcvvad99854ergyqwmdhd2md0ejknwuwmq29p4zygwvjggu4gqp74mdwufs6fsaggegv2cw8ttzgaja2q3elcvh6tpyt4lmsf8kt62q7ts67xcty7vkfe8djgqpn9f7zy5yl8vchpzju7aprwfjrh3aznhqc5x9n2m30d27j8237menxw2ynt329m9xac4qxnee', 'addr': 'ztestsapling1rrs7k0fhkgfws5llvrpfnaujttn73850e87jhajm3r2yw6xj68rkuqz2epumd48d2nwu7eqwqrk', 'num': 2}
        ]";

        test_address_derivation(&testdata, true)
    }

    #[test]
    fn test_address_derivation_main() {
        let testdata = "[
            {'seed': '1d1106102d30f7b8052e171f256502113b78580aacdbf6077bf6fbe4ce1727cf', 'pk': 'secret-extended-key-main1qdg3eqcgqqqqpq92300a2t0anvhqd3a4n0utwuuvn54v4vxxkx299y52t9xxvq22t63lvq66pfyqa3eh7kvyzxx8x3jqarsjryqcd20ckajjkt3fayksy2dfxp3qvxtm3q3uy9x87ygvma05djj20k6rphxy5k4n02ymj9stjspvp5ms5n0grputfwmf5pcau0vzvcyayrmdfw099klvyt5jsxrrgrl562zyussn4ez2mv2yc0xcqa5ey9hxuywhvh9w4dh2kcz9dpqystnfe', 'addr': 'zs1arutqahvwd0w9086f68jmhxhrs2h5qz8mgmdmzedd4cuhukdddd296fuwem5d5q0v3m4gckqcr8', 'num': 0},
            {'seed': '1d1106102d30f7b8052e171f256502113b78580aacdbf6077bf6fbe4ce1727cf', 'pk': 'secret-extended-key-main1qdg3eqcgqyqqpqx3dv79dxsaud3h5cmjhpp46appe800s6rnedypwslmn3qxpwpzuemfhvs5xa0lzp6q774mx4wryhqr802n2h3lte584lzlh9uwae8sxt7uzxga4zx9yjnvv0pw4yp9h47tr4ncua39qfvg8kytpwmvd5gq8s4hz4scvql44rcd86jcyqe0w5972vr9ee74mfw53ymxkeyu5n22egunm502r83rldxvet6gptg9fa20hdefy4adtl7zqpjev8zhe9csx8t5z', 'addr': 'zs1593jpgvyejx3z0wn6dsx5dk2axusk34gya9tyzhu0klhyw9nfdygt0njvpzehtxmf2gkwhyxs0h', 'num': 1},
            {'seed': '1d1106102d30f7b8052e171f256502113b78580aacdbf6077bf6fbe4ce1727cf', 'pk': 'secret-extended-key-main1qdg3eqcgqgqqpqyhj8qgv9vczd47rnhsd488st0k95aq5eq309u0ay3z4s5vyy7lpqs4hp29yajnczezx4s72u2pm5dargqrccdpwm2mhytpf8rr8e8sjqsl2e808rtv9ek3xnt34tk73nwxuwrme3a3ux549keq5tahvwcx8reg70uxy4mdh0ue5qpdxjn7224zry507s0uwn9m55xtxup0hfcr3m09gm2sm2sjkj9vxul2mwfuc76xccpt7w2utd8j3ckj06j2zxgrvzh32', 'addr': 'zs1ykmmrdrggmv75mvpa5q65rs8sczwk4c4gnuvpl2qr8fjwvrcrm0z8zzltsjvhw5e7ulww2y2mx6', 'num': 2},
            {'seed': 'd7bf8927364aa487f46e796a22241a56f49d7184d0a00dcc05ad150b11295d68', 'pk': 'secret-extended-key-main1qwe2kfhwqqqqpq9zevq2e804h24jgxuq3snn5j8vtt0ynq07medvmjdyq6zrn9xx09vzevr7rmtf7hcghhwah70m64ekd6p9v96pje9yxadl27c73lzs2h8kchplq00sqxwehsmntw485k5jt275e43m3m2pq8qqqkhsh9gzyvd70qxdxx6p02wkm75t8zd9c0r46ah2fsddhmzsvm79csxngy4jffnp9c4v47sx7cmdw3e9f4tzap3kqr3sugjptwu236u7j6vr3tsr45eu4', 'addr': 'zs1gc263gyag6fg7d7xkwkfph6f5ytacprcvjq5uvhy8aqgxtldghvvfu530frkf9azq8mgqmkw70f', 'num': 0},
            {'seed': 'd7bf8927364aa487f46e796a22241a56f49d7184d0a00dcc05ad150b11295d68', 'pk': 'secret-extended-key-main1qwe2kfhwqyqqpqxat5yyzz6648n748kfmsl4cmx2shmcllwtuuq8yyl373f8qg7g8l99y0juuvkqkfnnw9mlmce43cwf33605jhpunatwa6fsww7tq2qzstsapg2hqggya2dsf08takedp8r7ap3wu2hcf8s99uk5p8002gpts26h7zlzfvcyedd0t8yalp4av7pmae9t79l5u5ft6dx2q7pupdh7adp74uma379mvtveh0e00dlyvk3tuur5sy0e4rwzznfvwkh22s4ug8qy', 'addr': 'zs1pfsavjtvqjxcycx8nw3nhtu6zrjasrujgslt58jsx5g9xna5xv5np6c6hn0wtta2nc5aks7ufsx', 'num': 1},
            {'seed': 'd7bf8927364aa487f46e796a22241a56f49d7184d0a00dcc05ad150b11295d68', 'pk': 'secret-extended-key-main1qwe2kfhwqgqqpq8xpn2kh9g22hhjm82n9h3m87mmelhknhnhyeh65csz932r5gcj0y3wp2mej9h5pasa9s0c4afvf3ehdnrul8zqmucsdvxhj9t3uzgqyergtfm6g0cplke0fxg5m3ew2zqh02a3y6ejwtrf2zukw4mj7aq93zyjr8hzfjfrc6vnzdju5wjdnq72kx2vxzcrndm5u7d6xctn066kgw9mkmzr6fm9t0nt7qnqzgnachmkdzgrsaev3mafpg5enqmwzgsjrzxjf', 'addr': 'zs1684wlkjdu40kgkghrx0pz4svfsrl30ra743yqc3fgg7yfmtuylq8j7m55hw3syznyad0qac792e', 'num': 2},
            {'seed': '4516297df8cab14852754bd1db0f49e3b3f7b265ca4eb352c408b9a39ed28824', 'pk': 'secret-extended-key-main1qd0fdrwdqqqqpqz63x3a22gcjpmuxr8g48lr2pe0cjzy3ye9qyfwxd5f4gkhh3ygdcc9vmpqxuncvqn2cgdznttqnwgy3r3jaakh4g7qs0m0hzvyecxspxd56pshwnt82rk3ya8hcmkutkk7nxw4mftkyrxfclvpdj6zwdcwjw2y9jnk57qnd3hlp3t2dau4zx8a4gelk6ndceecdujwax2cd920t09rvepelerhndwtsvwxn3ztugxl0rhll8awvn4jm6y6kjsfgsg4qmsf0', 'addr': 'zs182jnzpmd7e8m2ft658e05hek2cup74yxul5lu6vrujzw6tcc90rdzz3525a90pt54dp5gmup355', 'num': 0},
            {'seed': '4516297df8cab14852754bd1db0f49e3b3f7b265ca4eb352c408b9a39ed28824', 'pk': 'secret-extended-key-main1qd0fdrwdqyqqpqr3fmhq5x9hm3dnlthwt80x2z8hctzdg9rx8ldx5vhmayunp3uyfwg2d4svv7xurday9qg5y005cwp4yqpp0ev0aylmhfm50dnundyquj5pew83vmgscjte33dmeecr8g5n335rm89s4ky877ydqs5l6vcg0qjrta3zsl9x5vg4x0gj9y9yxvfvrm7xenxxtcxcnt2lceum5g5jpe7cwz0e75tqrcecwa78x83308jk50fl4f6hcl48z4xgtaw9rjsxwrgrs', 'addr': 'zs1mfemkzjakakk96l7akkyc657yttawj49te8shxq6endsxkyhxx2rpga5d7jqfupkdumcw5da6k8', 'num': 1},
            {'seed': '4516297df8cab14852754bd1db0f49e3b3f7b265ca4eb352c408b9a39ed28824', 'pk': 'secret-extended-key-main1qd0fdrwdqgqqpqzds0tgps6arrk8na9ut6f6szcmkewf5mujdjw7hkas7uefw9nn38aqyzl778xyn6kl8fyumcwzlsxq9eueuu3pajzvxn3ycch6uqhqf8emggsalht83q0aztxux4lpnm35qsu37cmkzu5f4za2rju5sdc8p3njrps75wmy49jdjnfqz0nysyyw5pc84e56u4a3f43er59zptnxhh7hh20spexft85d6qlr4qy3kax8afm20907h89ug6825exyqsgv48sm3', 'addr': 'zs1wdrep9ejtvz9t06q005x0n0w6m6g0935pm5tpx9e49dngy97dwv7w737qpte5luxnayr7zvq049', 'num': 2}
        ]";

        test_address_derivation(&testdata, false);
    }

}
