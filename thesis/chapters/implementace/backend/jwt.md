
### JWT (Json Web Token)

Pro autentizaci uživatele je použit json web token, ve kterém je uloženo id uživatele.

K realizaci autentizace pomocí json web tokenu je vytvořen JwtMiddleware.
Toto softwarové lepidlo slouží k extrakci dat z tokenu, co uživatel pošle na server v hlavičce požadavku.
Token extrahovaný z hlavičky je dále dekódován a ověřen.
U tokenu je ověřováno několik věcí, jako je datum expirace, podpis a datum, od kdy je platný.
Dále jsou také načteny role uživatele a vloženy do úložiště, kde je později zpracovatel požadavku může nalézt.

Při extrakci je nutné používat hlavičku "authorization," protože je výhradně používaná na autentizaci nebo autorizaci pomocí tokenů.

```{.rust .linenos}
let auth_header_value = req.headers().get(header::AUTHORIZATION).map(|v| v.clone());
```

: JWT extrakce hlavičky {#lst:jwt_header_value_extraction}

Dále je hlavička dekódována pomocí funkce decode_jwt, která vrátí dekódovaná data tokenu, nebo nevrátí nic.
Když nevrátí nic, tak to může znamenat, že uživatel poslal neplatný token, nebo není přihlášen.
Z pohledu serveru je to jedno, protože tak jako tak uživatel nesmí mít přístup k funkcím, které vyžadují přihlášení.

```{.rust .linenos}
fn decode_jwt(
    header_value: Option<&HeaderValue>,
    decoding_key: &DecodingKey,
    validation: &Validation,
) -> Option<Claims> {
    let Some(val) = header_value else {
        return None;
    };
    let Ok(val) = val.to_str() else {
        return None;
    };
    if !val.starts_with("Bearer ") {
        return None;
    }
    match jsonwebtoken::decode::<Claims>(&val[7..], decoding_key, validation) {
        Ok(data) => Some(data.claims),
        Err(_) => None,
    }
}
```

: JWT dekódování tokenu {#lst:jwt_decode_jwt_func}

Poté, co je token dekódován a jsou načteny role uživatele (pokud je přihlášen),
tak jsou uživatelská data, která jsou oddělena od dat specifických ke správě tokenu,
vložena do požadavkového úložiště.
Tato data mají také speciální funkce a strukturu pro detekci změny uživatelských dat.
Po vložení dat do požadavkového úložiště je zavolána další funkce v řetězci,
tato funkce může být další middleware nebo zpracovatel požadavku.

```{.rust .linenos}
let ext = Rc::new(RefCell::new(AuthDataInner {
    changed: false,
    data: claims.map(|c| c.data),
}));

req.extensions_mut().insert(ext.clone());

let mut res = svc.call(req).await?;
```

: JWT vložení dat do úložiště a zavolání další funkce {#lst:jwt_extension_insert_and_next_call}

Po zavolání další funkce v řetězci, je získána odpověď.
Tato odpověď je dále upravena, aby vracela token v hlavičce odpovědi.
Uživatelská data jsou zkontrolována, jestli byla změněna,
pokud ano, tak je vytvořen nový token,
pokud ne, tak se použije starý token nebo žádný token.
Nevracení tokenu v hlavičce je použito, jen když uživatel není přihlášen, nebo byl právě odhlášen,
při této možnosti není do odpovědi vkládána hlavička "authorization."

```{.rust .linenos}
let inner_data = ext.borrow();
if inner_data.changed {
    if let Some(user_data) = inner_data.data.as_ref() {
        let token = encode_jwt(user_data.to_owned(), &encoding_key, token_ttl);
        res.headers_mut().insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
        );
    } else {
        res.headers_mut().remove(header::AUTHORIZATION);
    }
} else if let Some(val) = auth_header_value {
    res.headers_mut().insert(header::AUTHORIZATION, val);
}
```

: JWT práce s tokenem v odpovědi {#lst:jwt_res_header}

Nový token je vytvořen pomocí funkce encode_jwt, která bere řadu parametrů.
Tato funkce vytvoří nový token z uživatelských dat pomocí klíče a délky platnosti tokenu v sekundách.

```{.rust .linenos}
fn encode_jwt(user_data: UserData, encoding_key: &EncodingKey, token_ttl: u64) -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let claims = Claims {
        exp: now.as_secs() + token_ttl,
        iat: now.as_secs(),
        nbf: now.as_secs(),
        data: user_data,
    };
    jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, encoding_key).unwrap()
}
```

: JWT vytvoření tokenu {#lst:jwt_encode_jwt}

