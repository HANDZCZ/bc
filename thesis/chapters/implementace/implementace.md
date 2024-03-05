
# Implementace

Následující text se bude zabývat tím jak byly implementovány jednotlivé části backendu.

``` {.include}
databaze/databaze.md
backend/backend.md
```

## Testování a ladění

Testování backendu je výhradně děláno přes unit testy.
Pro vytváření unit testu je ve frameworku actix-web vytvořeno plno pomocných funkcí.
Spolu s vlastními implementacemi různých testovacích a pomocných funkcí probíhá testování.
Vlastní implementace těchto funkcí mohou být nalezeny v souboru [tests.rs](https://github.com/HANDZCZ/bc/blob/main/backend/src/tests.rs) v hlavním zdrojovém adresáři.

```{.text .linenos}
┌─« bc/backend on  main
└─» cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.29s
     Running unittests src\main.rs (target\debug\deps\backend-176820a2e66a0de0.exe)

running 70 tests
test games::get::tests::test_bad_req ... ok
test games::get::tests::test_ok ... ok
test users::get_all::tests::test_ok ... ok
.
.
.
test tournaments::signed_up_teams::get::tests::test_ok ... ok
test tournaments::team_applications::get::tests::test_ok ... ok
test users::login::tests::test_ok ... ok

test result: ok. 70 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.33s
```

: Zkrácená ukázka výsledku běhu unit testů {#lst:backend_unit_tests_showcase}

Jedna z nejjednodušších implementací unit testů je v ukázce níže.

```{.rust .linenos}
#[cfg(test)]
mod tests {
    use actix_web::test;

    use super::*;
    use crate::tests::*;
    const URI: &str = "/users";

    #[actix_web::test]
    async fn test_ok() {
        let (app, rollbacker, _pool) = get_test_app().await;
        let (_auth_header, id) = new_user_insert_random(&app).await;

        let req = test::TestRequest::get()
            .uri(&format!("{}/{}", URI, id))
            .to_request();
        let resp = test::call_service(&app, req).await;
        rollbacker.rollback().await;
        assert_eq!(resp.status().as_u16(), 200);
    }

    #[actix_web::test]
    async fn test_bad_req() {
        let (app, rollbacker, _pool) = get_test_app().await;

        let req = test::TestRequest::get()
            .uri(&format!("{}/{}", URI, Uuid::new_v4()))
            .to_request();
        let resp = test::call_service(&app, req).await;
        rollbacker.rollback().await;
        assert_eq!(resp.status().as_u16(), 400);
    }
}
```

: Ukázka jedné z nejjednodušší implementací unit testů {#lst:backend_unit_tests_implementation_showcase}

\newpage

