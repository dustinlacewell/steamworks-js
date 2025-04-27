use napi::Error;
use napi::Result;
use std::sync::mpsc;
use steamworks::{ClientManager, QueryHandle};

use super::types::*;

// /// Async Task for getting workshop item details
pub fn fetch_details(query_handle: QueryHandle<ClientManager>) -> Result<Vec<WorkshopItemDetails>> {
  let (tx, rx) = mpsc::channel();

  query_handle.fetch(move |result| {
    let _ = match result {
      Ok(qresults) => tx.send(Ok(query_result_to_details(qresults))),
      Err(e) => tx.send(Err(e.to_string())),
    };
  });

  match rx.recv() {
    Ok(Ok(result)) => Ok(result),
    Ok(Err(e)) => Err(Error::from_reason(e)),
    Err(e) => Err(Error::from_reason(format!(
      r"Steamworks: Failed to receive workshop item details: {:?}",
      e
    ))),
  }
}

fn query_result_to_details(qresults: steamworks::QueryResults) -> Vec<WorkshopItemDetails> {
  qresults
    .iter()
    .flatten()
    .map(|qr| WorkshopItemDetails {
      item_id: qr.published_file_id.0 as f64,
      title: qr.title.clone(),
      description: qr.description.clone(),
      owner_id: qr.owner.raw() as f64,
      time_created: qr.time_created,
      time_updated: qr.time_updated,
      time_added_to_user_list: qr.time_added_to_user_list,
      visibility: qr.visibility as u32,
      banned: qr.banned,
      accepted_for_use: qr.accepted_for_use,
      tags_truncated: qr.tags_truncated,
      tags: qr.tags.clone(),
      file_size: qr.file_size,
      url: qr.url.clone(),
      num_upvotes: qr.num_upvotes,
      num_downvotes: qr.num_downvotes,
      score: qr.score as f64,
      num_children: qr.num_children,
    })
    .collect()
}
