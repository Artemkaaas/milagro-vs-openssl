extern crate time;
extern crate openssl;
extern crate milagro_crypto;

use self::openssl::bn::{BigNum, BigNumContext};
use self::milagro_crypto::ff::FF;
use std::collections::HashMap;


pub fn milagro_calc_teq(pk: &PublicKeyMilagro, a_prime: &FF, e: &FF, v: &FF, mtilde: &HashMap<String, FF>, m1tilde: &FF, m2tilde: &FF, unrevealed_attr_names: &Vec<String>) -> FF {
    let start_time = time::get_time();

    let mut rur = FF::from_hex("1", 64);

    for k in unrevealed_attr_names.iter() {
        let cur_r = pk.r.get(k).unwrap();
        let cur_m = mtilde.get(k).unwrap();

        rur = &rur * &FF::pow(&cur_r, &cur_m, &pk.n).set_size(64)
    }

    rur = &rur * &FF::pow(&pk.rms, &m1tilde, &pk.n).set_size(64);

    rur = &rur * &FF::pow(&pk.rctxt, &m2tilde, &pk.n).set_size(64);

    let mut result = FF::pow(&a_prime, &e, &pk.n).set_size(64) * &rur;

    result = &result * FF::pow(&pk.s, &v, &pk.n).set_size(64);

    let res = FF::modulus(&mut result, &pk.n.clone().set_size(64));

    let end_time = time::get_time();

    println!("{}", &res);
    println!("Milagro: {}", end_time - start_time);

    res
}


pub fn openssl_calc_teq(pk: &PublicKeyOpenSsl, a_prime: &BigNum, e: &BigNum, v: &BigNum, mtilde: &HashMap<String, BigNum>, m1tilde: &BigNum, m2tilde: &BigNum, unrevealed_attr_names: &Vec<String>) -> BigNum {
    let start_time = time::get_time();

    let mut rur = BigNum::from_hex_str("1").unwrap();
    let mut ctx = BigNumContext::new().unwrap();

    for k in unrevealed_attr_names.iter() {
        let cur_r = pk.r.get(k).unwrap();
        let cur_m = mtilde.get(k).unwrap();

        let mut a = BigNum::new().unwrap();
        a.mod_exp(&cur_r, &cur_m, &pk.n, &mut ctx);

        rur = &rur * &a;
    }

    let mut a = BigNum::new().unwrap();

    a.mod_exp(&pk.rms, &m1tilde, &pk.n, &mut ctx);
    rur = &rur * &a;

    a.mod_exp(&pk.rctxt, &m2tilde, &pk.n, &mut ctx);
    rur = &rur * &a;

    let mut result = BigNum::new().unwrap();

    a.mod_exp(&a_prime, &e, &pk.n, &mut ctx);
    result = &a * &rur;

    a.mod_exp(&pk.s, &v, &pk.n, &mut ctx);
    result = &a * &result;

    let mut tmp = BigNum::new().unwrap();

    tmp.nnmod(&result, &pk.n, &mut ctx);

    let end_time = time::get_time();

    println!("{:?}", &tmp.to_hex_str().unwrap());
    println!("OpenSSL: {}", end_time - start_time);

    tmp
}

fn main() {

    let milagro_proof = get_eq_proof_milagro();
    milagro_calc_teq(
        &get_pk_milagro(),
        &milagro_proof.a_prime,
        &milagro_proof.e,
        &milagro_proof.v,
        &milagro_proof.m,
        &milagro_proof.m1,
        &milagro_proof.m2,
        &vec!["sex".to_string(), "age".to_string(), "height".to_string()]
    );



    let openssl_proof = get_eq_proof_openssl();
    openssl_calc_teq(
        &get_pk_openssl(),
        &openssl_proof.a_prime,
        &openssl_proof.e,
        &openssl_proof.v,
        &openssl_proof.m,
        &openssl_proof.m1,
        &openssl_proof.m2,
        &vec!["sex".to_string(), "age".to_string(), "height".to_string()]
    );
}

pub struct PrimaryEqualProofMilagro {
    revealed_attr_names: Vec<String>,
    a_prime: FF,
    e: FF,
    v: FF,
    m: HashMap<String, FF>,
    m1: FF,
    m2: FF,
}

pub struct PrimaryEqualProofOpenSsl {
    revealed_attr_names: Vec<String>,
    a_prime: BigNum,
    e: BigNum,
    v: BigNum,
    m: HashMap<String, BigNum>,
    m1: BigNum,
    m2: BigNum,
}

pub struct PublicKeyMilagro {
    n: FF,
    s: FF,
    rms: FF,
    r: HashMap<String, FF>,
    rctxt: FF,
    z: FF
}

pub struct PublicKeyOpenSsl {
    n: BigNum,
    s: BigNum,
    rms: BigNum,
    r: HashMap<String, BigNum>,
    rctxt: BigNum,
    z: BigNum
}

pub fn get_pk_milagro() -> PublicKeyMilagro {
    let mut r = HashMap::new();
    r.insert("name".to_string(), FF::from_hex("1b8babcacb5182020e0c04b9b49bc8b94ce57d7902ff3375a406fa1b07aca6076c1ef637229f24c4f9ac5a1bbfc465d085236a09fcec797187b9eaeec97cb24905a618e9e868cd5addf8444735d8c7c50d510fdbff1123889cf8ca5576f3db67836fa412ddd225316d0176f9b8f380516f39477fac70e09c15dd37d689d1277c38989fae60013f6dc9dfd6942050d391c496666479f7473db6df0ae110be596721ca2d60dea6d3bd8615723667f996bfd7100a8013a59328494c11e14eb348cc191a6c71ab7a3ff51c3c4f4bda3a1a0e25e6704f687cd3f6fd3037182115e34e3651cd3ceeec830539c20064fefb28566d686794a22d73d389add8015046c3b64", 32));
    r.insert("height".to_string(), FF::from_hex("fd99f2f56efdb2ed83298b4c52abc7308a7b60bef2f7a0f48a958130b58c9ccea8b0f0d55333ca5be20716342edb86c447f7858e75e10462f3e02c63f603f00e19809275f719f0c215ae83db27cd074ea91e7dfa65539ccffcd7cb0e0d1bc93c36da12f7f66eba24ccc3fbb2c120809d75b7ade620c9939b48afc1b65012c501c07afdbac1d3d115d6a9514ea4e228f60db43b5784881ec815f3732c963f4f7ce1ea2f6884192b2e933edefdbd637204273bec122cd39591eb166ff9ef81abf57a0ec84ec2d2c8f9e46fba136bfc0a048a676717697430128336e32b1b89435ca357a73375b39fbb1b6c5ed1a274a52dc8162e8d821a362a0595b0daf19d871c", 32));
    r.insert("age".to_string(), FF::from_hex("2c2758078ebc8be4c0644af3742e5dd6b83c6c4452eb4ac0b539333fbbecf9950b175bdf331f453f533f82a9ee585754bd429ba0363489ea0ca8febc08ab27e726bd52aa855fcf35d49a299bd2ded122ea20bb2ee439de02566958a54aa6bb365d7f04be1366f4424b74128307d719df6f2f8671d426fe4dce066a216de4c83c83c2b1cf2f1e58592b6c7984d0bc263bd3335ecefaac860d4b4593ace338d35a6e8c46bae2e01c7300fa304f73995a42d4f2c15025d27ecb9a83f7400879ad08f575bf9519f2f0a18e08a7870832167a4332dac3645102ccfb84fe3d97c1ad95c86add560621f491201672ffae58574a2f0723fad486c344638cec43efd1d805", 32));
    r.insert("sex".to_string(), FF::from_hex("15f1332c6649bfeb77ac6e2bcfd114c4655ebfb9d2b1b2ae1c9c43d8a1f2e40e5a93896881725d789909126b11c16027baec8f1cc755691ba7b515cd73edb82f09c17972e6e6d38498e1433d54bf42709b41d7eaabc21b94b01a3457711b2ae539950c2d4abbc14d2c223ad8e36b3b87e06a2a42a9b79217fbd59a4d0e3ba1f2e545de58b7d42dfd4b6f65283edaf3534fe5dc61e35d4a0c1f461c9b0c58ed4c71686a23f3b2c9acef57161054973e6d685dcd378cb5ffd93da55c5ab92dda6f02355f8d0588fb56e0cda7688c7bc26d153de6115f64b642f924be2f0934472ba89937c43f81bdfb804bf6636ecd94301b91fe8c8f020e4985660179f1c9a8bf9", 32));

    PublicKeyMilagro {
        n: FF::from_hex("2f25fa4163e25083717f150b16229c2fa57d56dcb5048c522a64251a23e03342d0772b46cf47fc9e66e705b1910be1b968f45af1e6ff1d95fe1e319bec7dc34f60e33a664ab202afbe4098f09aa7f9e233a82ac0c1958b900c2a7b26f8dce2ebf774b35acfbf9a87f682498d5913d476300a558cb536c8facc9ef6f7a8a8925cfae17f913cddc9d4582c9dfca648ad074f88113f261839c4342fa8f33653979582d7d4c0716fe892371161712f8b77af31545420d6f075474f8847dddc2821b32125fba3807957e05218e655f5a8c8e7b96f1e1fff38b9177ef81e30ae3cacaaf64e5987c2fcfdc197ac2e43800acf3709ae381b0196f1a1bff153b6e93a4088d", 32),
        s: FF::from_hex("2964f0b14716e819da4fbaa6abbaeced56471ff859bcaf6dc7b7b8ff92981e23cd20ed67adaf3b3a57dc772aaa28716db2e8a885b529b06b2176fbd138e2bf54719727f011cd0dcdc80f45a352ea1c8f317857370e8948f8b73d2d06ddd900b3cb8527e269ee184f7d2ef9c50c4ba769f23a9ad7838fd48266ac834e0de30fef6bf55ba801aff3bc0b2e413e7089ff0896d4eb2b676da6a04ff7e7460371c4a7f55381c116baafdf23a4a4899ab9c769334704712feaa4fff24606af98c2502dc6e1fe075cc56076665a47bcfeb3c0657796e65e0c57fd59e9bf78f5d5ceeb10f109f76e87c44142562ef9ae9d444c9f05f8b39c919dbcf707ad1873f1c555f2b", 32),
        rms: FF::from_hex("5f13d1162a2e4242437428b697fd18643532ad38baab29de02083f91c4a860fe0929c53ba51820d42d79479a31d4d61fd523ae4f77dca41df8e7141e91436d410efa49066c423b36a263c5d200bcaf03e3f35bc9da9ba9af9f24c34e4732102d4789061ae0c2f2f4b486f4f7cef7f858edf5f6fe14a033af5e5c9b4ef50e3b2e79d72b135e3defd8167052e2785647442c86c294692bb299de22b9b0fe842d31441a138d8ad0ebd7c225ad3a1078109e1b53e59436c7bd8d28b4768cedba031db0b1600633c0806e875ea84a9d123c0efb3f3c1a4b69e39ae53e174a20c1e34e7e0ef5ffa877f2f27b39c2ecb87fb33c585f2c04849fde795599477b7af2f4a0", 32),
        r: r,
        rctxt: FF::from_hex("262faf605951ceaaafa3aeccf7a97fc987e36611f93bc51e8aae92f065c45e96ab181a089e5aca9136dd90ef63a7fbccdf60393de176fba0ba96e4c20d5095140d775eae7b1026c01c05e65ed299537114b3d421409c1ca987c958f772c5c35b06dcc13349eb382257f43785d243a3b617eee42656e50aad00b8448bdfd0a1fc3300f43dd2caf560ad66c11eea31736b2154ee6a11d9cc2a534ac94e1f37aad54991da48fefaa689b9f4a296e24cead07959adbb7e1c3dd326a482753b3d243a07825df2f00b24d5866af815c8f6ec16066e2cdcd7503ccb40b6c1cc2975b20d96db7b307b2eb1e9c4195cddb07339b11a9ba2655cefdc6d68fe76995e5ff860e", 32),
        z: FF::from_hex("1b4fcc44dd0a81647c21b739300277053595b1dc731649d19567911be768a9ba7de536b1ce4d455434b617acf432e2901a2977fbee1bff419abf5d67783612c0e4e75dd1626bf7d78af4c9ce1e22e6494d95f70e1a1965db63c5c188bb33ceace744eb10b4dcf7db10724429c973f9c407a9fb9ceadf5d6be26afd6cfafcf626710a05c05e176406b068fa627ce1ea4069228bf709173e17244050db1dead61119836e8db3b2c1581909d44eafa191441f7f60f58b503f4348f118e3ae7a1d0164204ab2311791dc7cd7e7d87ee885aec0a52aaf69f6a39e9fb127d9c26322f6e8cbf56b98f9e10a74a65f5d2180f3eaf7e62db6fbdaa074d68a657fcb23b563f", 32)
    }
}

pub fn get_pk_openssl() -> PublicKeyOpenSsl {
    let mut r = HashMap::new();
    r.insert("name".to_string(), BigNum::from_hex_str("1b8babcacb5182020e0c04b9b49bc8b94ce57d7902ff3375a406fa1b07aca6076c1ef637229f24c4f9ac5a1bbfc465d085236a09fcec797187b9eaeec97cb24905a618e9e868cd5addf8444735d8c7c50d510fdbff1123889cf8ca5576f3db67836fa412ddd225316d0176f9b8f380516f39477fac70e09c15dd37d689d1277c38989fae60013f6dc9dfd6942050d391c496666479f7473db6df0ae110be596721ca2d60dea6d3bd8615723667f996bfd7100a8013a59328494c11e14eb348cc191a6c71ab7a3ff51c3c4f4bda3a1a0e25e6704f687cd3f6fd3037182115e34e3651cd3ceeec830539c20064fefb28566d686794a22d73d389add8015046c3b64").unwrap());
    r.insert("height".to_string(), BigNum::from_hex_str("fd99f2f56efdb2ed83298b4c52abc7308a7b60bef2f7a0f48a958130b58c9ccea8b0f0d55333ca5be20716342edb86c447f7858e75e10462f3e02c63f603f00e19809275f719f0c215ae83db27cd074ea91e7dfa65539ccffcd7cb0e0d1bc93c36da12f7f66eba24ccc3fbb2c120809d75b7ade620c9939b48afc1b65012c501c07afdbac1d3d115d6a9514ea4e228f60db43b5784881ec815f3732c963f4f7ce1ea2f6884192b2e933edefdbd637204273bec122cd39591eb166ff9ef81abf57a0ec84ec2d2c8f9e46fba136bfc0a048a676717697430128336e32b1b89435ca357a73375b39fbb1b6c5ed1a274a52dc8162e8d821a362a0595b0daf19d871c").unwrap());
    r.insert("age".to_string(), BigNum::from_hex_str("2c2758078ebc8be4c0644af3742e5dd6b83c6c4452eb4ac0b539333fbbecf9950b175bdf331f453f533f82a9ee585754bd429ba0363489ea0ca8febc08ab27e726bd52aa855fcf35d49a299bd2ded122ea20bb2ee439de02566958a54aa6bb365d7f04be1366f4424b74128307d719df6f2f8671d426fe4dce066a216de4c83c83c2b1cf2f1e58592b6c7984d0bc263bd3335ecefaac860d4b4593ace338d35a6e8c46bae2e01c7300fa304f73995a42d4f2c15025d27ecb9a83f7400879ad08f575bf9519f2f0a18e08a7870832167a4332dac3645102ccfb84fe3d97c1ad95c86add560621f491201672ffae58574a2f0723fad486c344638cec43efd1d805").unwrap());
    r.insert("sex".to_string(), BigNum::from_hex_str("15f1332c6649bfeb77ac6e2bcfd114c4655ebfb9d2b1b2ae1c9c43d8a1f2e40e5a93896881725d789909126b11c16027baec8f1cc755691ba7b515cd73edb82f09c17972e6e6d38498e1433d54bf42709b41d7eaabc21b94b01a3457711b2ae539950c2d4abbc14d2c223ad8e36b3b87e06a2a42a9b79217fbd59a4d0e3ba1f2e545de58b7d42dfd4b6f65283edaf3534fe5dc61e35d4a0c1f461c9b0c58ed4c71686a23f3b2c9acef57161054973e6d685dcd378cb5ffd93da55c5ab92dda6f02355f8d0588fb56e0cda7688c7bc26d153de6115f64b642f924be2f0934472ba89937c43f81bdfb804bf6636ecd94301b91fe8c8f020e4985660179f1c9a8bf9").unwrap());

    PublicKeyOpenSsl {
        n: BigNum::from_hex_str("2f25fa4163e25083717f150b16229c2fa57d56dcb5048c522a64251a23e03342d0772b46cf47fc9e66e705b1910be1b968f45af1e6ff1d95fe1e319bec7dc34f60e33a664ab202afbe4098f09aa7f9e233a82ac0c1958b900c2a7b26f8dce2ebf774b35acfbf9a87f682498d5913d476300a558cb536c8facc9ef6f7a8a8925cfae17f913cddc9d4582c9dfca648ad074f88113f261839c4342fa8f33653979582d7d4c0716fe892371161712f8b77af31545420d6f075474f8847dddc2821b32125fba3807957e05218e655f5a8c8e7b96f1e1fff38b9177ef81e30ae3cacaaf64e5987c2fcfdc197ac2e43800acf3709ae381b0196f1a1bff153b6e93a4088d").unwrap(),
        s: BigNum::from_hex_str("2964f0b14716e819da4fbaa6abbaeced56471ff859bcaf6dc7b7b8ff92981e23cd20ed67adaf3b3a57dc772aaa28716db2e8a885b529b06b2176fbd138e2bf54719727f011cd0dcdc80f45a352ea1c8f317857370e8948f8b73d2d06ddd900b3cb8527e269ee184f7d2ef9c50c4ba769f23a9ad7838fd48266ac834e0de30fef6bf55ba801aff3bc0b2e413e7089ff0896d4eb2b676da6a04ff7e7460371c4a7f55381c116baafdf23a4a4899ab9c769334704712feaa4fff24606af98c2502dc6e1fe075cc56076665a47bcfeb3c0657796e65e0c57fd59e9bf78f5d5ceeb10f109f76e87c44142562ef9ae9d444c9f05f8b39c919dbcf707ad1873f1c555f2b").unwrap(),
        rms: BigNum::from_hex_str("5f13d1162a2e4242437428b697fd18643532ad38baab29de02083f91c4a860fe0929c53ba51820d42d79479a31d4d61fd523ae4f77dca41df8e7141e91436d410efa49066c423b36a263c5d200bcaf03e3f35bc9da9ba9af9f24c34e4732102d4789061ae0c2f2f4b486f4f7cef7f858edf5f6fe14a033af5e5c9b4ef50e3b2e79d72b135e3defd8167052e2785647442c86c294692bb299de22b9b0fe842d31441a138d8ad0ebd7c225ad3a1078109e1b53e59436c7bd8d28b4768cedba031db0b1600633c0806e875ea84a9d123c0efb3f3c1a4b69e39ae53e174a20c1e34e7e0ef5ffa877f2f27b39c2ecb87fb33c585f2c04849fde795599477b7af2f4a0").unwrap(),
        r: r,
        rctxt: BigNum::from_hex_str("262faf605951ceaaafa3aeccf7a97fc987e36611f93bc51e8aae92f065c45e96ab181a089e5aca9136dd90ef63a7fbccdf60393de176fba0ba96e4c20d5095140d775eae7b1026c01c05e65ed299537114b3d421409c1ca987c958f772c5c35b06dcc13349eb382257f43785d243a3b617eee42656e50aad00b8448bdfd0a1fc3300f43dd2caf560ad66c11eea31736b2154ee6a11d9cc2a534ac94e1f37aad54991da48fefaa689b9f4a296e24cead07959adbb7e1c3dd326a482753b3d243a07825df2f00b24d5866af815c8f6ec16066e2cdcd7503ccb40b6c1cc2975b20d96db7b307b2eb1e9c4195cddb07339b11a9ba2655cefdc6d68fe76995e5ff860e").unwrap(),
        z: BigNum::from_hex_str("1b4fcc44dd0a81647c21b739300277053595b1dc731649d19567911be768a9ba7de536b1ce4d455434b617acf432e2901a2977fbee1bff419abf5d67783612c0e4e75dd1626bf7d78af4c9ce1e22e6494d95f70e1a1965db63c5c188bb33ceace744eb10b4dcf7db10724429c973f9c407a9fb9ceadf5d6be26afd6cfafcf626710a05c05e176406b068fa627ce1ea4069228bf709173e17244050db1dead61119836e8db3b2c1581909d44eafa191441f7f60f58b503f4348f118e3ae7a1d0164204ab2311791dc7cd7e7d87ee885aec0a52aaf69f6a39e9fb127d9c26322f6e8cbf56b98f9e10a74a65f5d2180f3eaf7e62db6fbdaa074d68a657fcb23b563f").unwrap()
    }
}

pub fn get_eq_proof_milagro() -> PrimaryEqualProofMilagro {
    let mut mtilde = HashMap::new();
    mtilde.insert("height".to_string(), FF::from_hex("3549957cde04217c329b319d54a18952bb92a8d7c0c2c15c5e9fd7ed7b4eb83efbfad23a31f201d8fbae6f67152d29c9b61eb75a9855de5cad808ba339217508c09bf157452aaa6fc1fa", 32));
    mtilde.insert("age".to_string(), FF::from_hex("2f018037913ea0c38793ececf8a71401d963f74bd923a5bd0e782db3e9a3b9fbdf2c204cf61b4428323df6eb7426aa6a83273b040bc842b8283a030403f9c7312792641da150c70b1843", 32));
    mtilde.insert("sex".to_string(), FF::from_hex("1066d2487a484fd43ad9f809a7b694bcf98dd7e9876173674e2573dd1152860bfd98cbb827794acac2e1546114fe85b2efb09f6cfb6974694d18822df221d7bb560ad1c810c9a58f8a55", 32));

    PrimaryEqualProofMilagro {
        revealed_attr_names: vec!["name".to_string()],
        a_prime: FF::from_hex("270922f1a37a893c3af86085ec0f8b07bdc3b5383047230aeca99f01739addba1acc8b12b329fce92d4b350cad28831c2fa4c711094b815873977201026b57e82298f6d953de95e16192004160b2e97645202d8c33f0d054124550aeb15e7647f762194b32d6e101d1efdb5a0afbb5b31d4baf03acb6e9f5f023002a40edef89a556d1a22f60b5a614c0468caf9ddc055dd633aa4c9d963f20f16d5cbd76c0039dd9c34276e049e53cb296d0f16c86dbf9872c866746476fe5854ee0c6deef01659a4ff3c4d2fa2a17b68381363e4d026b83bb2c3668440163cc99e0eede7c543fed670908409cf103aaad5676153659aedcdbe974770d69bc0af2dee71155127", 32),
        e: FF::from_hex("d84b55034b84daf8d48b4fda6c4cb3a56f9d881d0af52076eeadb1f5f00e63de33be0276e743e85676df4af89261d69ec66d6498b09a382771", 32),
        v: FF::from_hex("e7eb4ae019dc16eb04911666a2aa5624b8aca9c44114072e76efa13eb3625ee7061db4980b40930225bf4cb3c9d13d475209fb79842aa64443babd661fe343bd1b1a7db44694a8f5f77157e8d84c4da3a0b7d91af85f7efa7a0824594b2ad2e7a1d9ad0d6e3143944a5cc7db8772f42d639b130d79724412b1b8c4a406cc071bc5900ef5a973d8d107396c020c8dbe184ef4a32f6b8a6a6f12e49ca3f7354af898f2ad720cba1801fde4d93412d0942c1f4e5528cc28772470ef0edb99aecff216a18118683b65accc13f5ee534fcfdb0d99a61828f4db68165585d9d7830550a478da853eb562c3f8d5760b89d80f3feb3b3cf3433994ed67f41ceabb12fe0dc3a4dcfe9d223a767c9d69474b23e19e39b07be97d735da3d03287d93cec1a89321491b94448c7d5492d00c944d481a32fb961544c7f2b2201363d538ab5daffe04bcf3fc05f72aa1747fa33a6cd38c2de59d4c5a994fafeacf94b9aabc0529a5b06e853cd762437605edd03fbda479dc8c46d6776968642b9d3ec48434d6", 32),
        m: mtilde,
        m1: FF::from_hex("a226a06ec4b728326c3e6bf5b9512e273cfdbb4c382495264d0bbb884cf4eacda1e70e97f4ff07f476f1b4c20bb64989055877733d28acf88dc2c03de4eb558ad1b23cc0d4e779442fba0fafe376bea7c08d933b586eb970019dcebccd2d2467d2b66bdbcd91a8e4b4c6991dc5beda0f85df04bf577eef786f6812c616816faf", 32),
        m2: FF::from_hex("1e2964ab5186204c36316b8bace9369e54cfebec08642da00387c91ccf4ee9c4df0e79d612aad73f3b82bfde72c8ecd8792c2f57a2c70cb30b3b5672a2f729c812250cfd5b599f82d21ccad4281147684f862a1ccf76fbc63828bde185af6b863508785ddc4241b09e6cff809aeadc25116285d2598a972680f1e841b9fbdd5", 32)
    }
}

pub fn get_eq_proof_openssl() -> PrimaryEqualProofOpenSsl {
    let mut mtilde = HashMap::new();
    mtilde.insert("height".to_string(), BigNum::from_hex_str("3549957cde04217c329b319d54a18952bb92a8d7c0c2c15c5e9fd7ed7b4eb83efbfad23a31f201d8fbae6f67152d29c9b61eb75a9855de5cad808ba339217508c09bf157452aaa6fc1fa").unwrap());
    mtilde.insert("age".to_string(), BigNum::from_hex_str("2f018037913ea0c38793ececf8a71401d963f74bd923a5bd0e782db3e9a3b9fbdf2c204cf61b4428323df6eb7426aa6a83273b040bc842b8283a030403f9c7312792641da150c70b1843").unwrap());
    mtilde.insert("sex".to_string(), BigNum::from_hex_str("1066d2487a484fd43ad9f809a7b694bcf98dd7e9876173674e2573dd1152860bfd98cbb827794acac2e1546114fe85b2efb09f6cfb6974694d18822df221d7bb560ad1c810c9a58f8a55").unwrap());

    PrimaryEqualProofOpenSsl {
        revealed_attr_names: vec!["name".to_string()],
        a_prime: BigNum::from_hex_str("270922f1a37a893c3af86085ec0f8b07bdc3b5383047230aeca99f01739addba1acc8b12b329fce92d4b350cad28831c2fa4c711094b815873977201026b57e82298f6d953de95e16192004160b2e97645202d8c33f0d054124550aeb15e7647f762194b32d6e101d1efdb5a0afbb5b31d4baf03acb6e9f5f023002a40edef89a556d1a22f60b5a614c0468caf9ddc055dd633aa4c9d963f20f16d5cbd76c0039dd9c34276e049e53cb296d0f16c86dbf9872c866746476fe5854ee0c6deef01659a4ff3c4d2fa2a17b68381363e4d026b83bb2c3668440163cc99e0eede7c543fed670908409cf103aaad5676153659aedcdbe974770d69bc0af2dee71155127").unwrap(),
        e: BigNum::from_hex_str("d84b55034b84daf8d48b4fda6c4cb3a56f9d881d0af52076eeadb1f5f00e63de33be0276e743e85676df4af89261d69ec66d6498b09a382771").unwrap(),
        v: BigNum::from_hex_str("e7eb4ae019dc16eb04911666a2aa5624b8aca9c44114072e76efa13eb3625ee7061db4980b40930225bf4cb3c9d13d475209fb79842aa64443babd661fe343bd1b1a7db44694a8f5f77157e8d84c4da3a0b7d91af85f7efa7a0824594b2ad2e7a1d9ad0d6e3143944a5cc7db8772f42d639b130d79724412b1b8c4a406cc071bc5900ef5a973d8d107396c020c8dbe184ef4a32f6b8a6a6f12e49ca3f7354af898f2ad720cba1801fde4d93412d0942c1f4e5528cc28772470ef0edb99aecff216a18118683b65accc13f5ee534fcfdb0d99a61828f4db68165585d9d7830550a478da853eb562c3f8d5760b89d80f3feb3b3cf3433994ed67f41ceabb12fe0dc3a4dcfe9d223a767c9d69474b23e19e39b07be97d735da3d03287d93cec1a89321491b94448c7d5492d00c944d481a32fb961544c7f2b2201363d538ab5daffe04bcf3fc05f72aa1747fa33a6cd38c2de59d4c5a994fafeacf94b9aabc0529a5b06e853cd762437605edd03fbda479dc8c46d6776968642b9d3ec48434d6").unwrap(),
        m: mtilde,
        m1: BigNum::from_hex_str("a226a06ec4b728326c3e6bf5b9512e273cfdbb4c382495264d0bbb884cf4eacda1e70e97f4ff07f476f1b4c20bb64989055877733d28acf88dc2c03de4eb558ad1b23cc0d4e779442fba0fafe376bea7c08d933b586eb970019dcebccd2d2467d2b66bdbcd91a8e4b4c6991dc5beda0f85df04bf577eef786f6812c616816faf").unwrap(),
        m2: BigNum::from_hex_str("1e2964ab5186204c36316b8bace9369e54cfebec08642da00387c91ccf4ee9c4df0e79d612aad73f3b82bfde72c8ecd8792c2f57a2c70cb30b3b5672a2f729c812250cfd5b599f82d21ccad4281147684f862a1ccf76fbc63828bde185af6b863508785ddc4241b09e6cff809aeadc25116285d2598a972680f1e841b9fbdd5").unwrap()
    }
}