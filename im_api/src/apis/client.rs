use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient {
    _api: Box<dyn crate::apis::DefaultApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::Connect>(configuration: Configuration<C>) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            _api: Box::new(crate::apis::DefaultApiClient::new(rc.clone())),
        }
    }

    pub fn _api(&self) -> &dyn crate::apis::DefaultApi{
        self._api.as_ref()
    }

}
