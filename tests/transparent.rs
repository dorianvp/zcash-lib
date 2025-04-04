mod integration {
    use zcash_lib::keys::transparent::{Network, PrivateKey};

    #[test]
    fn test_address_generation_mainnet() {
        let sk = PrivateKey::generate();
        let address = sk
            .public_key()
            .to_dummy_p2sh_address()
            .to_base58check(Network::Mainnet);
        assert!(address.starts_with("t3"));
    }

    #[test]
    fn test_address_generation_testnet() {
        let sk = PrivateKey::generate();
        let address = sk
            .public_key()
            .to_dummy_p2sh_address()
            .to_base58check(Network::Testnet);
        assert!(address.starts_with("t2"));
    }

    #[test]
    #[ignore]
    fn test_address_generation_regtest() {
        todo!();
        // let sk = PrivateKey::generate();
        // let address = sk
        //     .public_key()
        //     .to_p2sh_address()
        //     .to_base58check(Network::Regtest);
        // assert!(address.starts_with("tm"));
    }
}
