use futures_util::stream;
use tonic::transport::Channel;
use tonic::Request;

use crate::core::core_api_client::CoreApiClient;
use crate::core::*;
use crate::mobility::*;

pub struct Client {
    inner: CoreApiClient<Channel>,
}

impl Client {
    pub async fn connect(core_addr: &'static str) -> Result<Self, tonic::transport::Error> {
        Ok(Client {
            inner: CoreApiClient::connect(core_addr).await?,
        })
    }

    pub fn inner(&mut self) -> &mut CoreApiClient<Channel> {
        &mut self.inner
    }

    pub async fn get_sessions(
        &mut self,
    ) -> Result<Vec<crate::core::SessionSummary>, tonic::Status> {
        let request = tonic::Request::new(GetSessionsRequest {});

        Ok(self
            .inner()
            .get_sessions(request)
            .await?
            .into_inner()
            .sessions)
    }

    pub async fn get_session(
        &mut self,
        session_id: i32,
    ) -> Result<Option<crate::core::Session>, tonic::Status> {
        let request = tonic::Request::new(GetSessionRequest { session_id });
        Ok(self
            .inner()
            .get_session(request)
            .await?
            .into_inner()
            .session)
    }

    pub async fn get_node(
        &mut self,
        session_id: i32,
        node_id: i32,
    ) -> Result<crate::core::GetNodeResponse, tonic::Status> {
        let request = tonic::Request::new(GetNodeRequest {
            session_id,
            node_id,
        });
        Ok(self.inner().get_node(request).await?.into_inner())
    }
    pub async fn edit_node(
        &mut self,
        session_id: i32,
        node_id: i32,
        position: Option<crate::core::Position>,
        icon: String,
        source: String,
        geo: Option<crate::core::Geo>,
    ) -> Result<crate::core::EditNodeResponse, tonic::Status> {
        let request = tonic::Request::new(EditNodeRequest {
            session_id,
            node_id,
            position,
            icon,
            source,
            geo,
        });
        Ok(self.inner().edit_node(request).await?.into_inner())
    }

    pub async fn create_session(
        &mut self,
        session_id: i32,
    ) -> Result<crate::core::CreateSessionResponse, tonic::Status> {
        let request = tonic::Request::new(CreateSessionRequest { session_id });
        Ok(self.inner().create_session(request).await?.into_inner())
    }
    pub async fn stop_session(
        &mut self,
        session_id: i32,
    ) -> Result<crate::core::StopSessionResponse, tonic::Status> {
        let request = tonic::Request::new(StopSessionRequest { session_id });
        Ok(self.inner().stop_session(request).await?.into_inner())
    }
    pub async fn open_xml(
        &mut self,
        data: String,
        start: bool,
        file: String,
    ) -> Result<crate::core::OpenXmlResponse, tonic::Status> {
        let request = tonic::Request::new(OpenXmlRequest { data, start, file });
        Ok(self.inner().open_xml(request).await?.into_inner())
    }
    pub async fn save_xml(
        &mut self,
        session_id: i32,
    ) -> Result<crate::core::SaveXmlResponse, tonic::Status> {
        let request = tonic::Request::new(SaveXmlRequest { session_id });
        Ok(self.inner().save_xml(request).await?.into_inner())
    }

    pub async fn execute_script(
        &mut self,
        script: String,
    ) -> Result<crate::core::ExecuteScriptResponse, tonic::Status> {
        let request = tonic::Request::new(ExecuteScriptRequest { script });
        Ok(self.inner().execute_script(request).await?.into_inner())
    }

    pub async fn node_command(
        &mut self,
        session_id: i32,
        node_id: i32,
        command: String,
        wait: bool,
        shell: bool,
    ) -> Result<crate::core::NodeCommandResponse, tonic::Status> {
        let request = tonic::Request::new(NodeCommandRequest {
            session_id,
            node_id,
            command,
            wait,
            shell,
        });
        Ok(self.inner().node_command(request).await?.into_inner())
    }
    pub async fn move_nodes(
        &mut self,
        moves: Vec<MoveNodesRequest>,
    ) -> Result<crate::core::MoveNodesResponse, tonic::Status> {
        /*let request = Request::new(MoveNodesRequest {
            session_id,
            node_id,
            source,
            move_type,
        });*/
        Ok(self
            .inner()
            .move_nodes(stream::iter(moves))
            .await?
            .into_inner())
    }
    pub async fn mobility_action(
        &mut self,
        session_id: i32,
        node_id: i32,
        action: crate::mobility::mobility_action::Enum,
    ) -> Result<MobilityActionResponse, tonic::Status> {
        let request = Request::new(MobilityActionRequest {
            session_id,
            node_id,
            action: action as i32,
        });
        Ok(self.inner().mobility_action(request).await?.into_inner())
    }
}
