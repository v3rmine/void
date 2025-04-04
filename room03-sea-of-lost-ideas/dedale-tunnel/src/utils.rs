use std::future::Future;

use tokio::select;
use tokio_util::sync::CancellationToken;
use tonic::{Response, Status};

/// SOURCE: https://github.com/hyperium/tonic/blob/6a213e9485965db0628591e30577ed81cdaeaf2b/examples/src/cancellation/server.rs
pub async fn with_tonic_cancellation_handler<Reply, FRequest, FCancellation>(
    request_future: FRequest,
    cancellation_future: FCancellation,
) -> Result<Response<Reply>, Status>
where
    FRequest: Future<Output = Result<Response<Reply>, Status>> + Send + 'static,
    FCancellation: Future<Output = Result<Response<Reply>, Status>> + Send + 'static,
    Reply: Send + 'static,
{
    let token = CancellationToken::new();
    // Will call token.cancel() when the future is dropped, such as when the client cancels the request
    let _drop_guard = token.clone().drop_guard();
    let select_task = tokio::spawn(async move {
        // Can select on token cancellation on any cancellable future while handling the request,
        // allowing for custom cleanup code or monitoring
        select! {
            res = request_future => res,
            _ = token.cancelled() => cancellation_future.await,
        }
    });

    select_task.await.unwrap()
}
