mod integration {
    use zcash_lib::keys::transparent::{Network, PrivateKey};

    #[test]
    fn test_address_generation_mainnet() {
        let sk = PrivateKey::generate();
        let address = sk.public_key().to_address(Network::Mainnet);
        assert!(address.starts_with("t1"));
    }

    #[test]
    fn test_address_generation_testnet() {
        let sk = PrivateKey::generate();
        let address = sk.public_key().to_address(Network::Testnet);
        assert!(address.starts_with("tm"));
    }

    #[test]
    fn test_address_generation_regtest() {
        let sk = PrivateKey::generate();
        let address = sk.public_key().to_address(Network::Regtest);
        assert!(address.starts_with("tm"));
    }
}
