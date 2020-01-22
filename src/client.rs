use crate::model::*;
use crate::error::Error;
use serde::Serialize;
use serde::de::DeserializeOwned;

pub static API_VERSION: &'static str = "2019-06-01";

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
            api_key: api_key.into()
        }
    }

    pub async fn create_address(&self, address: &NewAddress) -> Result<Address, Error> {
        self.post("https://api.lob.com/v1/addresses", &NO_QUERY, &address).await
    }

    pub async fn get_address(&self, id: &str) -> Result<Address, Error> {
        self.get(&format!("http://api.lob.com/v1/addresses/{}", id), &NO_QUERY).await
    }

    pub async fn delete_address(&self, id: &str) -> Result<Delete, Error> {
        self.delete(&format!("http://api.lob.com/v1/addresses/{}", id)).await
    }

    pub async fn list_addresses(&self, options: Option<ListAddressesOptions>) -> Result<ListResponse<Address>, Error> {
        self.get("https://api.lob.com/v1/addresses/", &options).await
    }

    pub async fn verify_us_address<A: VerifyAddress>(&self, address: A, options: Option<VerifyAddressOptions>) -> Result<UsVerification, Error> {
        match address.into_input() {
            AddressVerificationInput::Flat(address) => {
                self.post("https://api.lob.com/v1/us_verifications", &options, &("address", address)).await
            },
            AddressVerificationInput::Components(components) => {
                self.post("https://api.lob.com/v1/us_verifications", &options, &components).await
            }
        }
    }

    pub async fn autocomplete_address<S: Into<String>>(&self, address_prefix: S, options: Option<AutocompleteAddressOptions>) -> Result<UsAutocompletion, Error> {
        let mut request = self.inner.post("https://api.lob.com/v1/us_autocompletions");
        let geo_ip = options.as_ref()
            .and_then(|o| o.geo_ip_sort.as_ref())
            .map(|ip| ip.to_string());
        if let Some(geo_ip) = geo_ip {
            request = request.header("X-Forwarded-For", geo_ip);
        }
        let options = AutocompleteAddressOptionsQuery::new(address_prefix, options);
        self.make_request(request.query(&options)).await
    }

    async fn post<Q: Serialize, B: Serialize, R: DeserializeOwned + 'static>(&self, url: &str, query: &Option<Q>, body: &B) -> Result<R, Error> {
        self.make_request(self.inner.post(url).query(query).form(body)).await
    }

    async fn get<Q: Serialize, R: DeserializeOwned + 'static>(&self, url: &str, query: &Q) -> Result<R, Error> {
        self.make_request(self.inner.get(url).query(query)).await
    }

    async fn delete<R: DeserializeOwned + 'static>(&self, url: &str) -> Result<R, Error> {
        self.make_request(self.inner.delete(url)).await
    }

    //TODO identify and fix "hidden type for `impl Trait` captures lifetime that does not appear in bounds"
    //     error that appears when taking `body` by reference
    async fn make_request<R: DeserializeOwned>(&self, request: reqwest::RequestBuilder) -> Result<R, Error> {
        let response = request
            .basic_auth(&self.api_key, Option::<String>::None)
            .send()
            .await?;

        if response.status().is_success() {
            let response = response.json().await?;
            Ok(response)
        } else {
            let error: LobError = response.json().await?;
            Err(error.into())
        }
    }
}
