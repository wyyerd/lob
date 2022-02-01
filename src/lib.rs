mod client;
mod error;
pub mod model;

pub use self::client::{Client, API_VERSION};
pub use self::error::Error;

#[cfg(test)]
mod tests {
    use crate::{model::*, Client};
    use chrono::{Duration, Utc};
    use rand::distributions::Alphanumeric;
    use rand::Rng;
    use std::collections::BTreeMap;
    use tokio_test::block_on;

    #[test]
    fn addresses() {
        block_on(async {
            let client = client();
            let address = client
                .create_address(NewAddress {
                    description: Some("a description!".to_owned()),
                    name: Some("Wyyerd Central".to_owned()),
                    company: Some("Wyyerd Group".to_owned()),
                    phone: Some("555-555-5555".to_owned()),
                    email: Some("wyyerd@example.com".to_owned()),
                    address_line1: "5600 Arapahoe Ave. STE 200".to_owned(),
                    address_line2: None,
                    address_city: Some("Boulder".to_owned()),
                    address_state: Some("CO".to_owned()),
                    address_zip: Some("80304".to_owned()),
                    address_country: Some("US".to_owned()),
                    metadata: Some(rand_key()),
                })
                .await
                .unwrap();

            let addresses = client
                .list_addresses(Some(ListAddressesOptions {
                    metadata: Some(address.metadata.clone()),
                    ..ListAddressesOptions::default()
                }))
                .await
                .unwrap()
                .data;
            assert_eq!(addresses.len(), 1);

            let address_2 = addresses.into_iter().next().unwrap();
            assert_eq!(&address_2, &address);

            let delete = client.delete_address(&address.id).await.unwrap();
            assert_eq!(&delete.id, &address.id);
            assert!(&delete.deleted);

            let address_3 = client.get_address(&address.id).await.unwrap();
            assert_eq!(&address_3.id, &address_2.id);
            assert_eq!(&address_3.deleted, &Some(true));
        });
    }

    #[test]
    fn verify_us_address() {
        block_on(async {
            let _: UsVerification = client()
                .verify_us_address(
                    AddressVerificationComponents {
                        recipient: Some("Alice Person".to_owned()),
                        primary_line: "residential highrise".to_string(),
                        secondary_line: None,
                        urbanization: None,
                        city: Some("Boulder".to_owned()),
                        state: Some("CO".to_owned()),
                        zip_code: Some("80303".to_owned()),
                    },
                    Some(VerifyAddressOptions {
                        case: Some(Case::Upper),
                    }),
                )
                .await
                .unwrap();
        })
    }

    #[test]
    fn autocomplete_address() {
        block_on(async {
            // TODO verify w/ prod creds
            let completions = client()
                .autocomplete_address(
                    "1 s",
                    Some(AutocompleteAddressOptions {
                        city: Some("Boulder".into()),
                        state: Some("CO".into()),
                        geo_ip_sort: Some("2607:a780:b00:1:55ee:4ce7:8819:b8d0".parse().unwrap()),
                        only_valid_addresses: None
                    }),
                )
                .await
                .unwrap();
            println!("{:?}", completions);
        })
    }

    #[test]
    fn us_zip_lookup() {
        block_on(async {
            let _: UsZipLookup = client().us_zip_lookup("80303").await.unwrap();
        })
    }

    #[test]
    fn verify_intl_address() {
        block_on(async {
            // TODO verify w/ prod creds
            let addr = client()
                .verify_intl_address(&InternationalVerificationInput {
                    recipient: Some("Elizabeth Windsor".into()),
                    primary_line: "Westminster".to_string(),
                    secondary_line: None,
                    city: Some("London".to_owned()),
                    state: None,
                    postal_code: Some("SW1A 1AA".to_owned()),
                    country: "GB".to_string(),
                })
                .await
                .unwrap();
            println!("{:?}", addr);
        })
    }

    #[test]
    fn postcards() {
        block_on(async {
            let client = client();
            let address = client
                .create_address(NewAddress {
                    description: Some("a description!".to_owned()),
                    name: Some("Wyyerd Central".to_owned()),
                    company: Some("Wyyerd Group".to_owned()),
                    phone: Some("555-555-5555".to_owned()),
                    email: Some("wyyerd@example.com".to_owned()),
                    address_line1: "5600 Arapahoe Ave. STE 200".to_owned(),
                    address_line2: None,
                    address_city: Some("Boulder".to_owned()),
                    address_state: Some("CO".to_owned()),
                    address_zip: Some("80304".to_owned()),
                    address_country: Some("US".to_owned()),
                    metadata: Some(rand_key()),
                })
                .await
                .unwrap();
            let postcard = client
                .create_postcard(NewPostcard {
                    description: Some("another description!".into()),
                    to: SendAddress::Components(SendAddressComponents {
                        name: "Jared Polis".to_string(),
                        address_line1: "200 E Colfax Ave".to_string(),
                        address_line2: None,
                        address_city: "Denver".to_string(),
                        address_state: "CO".to_string(),
                        address_zip: "80203".to_string(),
                        address_country: None
                    }),
                    from: Some(SendAddress::AddressId(address.id)),
                    front: FileInput::Html(include_str!("../postcard_front.html").into()),
                    back: FileInput::Html(include_str!("../postcard_back.html").into()),
                    // TODO: determine why we get a 503 when using files
                    // back: FileInput::File { filename: "postcard_back.png".into(), data: include_bytes!("../postcard_back.png").to_vec()},
                    merge_variables: None,
                    size: Some(PostcardSize::FourBySix),
                    mail_type: Some(MailType::UspsFirstClass),
                    send_date: None,
                    metadata: Some(rand_key()),
                })
                .await
                .unwrap();

            let postcards = client
                .list_postcards(Some(ListPostcardOptions {
                    metadata: Some(postcard.metadata.clone()),
                    ..ListPostcardOptions::default()
                }))
                .await
                .unwrap();
            assert_eq!(postcards.count, 1);
            assert_eq!(&postcards.data[0], &postcard);
            let delete = client.cancel_postcard(&postcard.id).await.unwrap();
            assert!(delete.deleted);
            assert_eq!(&delete.id, &postcard.id);
            let canceled = client.get_postcard(&postcard.id).await.unwrap();
            assert_eq!(&postcard.id, &canceled.id);
            assert_eq!(canceled.deleted, Some(true));
        })
    }

    #[test]
    fn letters() {
        block_on(async {
            let client = client();
            let address = client
                .create_address(NewAddress {
                    description: Some("a description!".to_owned()),
                    name: Some("Wyyerd Central".to_owned()),
                    company: Some("Wyyerd Group".to_owned()),
                    phone: Some("555-555-5555".to_owned()),
                    email: Some("wyyerd@example.com".to_owned()),
                    address_line1: "5600 Arapahoe Ave. STE 200".to_owned(),
                    address_line2: None,
                    address_city: Some("Boulder".to_owned()),
                    address_state: Some("CO".to_owned()),
                    address_zip: Some("80304".to_owned()),
                    address_country: Some("US".to_owned()),
                    metadata: Some(rand_key()),
                })
                .await
                .unwrap();
            let key = rand_key();
            let us_letter = client
                .create_letter(NewLetter {
                    description: Some("another description!".into()),
                    to: SendAddress::Components(SendAddressComponents {
                        name: "Jared Polis".to_string(),
                        address_line1: "200 E Colfax Ave".to_string(),
                        address_line2: None,
                        address_city: "Denver".to_string(),
                        address_state: "CO".to_string(),
                        address_zip: "80203".to_string(),
                        address_country: None,
                    }),
                    from: SendAddress::AddressId(address.id.clone()),
                    //                back: FileInput::File { filename: "postcard_back.png".into(), data: include_bytes!("../postcard_back.png").to_vec()},
                    color: false,
                    file: FileInput::Html(include_str!("../letter.html").into()),
                    merge_variables: None,
                    double_sided: None,
                    address_placement: None,
                    return_envelope: None,
                    custom_envelope: None,
                    mail_type: Some(MailType::UspsFirstClass),
                    extra_service: None,
                    send_date: None,
                    perforated_page: None,
                    metadata: Some(key.clone()),
                })
                .await
                .unwrap();

            let intl_letter = client
                .create_letter(NewLetter {
                    description: Some("another description!".into()),
                    to: SendAddress::Components(SendAddressComponents {
                        name: "Justin Trudeau".to_string(),
                        address_line1: "65 ARCHER DR".to_string(),
                        address_line2: None,
                        address_city: "RED DEER".to_string(),
                        address_state: "AB".to_string(),
                        address_zip: "T4R3B2".to_string(),
                        address_country: Some("CA".to_string()),
                    }),
                    from: SendAddress::AddressId(address.id),
                    //                back: FileInput::File { filename: "postcard_back.png".into(), data: include_bytes!("../postcard_back.png").to_vec()},
                    color: false,
                    file: FileInput::Html(include_str!("../letter.html").into()),
                    merge_variables: None,
                    double_sided: None,
                    address_placement: None,
                    return_envelope: None,
                    custom_envelope: None,
                    mail_type: Some(MailType::UspsFirstClass),
                    extra_service: None,
                    send_date: None,
                    perforated_page: None,
                    metadata: Some(key.clone()),
                })
                .await
                .unwrap();

            let letters = client
                .list_letters(Some(ListLetterOptions {
                    metadata: Some(key),
                    ..ListLetterOptions::default()
                }))
                .await
                .unwrap();
            assert_eq!(letters.count, 2);
            assert!(letters.data.iter().any(|l| l == &us_letter));
            assert!(letters.data.iter().any(|l| l == &intl_letter));
            let delete = client.cancel_letter(&us_letter.id).await.unwrap();
            assert!(delete.deleted);
            assert_eq!(&delete.id, &us_letter.id);
            let canceled = client.get_letter(&us_letter.id).await.unwrap();
            assert_eq!(&us_letter.id, &canceled.id);
            assert_eq!(canceled.deleted, Some(true));
        })
    }

    #[test]
    fn checks() {
        // TODO test with a real bank account
        block_on(async {
            let now = Utc::now().naive_local().date();
            let next_month = now + Duration::days(30);

            let client = client();
            let address = client
                .create_address(NewAddress {
                    description: Some("a description!".to_owned()),
                    name: Some("Wyyerd Central".to_owned()),
                    company: Some("Wyyerd Group".to_owned()),
                    phone: Some("555-555-5555".to_owned()),
                    email: Some("wyyerd@example.com".to_owned()),
                    address_line1: "5600 Arapahoe Ave. STE 200".to_owned(),
                    address_line2: None,
                    address_city: Some("Boulder".to_owned()),
                    address_state: Some("CO".to_owned()),
                    address_zip: Some("80304".to_owned()),
                    address_country: Some("US".to_owned()),
                    metadata: Some(rand_key()),
                })
                .await
                .unwrap();
            let check = client
                .create_check(NewCheck {
                    description: Some("another description!".into()),
                    to: SendAddress::Components(SendAddressComponents {
                        name: "Jared Polis".to_string(),
                        address_line1: "200 E Colfax Ave".to_string(),
                        address_line2: None,
                        address_city: "Denver".to_string(),
                        address_state: "CO".to_string(),
                        address_zip: "80203".to_string(),
                        address_country: None
                    }),
                    from: SendAddress::AddressId(address.id),
                    bank_account: "a_fake_bank_account".to_string(),
                    amount: 10.00.into(),
                    memo: None,
                    check_number: None,
                    logo: Some(FileInput::Url("https://s3-us-west-2.amazonaws.com/public.lob.com/logo/LobLogoLightSmall.png".to_owned())),
                    message: Some("a message!".into()),
                    check_bottom: None,
                    mail_type: None,
                    send_date: Some(next_month),
                    metadata: Some(rand_key()),
                    attachment: None
                })
                .await
                .unwrap();

            let checks = client
                .list_checks(Some(ListCheckOptions {
                    metadata: Some(check.metadata.clone()),
                    ..ListCheckOptions::default()
                }))
                .await
                .unwrap();
            assert_eq!(checks.count, 1);
            assert_eq!(&checks.data[0], &check);
            let delete = client.cancel_check(&check.id).await.unwrap();
            assert!(delete.deleted);
            assert_eq!(&delete.id, &check.id);
            let canceled = client.get_check(&check.id).await.unwrap();
            assert_eq!(&check.id, &canceled.id);
            assert_eq!(canceled.deleted, Some(true));
        })
    }

    #[test]
    fn bank_accounts() {
        block_on(async {
            let client = client();
            let bank_account = client
                .create_bank_account(&NewBankAccount {
                    description: None,
                    routing_number: "021000021".to_string(),
                    account_number: "12345678901234".to_string(),
                    account_type: AccountType::Company,
                    signatory: "me".to_string(),
                    metadata: rand_key(),
                })
                .await
                .unwrap();

            let bank_accounts = client
                .list_bank_accounts(Some(ListBankAccountOptions {
                    metadata: Some(bank_account.metadata.clone()),
                    ..ListBankAccountOptions::default()
                }))
                .await
                .unwrap();
            assert_eq!(bank_accounts.count, 1);
            assert_eq!(&bank_accounts.data[0], &bank_account);

            let delete = client.delete_bank_account(&bank_account.id).await.unwrap();
            assert!(delete.deleted);
            assert_eq!(&delete.id, &bank_account.id);

            let deleted = client.get_bank_account(&bank_account.id).await.unwrap();
            assert_eq!(deleted.deleted, Some(true));
            assert_eq!(&deleted.id, &bank_account.id);
        })
    }

    #[test]
    fn verify_bank_account() {
        block_on(async {
            let client = client();
            let bank_account = client
                .create_bank_account(&NewBankAccount {
                    description: None,
                    routing_number: "123456789".to_string(),
                    account_number: "12345678901234".to_string(),
                    account_type: AccountType::Company,
                    signatory: "me".to_string(),
                    metadata: rand_key(),
                })
                .await
                .unwrap();
            let verified = client
                .verify_bank_account(&bank_account.id, [5, 7])
                .await
                .unwrap();
            assert_eq!(&bank_account.id, &verified.id);
            assert!(verified.verified);
        })
    }

    fn client() -> Client {
        Client::new(dotenv::var("LOB_API_KEY").unwrap())
    }

    fn rand_key() -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        let key = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(15)
            .collect();
        map.insert("key".to_owned(), key);
        map
    }
}
