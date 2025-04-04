use pb::{report_server::*, *};

use tonic::{Request, Response, Status, Streaming};
use tonic_health::server::HealthReporter;

pub mod pb {
    tonic::include_proto!("dedale.report");
}

#[derive(Debug)]
pub struct ReportServerService {
    health_reporter: HealthReporter,
}

impl ReportServerService {
    pub async fn new(mut health_reporter: HealthReporter) -> ReportServer<Self> {
        health_reporter
            .set_serving::<ReportServer<ReportServerService>>()
            .await;
        ReportServer::new(Self { health_reporter })
    }

    async fn set_serving(&mut self) {
        self.health_reporter
            .set_serving::<ReportServer<ReportServerService>>()
            .await;
    }

    async fn set_not_serving(&mut self) {
        self.health_reporter
            .set_not_serving::<ReportServer<ReportServerService>>()
            .await;
    }
}

#[tonic::async_trait]
impl Report for ReportServerService {
    #[tracing::instrument]
    async fn system(
        &self,
        request: Request<Streaming<SystemRequest>>,
    ) -> Result<Response<SystemResponse>, Status> {
        todo!()
    }

    #[tracing::instrument]
    async fn ping(&self, request: Request<PingRequest>) -> Result<Response<PongResponse>, Status> {
        Ok(Response::new(PongResponse {}))
    }
}
