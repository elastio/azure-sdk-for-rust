use crate::prelude::*;
use azure_core::{
    headers::{BLOB_TYPE, COPY_SOURCE, PAGE_WRITE, SOURCE_RANGE, *},
    prelude::*,
    RequestId, Url,
};

operation! {
    PutPageFromUrl,
    client: BlobClient,
    ba512_range: BA512Range,
    source_url: Url,
    source_range: BA512Range,
    ?if_sequence_number: IfSequenceNumber,
    ?if_modified_since: IfModifiedSinceCondition,
    ?if_match: IfMatchCondition,
    ?lease_id: LeaseId
}

impl PutPageFromUrlBuilder {
    pub fn into_future(mut self) -> PutPageFromUrl {
        Box::pin(async move {
            let mut url = self.client.url()?;

            url.query_pairs_mut().append_pair("comp", "page");

            let mut headers = Headers::new();
            headers.insert(PAGE_WRITE, "update");
            headers.insert(BLOB_TYPE, "PageBlob");
            headers.insert(COPY_SOURCE, self.source_url.to_string());
            headers.insert(SOURCE_RANGE, self.source_range.to_string());
            headers.add(self.ba512_range);
            headers.add(self.if_sequence_number);
            headers.add(self.if_modified_since);
            headers.add(self.if_match);
            headers.add(self.lease_id);

            let mut request =
                BlobClient::finalize_request(url, azure_core::Method::Put, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            PutPageFromUrlResponse::from_headers(response.headers())
        })
    }
}

// The upload endpoint of a managed disk answers with no `ETag`, `Last-Modified` or sequence
// number, so only the request ID is read.
azure_storage::response_from_headers!(PutPageFromUrlResponse,
    request_id_from_headers => request_id: RequestId
);
