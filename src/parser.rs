use super::*;
pub mod request_header;
pub use self::request_header::{request_header, RequestHeader};
pub mod response_header;
pub use self::response_header::{response_header, ResponseHeader};
pub mod produce_request;
pub use self::produce_request::{produce_request, ProduceRequest};
pub mod produce_response;
pub use self::produce_response::{produce_response, ProduceResponse};
pub mod fetch_request;
pub use self::fetch_request::{fetch_request, FetchRequest};
pub mod fetch_response;
pub use self::fetch_response::{fetch_response, FetchResponse};
pub mod list_offsets_request;
pub use self::list_offsets_request::{list_offsets_request, ListOffsetsRequest};
pub mod list_offsets_response;
pub use self::list_offsets_response::{list_offsets_response, ListOffsetsResponse};
pub mod metadata_request;
pub use self::metadata_request::{metadata_request, MetadataRequest};
pub mod metadata_response;
pub use self::metadata_response::{metadata_response, MetadataResponse};
pub mod leader_and_isr_request;
pub use self::leader_and_isr_request::{leader_and_isr_request, LeaderAndIsrRequest};
pub mod leader_and_isr_response;
pub use self::leader_and_isr_response::{leader_and_isr_response, LeaderAndIsrResponse};
pub mod stop_replica_request;
pub use self::stop_replica_request::{stop_replica_request, StopReplicaRequest};
pub mod stop_replica_response;
pub use self::stop_replica_response::{stop_replica_response, StopReplicaResponse};
pub mod update_metadata_request;
pub use self::update_metadata_request::{update_metadata_request, UpdateMetadataRequest};
pub mod update_metadata_response;
pub use self::update_metadata_response::{update_metadata_response, UpdateMetadataResponse};
pub mod controlled_shutdown_request;
pub use self::controlled_shutdown_request::{
    controlled_shutdown_request, ControlledShutdownRequest,
};
pub mod controlled_shutdown_response;
pub use self::controlled_shutdown_response::{
    controlled_shutdown_response, ControlledShutdownResponse,
};
pub mod offset_commit_request;
pub use self::offset_commit_request::{offset_commit_request, OffsetCommitRequest};
pub mod offset_commit_response;
pub use self::offset_commit_response::{offset_commit_response, OffsetCommitResponse};
pub mod offset_fetch_request;
pub use self::offset_fetch_request::{offset_fetch_request, OffsetFetchRequest};
pub mod offset_fetch_response;
pub use self::offset_fetch_response::{offset_fetch_response, OffsetFetchResponse};
pub mod find_coordinator_request;
pub use self::find_coordinator_request::{find_coordinator_request, FindCoordinatorRequest};
pub mod find_coordinator_response;
pub use self::find_coordinator_response::{find_coordinator_response, FindCoordinatorResponse};
pub mod join_group_request;
pub use self::join_group_request::{join_group_request, JoinGroupRequest};
pub mod join_group_response;
pub use self::join_group_response::{join_group_response, JoinGroupResponse};
pub mod heartbeat_request;
pub use self::heartbeat_request::{heartbeat_request, HeartbeatRequest};
pub mod heartbeat_response;
pub use self::heartbeat_response::{heartbeat_response, HeartbeatResponse};
pub mod leave_group_request;
pub use self::leave_group_request::{leave_group_request, LeaveGroupRequest};
pub mod leave_group_response;
pub use self::leave_group_response::{leave_group_response, LeaveGroupResponse};
pub mod sync_group_request;
pub use self::sync_group_request::{sync_group_request, SyncGroupRequest};
pub mod sync_group_response;
pub use self::sync_group_response::{sync_group_response, SyncGroupResponse};
pub mod describe_groups_request;
pub use self::describe_groups_request::{describe_groups_request, DescribeGroupsRequest};
pub mod describe_groups_response;
pub use self::describe_groups_response::{describe_groups_response, DescribeGroupsResponse};
pub mod list_groups_request;
pub use self::list_groups_request::{list_groups_request, ListGroupsRequest};
pub mod list_groups_response;
pub use self::list_groups_response::{list_groups_response, ListGroupsResponse};
pub mod sasl_handshake_request;
pub use self::sasl_handshake_request::{sasl_handshake_request, SaslHandshakeRequest};
pub mod sasl_handshake_response;
pub use self::sasl_handshake_response::{sasl_handshake_response, SaslHandshakeResponse};
pub mod api_versions_request;
pub use self::api_versions_request::{api_versions_request, ApiVersionsRequest};
pub mod api_versions_response;
pub use self::api_versions_response::{api_versions_response, ApiVersionsResponse};
pub mod create_topics_request;
pub use self::create_topics_request::{create_topics_request, CreateTopicsRequest};
pub mod create_topics_response;
pub use self::create_topics_response::{create_topics_response, CreateTopicsResponse};
pub mod delete_topics_request;
pub use self::delete_topics_request::{delete_topics_request, DeleteTopicsRequest};
pub mod delete_topics_response;
pub use self::delete_topics_response::{delete_topics_response, DeleteTopicsResponse};
pub mod delete_records_request;
pub use self::delete_records_request::{delete_records_request, DeleteRecordsRequest};
pub mod delete_records_response;
pub use self::delete_records_response::{delete_records_response, DeleteRecordsResponse};
pub mod init_producer_id_request;
pub use self::init_producer_id_request::{init_producer_id_request, InitProducerIdRequest};
pub mod init_producer_id_response;
pub use self::init_producer_id_response::{init_producer_id_response, InitProducerIdResponse};
pub mod offset_for_leader_epoch_request;
pub use self::offset_for_leader_epoch_request::{
    offset_for_leader_epoch_request, OffsetForLeaderEpochRequest,
};
pub mod offset_for_leader_epoch_response;
pub use self::offset_for_leader_epoch_response::{
    offset_for_leader_epoch_response, OffsetForLeaderEpochResponse,
};
pub mod add_partitions_to_txn_request;
pub use self::add_partitions_to_txn_request::{
    add_partitions_to_txn_request, AddPartitionsToTxnRequest,
};
pub mod add_partitions_to_txn_response;
pub use self::add_partitions_to_txn_response::{
    add_partitions_to_txn_response, AddPartitionsToTxnResponse,
};
pub mod add_offsets_to_txn_request;
pub use self::add_offsets_to_txn_request::{add_offsets_to_txn_request, AddOffsetsToTxnRequest};
pub mod add_offsets_to_txn_response;
pub use self::add_offsets_to_txn_response::{add_offsets_to_txn_response, AddOffsetsToTxnResponse};
pub mod end_txn_request;
pub use self::end_txn_request::{end_txn_request, EndTxnRequest};
pub mod end_txn_response;
pub use self::end_txn_response::{end_txn_response, EndTxnResponse};
pub mod write_txn_markers_request;
pub use self::write_txn_markers_request::{write_txn_markers_request, WriteTxnMarkersRequest};
pub mod write_txn_markers_response;
pub use self::write_txn_markers_response::{write_txn_markers_response, WriteTxnMarkersResponse};
pub mod txn_offset_commit_request;
pub use self::txn_offset_commit_request::{txn_offset_commit_request, TxnOffsetCommitRequest};
pub mod txn_offset_commit_response;
pub use self::txn_offset_commit_response::{txn_offset_commit_response, TxnOffsetCommitResponse};
pub mod describe_acls_request;
pub use self::describe_acls_request::{describe_acls_request, DescribeAclsRequest};
pub mod describe_acls_response;
pub use self::describe_acls_response::{describe_acls_response, DescribeAclsResponse};
pub mod create_acls_request;
pub use self::create_acls_request::{create_acls_request, CreateAclsRequest};
pub mod create_acls_response;
pub use self::create_acls_response::{create_acls_response, CreateAclsResponse};
pub mod delete_acls_request;
pub use self::delete_acls_request::{delete_acls_request, DeleteAclsRequest};
pub mod delete_acls_response;
pub use self::delete_acls_response::{delete_acls_response, DeleteAclsResponse};
pub mod describe_configs_request;
pub use self::describe_configs_request::{describe_configs_request, DescribeConfigsRequest};
pub mod describe_configs_response;
pub use self::describe_configs_response::{describe_configs_response, DescribeConfigsResponse};
pub mod alter_configs_request;
pub use self::alter_configs_request::{alter_configs_request, AlterConfigsRequest};
pub mod alter_configs_response;
pub use self::alter_configs_response::{alter_configs_response, AlterConfigsResponse};
pub mod alter_replica_log_dirs_request;
pub use self::alter_replica_log_dirs_request::{
    alter_replica_log_dirs_request, AlterReplicaLogDirsRequest,
};
pub mod alter_replica_log_dirs_response;
pub use self::alter_replica_log_dirs_response::{
    alter_replica_log_dirs_response, AlterReplicaLogDirsResponse,
};
pub mod describe_log_dirs_request;
pub use self::describe_log_dirs_request::{describe_log_dirs_request, DescribeLogDirsRequest};
pub mod describe_log_dirs_response;
pub use self::describe_log_dirs_response::{describe_log_dirs_response, DescribeLogDirsResponse};
pub mod sasl_authenticate_request;
pub use self::sasl_authenticate_request::{sasl_authenticate_request, SaslAuthenticateRequest};
pub mod sasl_authenticate_response;
pub use self::sasl_authenticate_response::{sasl_authenticate_response, SaslAuthenticateResponse};
pub mod create_partitions_request;
pub use self::create_partitions_request::{create_partitions_request, CreatePartitionsRequest};
pub mod create_partitions_response;
pub use self::create_partitions_response::{create_partitions_response, CreatePartitionsResponse};
pub mod create_delegation_token_request;
pub use self::create_delegation_token_request::{
    create_delegation_token_request, CreateDelegationTokenRequest,
};
pub mod create_delegation_token_response;
pub use self::create_delegation_token_response::{
    create_delegation_token_response, CreateDelegationTokenResponse,
};
pub mod renew_delegation_token_request;
pub use self::renew_delegation_token_request::{
    renew_delegation_token_request, RenewDelegationTokenRequest,
};
pub mod renew_delegation_token_response;
pub use self::renew_delegation_token_response::{
    renew_delegation_token_response, RenewDelegationTokenResponse,
};
pub mod expire_delegation_token_request;
pub use self::expire_delegation_token_request::{
    expire_delegation_token_request, ExpireDelegationTokenRequest,
};
pub mod expire_delegation_token_response;
pub use self::expire_delegation_token_response::{
    expire_delegation_token_response, ExpireDelegationTokenResponse,
};
pub mod describe_delegation_token_request;
pub use self::describe_delegation_token_request::{
    describe_delegation_token_request, DescribeDelegationTokenRequest,
};
pub mod describe_delegation_token_response;
pub use self::describe_delegation_token_response::{
    describe_delegation_token_response, DescribeDelegationTokenResponse,
};
pub mod delete_groups_request;
pub use self::delete_groups_request::{delete_groups_request, DeleteGroupsRequest};
pub mod delete_groups_response;
pub use self::delete_groups_response::{delete_groups_response, DeleteGroupsResponse};
pub mod elect_preferred_leaders_request;
pub use self::elect_preferred_leaders_request::{
    elect_preferred_leaders_request, ElectPreferredLeadersRequest,
};
pub mod elect_preferred_leaders_response;
pub use self::elect_preferred_leaders_response::{
    elect_preferred_leaders_response, ElectPreferredLeadersResponse,
};
pub mod incremental_alter_configs_request;
pub use self::incremental_alter_configs_request::{
    incremental_alter_configs_request, IncrementalAlterConfigsRequest,
};
pub mod incremental_alter_configs_response;
pub use self::incremental_alter_configs_response::{
    incremental_alter_configs_response, IncrementalAlterConfigsResponse,
};
impl<I> Client<I>
where
    I: AsyncRead + AsyncWrite + std::marker::Unpin,
{
    pub async fn add_offsets_to_txn<'i>(
        &'i mut self,
        request: AddOffsetsToTxnRequest<'i>,
    ) -> io::Result<AddOffsetsToTxnResponse> {
        self.call(
            request,
            ApiKey::AddOffsetsToTxn,
            add_offsets_to_txn_request::VERSION,
            add_offsets_to_txn_response(),
        )
        .await
    }
    pub async fn add_partitions_to_txn<'i>(
        &'i mut self,
        request: AddPartitionsToTxnRequest<'i>,
    ) -> io::Result<AddPartitionsToTxnResponse<'i>> {
        self.call(
            request,
            ApiKey::AddPartitionsToTxn,
            add_partitions_to_txn_request::VERSION,
            add_partitions_to_txn_response(),
        )
        .await
    }
    pub async fn alter_configs<'i>(
        &'i mut self,
        request: AlterConfigsRequest<'i>,
    ) -> io::Result<AlterConfigsResponse<'i>> {
        self.call(
            request,
            ApiKey::AlterConfigs,
            alter_configs_request::VERSION,
            alter_configs_response(),
        )
        .await
    }
    pub async fn alter_replica_log_dirs<'i>(
        &'i mut self,
        request: AlterReplicaLogDirsRequest<'i>,
    ) -> io::Result<AlterReplicaLogDirsResponse<'i>> {
        self.call(
            request,
            ApiKey::AlterReplicaLogDirs,
            alter_replica_log_dirs_request::VERSION,
            alter_replica_log_dirs_response(),
        )
        .await
    }
    pub async fn api_versions<'i>(
        &'i mut self,
        request: ApiVersionsRequest,
    ) -> io::Result<ApiVersionsResponse> {
        self.call(
            request,
            ApiKey::ApiVersions,
            api_versions_request::VERSION,
            api_versions_response(),
        )
        .await
    }
    pub async fn controlled_shutdown<'i>(
        &'i mut self,
        request: ControlledShutdownRequest,
    ) -> io::Result<ControlledShutdownResponse<'i>> {
        self.call(
            request,
            ApiKey::ControlledShutdown,
            controlled_shutdown_request::VERSION,
            controlled_shutdown_response(),
        )
        .await
    }
    pub async fn create_acls<'i>(
        &'i mut self,
        request: CreateAclsRequest<'i>,
    ) -> io::Result<CreateAclsResponse<'i>> {
        self.call(
            request,
            ApiKey::CreateAcls,
            create_acls_request::VERSION,
            create_acls_response(),
        )
        .await
    }
    pub async fn create_delegation_token<'i>(
        &'i mut self,
        request: CreateDelegationTokenRequest<'i>,
    ) -> io::Result<CreateDelegationTokenResponse<'i>> {
        self.call(
            request,
            ApiKey::CreateDelegationToken,
            create_delegation_token_request::VERSION,
            create_delegation_token_response(),
        )
        .await
    }
    pub async fn create_partitions<'i>(
        &'i mut self,
        request: CreatePartitionsRequest<'i>,
    ) -> io::Result<CreatePartitionsResponse<'i>> {
        self.call(
            request,
            ApiKey::CreatePartitions,
            create_partitions_request::VERSION,
            create_partitions_response(),
        )
        .await
    }
    pub async fn create_topics<'i>(
        &'i mut self,
        request: CreateTopicsRequest<'i>,
    ) -> io::Result<CreateTopicsResponse<'i>> {
        self.call(
            request,
            ApiKey::CreateTopics,
            create_topics_request::VERSION,
            create_topics_response(),
        )
        .await
    }
    pub async fn delete_acls<'i>(
        &'i mut self,
        request: DeleteAclsRequest<'i>,
    ) -> io::Result<DeleteAclsResponse<'i>> {
        self.call(
            request,
            ApiKey::DeleteAcls,
            delete_acls_request::VERSION,
            delete_acls_response(),
        )
        .await
    }
    pub async fn delete_groups<'i>(
        &'i mut self,
        request: DeleteGroupsRequest<'i>,
    ) -> io::Result<DeleteGroupsResponse<'i>> {
        self.call(
            request,
            ApiKey::DeleteGroups,
            delete_groups_request::VERSION,
            delete_groups_response(),
        )
        .await
    }
    pub async fn delete_records<'i>(
        &'i mut self,
        request: DeleteRecordsRequest<'i>,
    ) -> io::Result<DeleteRecordsResponse<'i>> {
        self.call(
            request,
            ApiKey::DeleteRecords,
            delete_records_request::VERSION,
            delete_records_response(),
        )
        .await
    }
    pub async fn delete_topics<'i>(
        &'i mut self,
        request: DeleteTopicsRequest<'i>,
    ) -> io::Result<DeleteTopicsResponse<'i>> {
        self.call(
            request,
            ApiKey::DeleteTopics,
            delete_topics_request::VERSION,
            delete_topics_response(),
        )
        .await
    }
    pub async fn describe_acls<'i>(
        &'i mut self,
        request: DescribeAclsRequest<'i>,
    ) -> io::Result<DescribeAclsResponse<'i>> {
        self.call(
            request,
            ApiKey::DescribeAcls,
            describe_acls_request::VERSION,
            describe_acls_response(),
        )
        .await
    }
    pub async fn describe_configs<'i>(
        &'i mut self,
        request: DescribeConfigsRequest<'i>,
    ) -> io::Result<DescribeConfigsResponse<'i>> {
        self.call(
            request,
            ApiKey::DescribeConfigs,
            describe_configs_request::VERSION,
            describe_configs_response(),
        )
        .await
    }
    pub async fn describe_delegation_token<'i>(
        &'i mut self,
        request: DescribeDelegationTokenRequest<'i>,
    ) -> io::Result<DescribeDelegationTokenResponse<'i>> {
        self.call(
            request,
            ApiKey::DescribeDelegationToken,
            describe_delegation_token_request::VERSION,
            describe_delegation_token_response(),
        )
        .await
    }
    pub async fn describe_groups<'i>(
        &'i mut self,
        request: DescribeGroupsRequest<'i>,
    ) -> io::Result<DescribeGroupsResponse<'i>> {
        self.call(
            request,
            ApiKey::DescribeGroups,
            describe_groups_request::VERSION,
            describe_groups_response(),
        )
        .await
    }
    pub async fn describe_log_dirs<'i>(
        &'i mut self,
        request: DescribeLogDirsRequest<'i>,
    ) -> io::Result<DescribeLogDirsResponse<'i>> {
        self.call(
            request,
            ApiKey::DescribeLogDirs,
            describe_log_dirs_request::VERSION,
            describe_log_dirs_response(),
        )
        .await
    }
    pub async fn elect_preferred_leaders<'i>(
        &'i mut self,
        request: ElectPreferredLeadersRequest<'i>,
    ) -> io::Result<ElectPreferredLeadersResponse<'i>> {
        self.call(
            request,
            ApiKey::ElectPreferredLeaders,
            elect_preferred_leaders_request::VERSION,
            elect_preferred_leaders_response(),
        )
        .await
    }
    pub async fn end_txn<'i>(
        &'i mut self,
        request: EndTxnRequest<'i>,
    ) -> io::Result<EndTxnResponse> {
        self.call(
            request,
            ApiKey::EndTxn,
            end_txn_request::VERSION,
            end_txn_response(),
        )
        .await
    }
    pub async fn expire_delegation_token<'i>(
        &'i mut self,
        request: ExpireDelegationTokenRequest<'i>,
    ) -> io::Result<ExpireDelegationTokenResponse> {
        self.call(
            request,
            ApiKey::ExpireDelegationToken,
            expire_delegation_token_request::VERSION,
            expire_delegation_token_response(),
        )
        .await
    }
    pub async fn fetch<'i>(
        &'i mut self,
        request: FetchRequest<'i>,
    ) -> io::Result<FetchResponse<'i>> {
        self.call(
            request,
            ApiKey::Fetch,
            fetch_request::VERSION,
            fetch_response(),
        )
        .await
    }
    pub async fn find_coordinator<'i>(
        &'i mut self,
        request: FindCoordinatorRequest<'i>,
    ) -> io::Result<FindCoordinatorResponse<'i>> {
        self.call(
            request,
            ApiKey::FindCoordinator,
            find_coordinator_request::VERSION,
            find_coordinator_response(),
        )
        .await
    }
    pub async fn heartbeat<'i>(
        &'i mut self,
        request: HeartbeatRequest<'i>,
    ) -> io::Result<HeartbeatResponse> {
        self.call(
            request,
            ApiKey::Heartbeat,
            heartbeat_request::VERSION,
            heartbeat_response(),
        )
        .await
    }
    pub async fn incremental_alter_configs<'i>(
        &'i mut self,
        request: IncrementalAlterConfigsRequest<'i>,
    ) -> io::Result<IncrementalAlterConfigsResponse<'i>> {
        self.call(
            request,
            ApiKey::IncrementalAlterConfigs,
            incremental_alter_configs_request::VERSION,
            incremental_alter_configs_response(),
        )
        .await
    }
    pub async fn init_producer_id<'i>(
        &'i mut self,
        request: InitProducerIdRequest<'i>,
    ) -> io::Result<InitProducerIdResponse> {
        self.call(
            request,
            ApiKey::InitProducerId,
            init_producer_id_request::VERSION,
            init_producer_id_response(),
        )
        .await
    }
    pub async fn join_group<'i>(
        &'i mut self,
        request: JoinGroupRequest<'i>,
    ) -> io::Result<JoinGroupResponse<'i>> {
        self.call(
            request,
            ApiKey::JoinGroup,
            join_group_request::VERSION,
            join_group_response(),
        )
        .await
    }
    pub async fn leader_and_isr<'i>(
        &'i mut self,
        request: LeaderAndIsrRequest<'i>,
    ) -> io::Result<LeaderAndIsrResponse<'i>> {
        self.call(
            request,
            ApiKey::LeaderAndIsr,
            leader_and_isr_request::VERSION,
            leader_and_isr_response(),
        )
        .await
    }
    pub async fn leave_group<'i>(
        &'i mut self,
        request: LeaveGroupRequest<'i>,
    ) -> io::Result<LeaveGroupResponse> {
        self.call(
            request,
            ApiKey::LeaveGroup,
            leave_group_request::VERSION,
            leave_group_response(),
        )
        .await
    }
    pub async fn list_groups<'i>(
        &'i mut self,
        request: ListGroupsRequest,
    ) -> io::Result<ListGroupsResponse<'i>> {
        self.call(
            request,
            ApiKey::ListGroups,
            list_groups_request::VERSION,
            list_groups_response(),
        )
        .await
    }
    pub async fn list_offsets<'i>(
        &'i mut self,
        request: ListOffsetsRequest<'i>,
    ) -> io::Result<ListOffsetsResponse<'i>> {
        self.call(
            request,
            ApiKey::ListOffsets,
            list_offsets_request::VERSION,
            list_offsets_response(),
        )
        .await
    }
    pub async fn metadata<'i>(
        &'i mut self,
        request: MetadataRequest<'i>,
    ) -> io::Result<MetadataResponse<'i>> {
        self.call(
            request,
            ApiKey::Metadata,
            metadata_request::VERSION,
            metadata_response(),
        )
        .await
    }
    pub async fn offset_commit<'i>(
        &'i mut self,
        request: OffsetCommitRequest<'i>,
    ) -> io::Result<OffsetCommitResponse<'i>> {
        self.call(
            request,
            ApiKey::OffsetCommit,
            offset_commit_request::VERSION,
            offset_commit_response(),
        )
        .await
    }
    pub async fn offset_fetch<'i>(
        &'i mut self,
        request: OffsetFetchRequest<'i>,
    ) -> io::Result<OffsetFetchResponse<'i>> {
        self.call(
            request,
            ApiKey::OffsetFetch,
            offset_fetch_request::VERSION,
            offset_fetch_response(),
        )
        .await
    }
    pub async fn offset_for_leader_epoch<'i>(
        &'i mut self,
        request: OffsetForLeaderEpochRequest<'i>,
    ) -> io::Result<OffsetForLeaderEpochResponse<'i>> {
        self.call(
            request,
            ApiKey::OffsetForLeaderEpoch,
            offset_for_leader_epoch_request::VERSION,
            offset_for_leader_epoch_response(),
        )
        .await
    }
    pub async fn produce<'i>(
        &'i mut self,
        request: ProduceRequest<'i>,
    ) -> io::Result<ProduceResponse<'i>> {
        self.call(
            request,
            ApiKey::Produce,
            produce_request::VERSION,
            produce_response(),
        )
        .await
    }
    pub async fn renew_delegation_token<'i>(
        &'i mut self,
        request: RenewDelegationTokenRequest<'i>,
    ) -> io::Result<RenewDelegationTokenResponse> {
        self.call(
            request,
            ApiKey::RenewDelegationToken,
            renew_delegation_token_request::VERSION,
            renew_delegation_token_response(),
        )
        .await
    }
    pub async fn sasl_authenticate<'i>(
        &'i mut self,
        request: SaslAuthenticateRequest<'i>,
    ) -> io::Result<SaslAuthenticateResponse<'i>> {
        self.call(
            request,
            ApiKey::SaslAuthenticate,
            sasl_authenticate_request::VERSION,
            sasl_authenticate_response(),
        )
        .await
    }
    pub async fn sasl_handshake<'i>(
        &'i mut self,
        request: SaslHandshakeRequest<'i>,
    ) -> io::Result<SaslHandshakeResponse<'i>> {
        self.call(
            request,
            ApiKey::SaslHandshake,
            sasl_handshake_request::VERSION,
            sasl_handshake_response(),
        )
        .await
    }
    pub async fn stop_replica<'i>(
        &'i mut self,
        request: StopReplicaRequest<'i>,
    ) -> io::Result<StopReplicaResponse<'i>> {
        self.call(
            request,
            ApiKey::StopReplica,
            stop_replica_request::VERSION,
            stop_replica_response(),
        )
        .await
    }
    pub async fn sync_group<'i>(
        &'i mut self,
        request: SyncGroupRequest<'i>,
    ) -> io::Result<SyncGroupResponse<'i>> {
        self.call(
            request,
            ApiKey::SyncGroup,
            sync_group_request::VERSION,
            sync_group_response(),
        )
        .await
    }
    pub async fn txn_offset_commit<'i>(
        &'i mut self,
        request: TxnOffsetCommitRequest<'i>,
    ) -> io::Result<TxnOffsetCommitResponse<'i>> {
        self.call(
            request,
            ApiKey::TxnOffsetCommit,
            txn_offset_commit_request::VERSION,
            txn_offset_commit_response(),
        )
        .await
    }
    pub async fn update_metadata<'i>(
        &'i mut self,
        request: UpdateMetadataRequest<'i>,
    ) -> io::Result<UpdateMetadataResponse> {
        self.call(
            request,
            ApiKey::UpdateMetadata,
            update_metadata_request::VERSION,
            update_metadata_response(),
        )
        .await
    }
    pub async fn write_txn_markers<'i>(
        &'i mut self,
        request: WriteTxnMarkersRequest<'i>,
    ) -> io::Result<WriteTxnMarkersResponse<'i>> {
        self.call(
            request,
            ApiKey::WriteTxnMarkers,
            write_txn_markers_request::VERSION,
            write_txn_markers_response(),
        )
        .await
    }
}
