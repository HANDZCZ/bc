
### Autorizace

Autorizace slouží k zjištění práv uživatele.
Pro tento účel jsou implementovány dvě struktury, jedno makro a extrakční funkce pro autority.

Funkce pro extrakci autorit z požadavku slouží k jednoduché a jednotné kontrole oprávnění uživatele.
Tato funkce extrahuje jak role, tak stav uživatele.
Níže je popsaná implementace funkce pro extrakci autorit.

```{.rust .linenos}
pub async fn extract(req: &ServiceRequest) -> Result<HashSet<String>, Error> {
    // požadavkové úložiště si jen pučíme, nebudeme do něj zapisovat
    let extensions = req.extensions();
    // získání uživatelských údajů
    let auth_data = extensions
        .get::<Rc<RefCell<AuthDataInner>>>()
        .expect("You most likely forgot to add JwtMiddleware");
    // data si jen pučíme, bez možnosti editace
    let borrow = auth_data.borrow();
    // vnitřní data checeme jako referenci
    let data = borrow.data.as_ref();
    // vytvoříme kolekci autorit z rolí
    let mut set = data
        .map(|data| {
            data.roles
                .clone()
                .into_iter()
                .map(|e| format!("role::{e}"))
                .collect()
        })
        .or(Some(HashSet::new()))
        .unwrap();
    // do kolekce vložíme stav uživatele
    if data.is_some() {
        set.insert("state::LoggedIn".to_owned());
    } else {
        set.insert("state::LoggedOut".to_owned());
    }
    // vrátíme vytvořenou kolekci
    Ok(set)
}
```

: Implementace funkce pro extrakci autorit s popiskama {#lst:authorities_extract_function_with_comments}

Struktura LoggedInUser zajišťuje, že je uživatel přihlášen a vrací id uživatele.
Níže je popsaná implementace traitu FromRequest pro strukturu LoggedInUser.

```{.rust .linenos}
fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
    // požadavkové úložiště si jen pučíme, nebudeme do něj zapisovat
    let extensions = req.extensions();
    // získání uživatelských údajů
    let auth_data = extensions
        .get::<Rc<RefCell<AuthDataInner>>>()
        .expect("You most likely forgot to add JwtMiddleware");
    // data si jen pučíme, bez možnosti editace
    let borrow = auth_data.borrow();
    // vnitřní data checeme jako referenci
    let data = borrow.data.as_ref();

    // zkontrolujeme zda je uživatel přihlášen
    let res = if let Some(user_data) = data {
        // pokud ano získáme jeho id
        Ok(LoggedInUser { id: user_data.id })
    } else {
        // pokud ne tak vrátíme chybu
        let err = common::Error::new("not logged in");
        Err(JsonError::new(err, StatusCode::UNAUTHORIZED))
    };

    // vrátíme výsledek
    ready(res)
}
```

: Implementace LoggedInUser s popiskama {#lst:struct_loggedinuser_with_comments}

Další strukturou je struktura LoggedInUserWithAuthorities.
Tato struktura zajišťuje, že je uživatel přihlášen a vrací id uživatele a jeho autority.
Níže je popsaná implementace traitu FromRequest pro strukturu LoggedInUserWithAuthorities.

```{.rust .linenos}
fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
    // požadavkové úložiště si jen pučíme, nebudeme do něj zapisovat
    let extensions = req.extensions();
    // získání uživatelských údajů
    let auth_data = extensions
        .get::<Rc<RefCell<AuthDataInner>>>()
        .expect("You most likely forgot to add JwtMiddleware");
    // data si jen pučíme, bez možnosti editace
    let borrow = auth_data.borrow();
    // vnitřní data checeme jako referenci
    let data = borrow.data.as_ref();

    // zkontrolujeme zda je uživatel přihlášen
    let res = if let Some(user_data) = data {
        // získáme autority uživatele
        let authorities = extensions.get::<AuthDetails>().unwrap().clone();
        // vrátíme jeho id a autority
        Ok(LoggedInUserWithAuthorities {
            id: user_data.id,
            authorities,
        })
    } else {
        // pokud není přihlášen, tak vrátíme chybu
        let err = common::Error::new("not logged in");
        Err(JsonError::new(err, StatusCode::UNAUTHORIZED))
    };

    // vrátíme výsledek
    ready(res)
}
```

: Implementace LoggedInUserWithAuthorities s popiskama {#lst:struct_loggedinuserwithauthorities_with_comments}


Makro check_user_authority slouží ke kontrole jestli uživatel vlastní danou autoritu.
Pokud uživatel vlastní danou autoritu tak se nic nestane, ale pokud danou autoritu nevlastní tak se vrátí chyba.

```{.rust .linenos}
#[macro_export]
macro_rules! check_user_authority_macro {
    ($user:ident, $a:expr) => {
        use actix_web_grants::authorities::AuthoritiesCheck;
        if !$user.authorities.has_authority($a) {
            let err = crate::common::Error::new(format!("user is missing \"{}\" authority", $a));
            return crate::macros::resp_403_Forbidden_json!(err);
        }
    };
}

#[allow(unused_imports)]
pub use check_user_authority_macro as check_user_authority;
```

: Implementace check_user_authority makra {#lst:macro_check_user_authority}

