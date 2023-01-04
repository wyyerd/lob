use crate::error::Error;
use crate::model::*;
use reqwest::multipart::{Form, Part};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::mem;
use serde_json::json;

pub static API_VERSION: &'static str = "2020-02-11";

const NO_QUERY: Option<&'static str> = None;

#[derive(Clone)]
pub struct Client {
    inner: reqwest::Client,
    api_key: String,
}

impl Client {
    pub fn new<S: Into<String>>(api_key: S) -> Client {
        Client {
            inner: reqwest::Client::new(),
            api_key: api_key.into(),
        }
    }

    pub async fn create_address(&self, address: NewAddress) -> Result<Address, Error> {
        self.post("https://api.lob.com/v1/addresses", &NO_QUERY, &address)
            .await
    }

    pub async fn get_address(&self, id: &str) -> Result<Address, Error> {
        self.get(
            &format!("https://api.lob.com/v1/addresses/{}", id),
            &NO_QUERY,
        )
            .await
    }

    pub async fn delete_address(&self, id: &str) -> Result<Delete, Error> {
        self.delete(&format!("https://api.lob.com/v1/addresses/{}", id))
            .await
    }

    pub async fn list_addresses(
        &self,
        options: Option<ListAddressesOptions>,
    ) -> Result<ListResponse<Address>, Error> {
        self.get("https://api.lob.com/v1/addresses/", &options)
            .await
    }

    pub async fn verify_us_address<A: VerifyAddress>(
        &self,
        address: A,
        options: Option<VerifyAddressOptions>,
    ) -> Result<UsVerification, Error> {
        match address.into_input() {
            AddressVerificationInput::Flat(address) => {
                self.post(
                    "https://api.lob.com/v1/us_verifications",
                    &options,
                    &("address", address),
                )
                    .await
            }
            AddressVerificationInput::Components(components) => {
                self.post(
                    "https://api.lob.com/v1/us_verifications",
                    &options,
                    &components,
                )
                    .await
            }
        }
    }

    pub async fn autocomplete_address<S: Into<String>>(
        &self,
        address_prefix: S,
        options: Option<AutocompleteAddressOptions>,
    ) -> Result<UsAutocompletion, Error> {
        let mut request = self.inner.post("https://api.lob.com/v1/us_autocompletions");
        if let Some(true) = options.as_ref().and_then(|o| o.only_valid_addresses) {
            request = request.query(&[("valid_addresses", "true")]);
        }
        let geo_ip = options
            .as_ref()
            .and_then(|o| o.geo_ip_sort.as_ref())
            .map(|ip| ip.to_string());
        if let Some(geo_ip) = geo_ip {
            request = request.header("X-Forwarded-For", geo_ip);
        }
        let options = AutocompleteAddressOptionsQuery::new(address_prefix, options);
        self.make_request(request.json(&options)).await
    }

    pub async fn us_zip_lookup<S: Into<String>>(&self, zip_code: S) -> Result<UsZipLookup, Error> {
        self.post(
            "https://api.lob.com/v1/us_zip_lookups",
            &NO_QUERY,
            &UsZipLookupBody {
                zip_code: zip_code.into(),
            },
        )
            .await
    }

    pub async fn verify_intl_address(
        &self,
        address: &InternationalVerificationInput,
    ) -> Result<InternationalVerification, Error> {
        self.post(
            "https://api.lob.com/v1/intl_verifications",
            &NO_QUERY,
            address,
        )
            .await
    }

    pub async fn create_postcard(&self, mut postcard: NewPostcard) -> Result<Postcard, Error> {
        let mut request = self.inner.post("https://api.lob.com/v1/postcards");
        if let FileInput::File { filename, data } = &mut postcard.front {
            let filename = mem::take(filename);
            let data = mem::take(data);
            request =
                request.multipart(Form::new().part("front", Part::bytes(data).file_name(filename)));
        }
        if let FileInput::File { filename, data } = &mut postcard.back {
            let filename = mem::take(filename);
            let data = mem::take(data);
            request =
                request.multipart(Form::new().part("back", Part::bytes(data).file_name(filename)));
        }
        self.make_request(request.json(&postcard)).await
    }

    pub async fn get_postcard(&self, postcard_id: &str) -> Result<Postcard, Error> {
        self.get(
            &format!("https://api.lob.com/v1/postcards/{}", postcard_id),
            &NO_QUERY,
        )
            .await
    }

    pub async fn cancel_postcard(&self, postcard_id: &str) -> Result<Delete, Error> {
        self.delete(&format!("https://api.lob.com/v1/postcards/{}", postcard_id))
            .await
    }

    pub async fn list_postcards(
        &self,
        options: Option<ListPostcardOptions>,
    ) -> Result<ListResponse<Postcard>, Error> {
        self.get("https://api.lob.com/v1/postcards", &options).await
    }

    pub async fn create_letter(&self, mut letter: NewLetter) -> Result<Letter, Error> {
        let mut request = self.inner.post("https://api.lob.com/v1/letters");
        if let FileInput::File { filename, data } = &mut letter.file {
            let filename = mem::take(filename);
            let data = mem::take(data);
            request =
                request.multipart(Form::new().part("file", Part::bytes(data).file_name(filename)));
        }
        self.make_request(request.json(&letter)).await
    }

    pub async fn get_letter(&self, letter_id: &str) -> Result<Letter, Error> {
        self.get(
            &format!("https://api.lob.com/v1/letters/{}", letter_id),
            &NO_QUERY,
        )
            .await
    }

    pub async fn cancel_letter(&self, letter_id: &str) -> Result<Delete, Error> {
        self.delete(&format!("https://api.lob.com/v1/letters/{}", letter_id))
            .await
    }

    pub async fn list_letters(
        &self,
        options: Option<ListLetterOptions>,
    ) -> Result<ListResponse<Letter>, Error> {
        self.get("https://api.lob.com/v1/letters", &options).await
    }

    pub async fn create_check(&self, mut check: NewCheck) -> Result<Check, Error> {
        if let Some(logo) = &check.logo {
            if !(logo.is_file() || logo.is_url()) {
                return Err(Error::bad_request("check bottom must be `File` or `URL`"));
            }
        }
        match (&check.message, &check.check_bottom) {
            (Some(_), None) | (None, Some(_)) => {}
            _ => {
                return Err(Error::bad_request(
                    "One, but not both of `check_bottom` and `message` must be set",
                ));
            }
        }
        let mut request = self.inner.post("https://api.lob.com/v1/checks");
        if let Some(FileInput::File { filename, data }) = &mut check.logo {
            let filename = mem::take(filename);
            let data = mem::take(data);
            request =
                request.multipart(Form::new().part("logo", Part::bytes(data).file_name(filename)));
        }
        if let Some(FileInput::File { filename, data }) = &mut check.check_bottom {
            let filename = mem::take(filename);
            let data = mem::take(data);
            request = request
                .multipart(Form::new().part("check_bottom", Part::bytes(data).file_name(filename)));
        }
        if let Some(FileInput::File { filename, data }) = &mut check.attachment {
            let filename = mem::take(filename);
            let data = mem::take(data);
            request = request
                .multipart(Form::new().part("attachment", Part::bytes(data).file_name(filename)));
        }
        self.make_request(request.json(&check)).await
    }

    pub async fn get_check(&self, check_id: &str) -> Result<Check, Error> {
        self.get(
            &format!("https://api.lob.com/v1/checks/{}", check_id),
            &NO_QUERY,
        )
            .await
    }

    pub async fn cancel_check(&self, check_id: &str) -> Result<Delete, Error> {
        self.delete(&format!("https://api.lob.com/v1/checks/{}", check_id))
            .await
    }

    pub async fn list_checks(
        &self,
        options: Option<ListCheckOptions>,
    ) -> Result<ListResponse<Check>, Error> {
        self.get("https://api.lob.com/v1/checks", &options).await
    }

    pub async fn create_bank_account(
        &self,
        bank_account: &NewBankAccount,
    ) -> Result<BankAccount, Error> {
        self.post(
            "https://api.lob.com/v1/bank_accounts",
            &NO_QUERY,
            &bank_account,
        )
            .await
    }

    pub async fn get_bank_account(&self, bank_account_id: &str) -> Result<BankAccount, Error> {
        self.get(
            &format!("https://api.lob.com/v1/bank_accounts/{}", bank_account_id),
            &NO_QUERY,
        )
            .await
    }

    pub async fn delete_bank_account(&self, bank_account_id: &str) -> Result<Delete, Error> {
        self.delete(&format!(
            "https://api.lob.com/v1/bank_accounts/{}",
            bank_account_id
        ))
            .await
    }

    pub async fn verify_bank_account(
        &self,
        bank_account_id: &str,
        amounts: [u32; 2],
    ) -> Result<BankAccount, Error> {
        self.post(
            &format!(
                "https://api.lob.com/v1/bank_accounts/{}/verify",
                bank_account_id
            ),
            &NO_QUERY,
            &json!({ "amounts": amounts }),
        )
            .await
    }

    pub async fn list_bank_accounts(
        &self,
        options: Option<ListBankAccountOptions>,
    ) -> Result<ListResponse<BankAccount>, Error> {
        self.get("https://api.lob.com/v1/bank_accounts/", &options)
            .await
    }

    async fn post<Q: Serialize, B: Serialize, R: DeserializeOwned + 'static>(
        &self,
        url: &str,
        query: &Option<Q>,
        body: &B,
    ) -> Result<R, Error> {
        let query = make_query_string(query)?;
        self.make_request(self.inner.post(&format!("{}{}", url, query)).json(body))
            .await
    }

    async fn get<Q: Serialize, R: DeserializeOwned + 'static>(
        &self,
        url: &str,
        query: &Option<Q>,
    ) -> Result<R, Error> {
        let query = make_query_string(query)?;
        self.make_request(self.inner.get(&format!("{}{}", url, query)))
            .await
    }

    async fn delete<R: DeserializeOwned + 'static>(&self, url: &str) -> Result<R, Error> {
        self.make_request(self.inner.delete(url)).await
    }

    async fn make_request<R: DeserializeOwned>(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<R, Error> {
        let response = request
            .basic_auth(&self.api_key, Option::<String>::None)
            .send()
            .await?;

        if response.status().is_success() {
            let response = response.json().await?;
            Ok(response)
        } else {
            let LobErrorResponse { error } = response.json().await?;
            Err(error.into())
        }
    }
}

fn make_query_string<S: Serialize>(options: &Option<S>) -> Result<String, Error> {
    if let Some(options) = options {
        let s = serde_qs::to_string(&options)?;
        Ok(format!("?{}", s))
    } else {
        Ok(String::new())
    }
}
