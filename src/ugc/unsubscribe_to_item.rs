use napi::{bindgen_prelude::*};
use napi::{Env, Result, Error};
use std::sync::Arc;
use std::sync::mpsc;

/// Async Task for getting workshop item details
pub struct UnsubscribeToItemTask {
    pub client: Arc<steamworks::Client>,
    pub item_id: f64,
}

impl UnsubscribeToItemTask {
    fn unsubscribe_item(&self) -> Result<Option<()>> {
        let ugc = self.client.ugc();
        let (tx, rx) = mpsc::channel();
        ugc.unsubscribe_item(steamworks::PublishedFileId(self.item_id as u64), move |result| {
            let _ = match result {
              Ok(b) => tx.send(Ok(b)),
              Err(e) => tx.send(Err(e.to_string()))
            };
        });
        match rx.recv() {
            Ok(Ok(_)) => Ok(Some(())),
            Ok(Err(e)) => Err(napi::Error::from_reason(format!(r"Steamworks: Failed to subscribe to item: {:?}", e))),
            Err(e) => Err(napi::Error::from_reason(format!(r"Steamworks: Failed to receive subscription result: {:?}", e))),
        }
    }        
}

#[napi]
impl Task for UnsubscribeToItemTask {
    type Output = Option<()>;
    type JsValue = Option<()>;

    fn compute(&mut self) -> Result<Self::Output> {
        self.unsubscribe_item()
    }

    fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
        Ok(output)
    }

    fn reject(&mut self, env: Env, error: Error) -> Result<Self::JsValue> {
        Err(error)
    }
}
