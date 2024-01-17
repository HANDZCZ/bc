
### Pomocné hašovací funkce

Pro registraci a přihlášení uživatele je použit hašovací algoritmus argon2.
Pro jednodušší použití byly vytvořeny pomocné hašovací funkce.

Jednou z těchto funkcí je funkce make_salt, která vytvoří unikátní sůl pro hashování.
Tato sůl je 128 znaků dlouhá a každý znak v této sekvenci může nabýt jedné ze 71 hodnot.
Jinak řečeno tento řetězec může nabýt $9.1426\cdot10^{236}$ unikátních hodnot.

```{.rust .linenos}
pub fn make_salt() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const SALT_LEN: usize = 128;
    let mut rng = rand::thread_rng();

    let salt: String = (0..SALT_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    salt
}
```

: Pomocná funkce make_salt {#lst:make_salt_func}

Další funkce jsou na ověřování a hašování.
Hašovací funkce z hesla a soli vytvoří hash o 32 bytech.
Ověřovací funkce ověřuje hash oproti zadanému heslu a soli a vrací jestli jsou stejné.

```{.rust .linenos}
pub fn make_hash(password: &str, salt: &str) -> [u8; 32] {
    argon2rs::argon2i_simple(password, salt)
}

pub fn verify_password(hash: &[u8; 32], salt: &str, password: &str) -> bool {
    make_hash(password, salt) == *hash
}
```

: Pomocné funkce make_hash a verify_password {#lst:make_hash_and_verify_password_func}

