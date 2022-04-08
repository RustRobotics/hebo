// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use protos::connection::connection_server::Connection;
use protos::connection::{
    BagIndex, BagRecordResult, BagReplayResult, BenchmarkReply, ConnectStateReply, ConnectionIndex,
    ConnectionInfo, GeneralConnectionReply, GeneralSubscriptionReply, GetConnectionReply,
    PublishReply, PublishRequest, StaticsReply, SubscribeRequest, SubscriptionIndex,
    SubscriptionInfo, SubscriptionInfos,
};

#[derive(Default)]
pub struct ConnectionManager {}

#[tonic::async_trait]
impl Connection for ConnectionManager {
    /// Create a new connection, uuid field is ignored.
    async fn new_connection(
        &self,
        _request: tonic::Request<ConnectionInfo>,
    ) -> Result<tonic::Response<GeneralConnectionReply>, tonic::Status> {
        todo!()
    }

    /// Get details of a connection.
    async fn get_connection(
        &self,
        _request: tonic::Request<ConnectionIndex>,
    ) -> Result<tonic::Response<GetConnectionReply>, tonic::Status> {
        todo!()
    }

    /// Edit a connection by overriding all fields.
    async fn edit_connection(
        &self,
        _request: tonic::Request<ConnectionInfo>,
    ) -> Result<tonic::Response<GeneralConnectionReply>, tonic::Status> {
        todo!()
    }

    /// Delete a connection by index.
    async fn delete_connection(
        &self,
        _request: tonic::Request<ConnectionIndex>,
    ) -> Result<tonic::Response<GeneralConnectionReply>, tonic::Status> {
        todo!()
    }

    /// Connect to broker.
    async fn connect_to(
        &self,
        _request: tonic::Request<ConnectionIndex>,
    ) -> Result<tonic::Response<ConnectStateReply>, tonic::Status> {
        todo!()
    }

    /// Get connection state.
    async fn get_connection_state(
        &self,
        _request: tonic::Request<ConnectionIndex>,
    ) -> Result<tonic::Response<ConnectStateReply>, tonic::Status> {
        todo!()
    }

    /// Disconnect from broker.
    async fn disconnect(
        &self,
        _request: tonic::Request<ConnectionIndex>,
    ) -> Result<tonic::Response<ConnectStateReply>, tonic::Status> {
        todo!()
    }

    /// Publish message to broker.
    async fn publish(
        &self,
        _request: tonic::Request<PublishRequest>,
    ) -> Result<tonic::Response<PublishReply>, tonic::Status> {
        todo!()
    }

    /// Add a new subscription info.
    async fn add_subscription(
        &self,
        _request: tonic::Request<SubscriptionInfo>,
    ) -> Result<tonic::Response<GeneralSubscriptionReply>, tonic::Status> {
        todo!()
    }

    /// List available subscriptions of this connection.
    async fn list_subscriptions(
        &self,
        _request: tonic::Request<ConnectionIndex>,
    ) -> Result<tonic::Response<SubscriptionInfos>, tonic::Status> {
        todo!()
    }

    /// Get detail of a subscription.
    async fn get_subscription(
        &self,
        _request: tonic::Request<SubscriptionIndex>,
    ) -> Result<tonic::Response<SubscriptionInfo>, tonic::Status> {
        todo!()
    }

    /// Edit a specific subscription.
    async fn edit_subscription(
        &self,
        _request: tonic::Request<SubscriptionInfo>,
    ) -> Result<tonic::Response<SubscriptionInfo>, tonic::Status> {
        todo!()
    }

    /// Subscript to or unsubscribe from the broker.
    async fn toggle_subscription(
        &self,
        _request: tonic::Request<SubscribeRequest>,
    ) -> Result<tonic::Response<SubscriptionInfo>, tonic::Status> {
        todo!()
    }

    /// Delete a subscription from this connection.
    async fn delete_subscription(
        &self,
        _request: tonic::Request<SubscriptionIndex>,
    ) -> Result<tonic::Response<GeneralSubscriptionReply>, tonic::Status> {
        todo!()
    }

    /// Save current subscribed messages to bag file.
    async fn bag_start_record(
        &self,
        _request: tonic::Request<BagIndex>,
    ) -> Result<tonic::Response<BagRecordResult>, tonic::Status> {
        todo!()
    }

    /// Stop recording messages of current connection.
    async fn bag_stop_record(
        &self,
        _request: tonic::Request<BagIndex>,
    ) -> Result<tonic::Response<BagRecordResult>, tonic::Status> {
        todo!()
    }

    /// Get detail of a bag file.
    async fn bag_get_info(
        &self,
        _request: tonic::Request<BagIndex>,
    ) -> Result<tonic::Response<BagRecordResult>, tonic::Status> {
        todo!()
    }

    /// Start replay of a bag file.
    async fn bag_start_replay(
        &self,
        _request: tonic::Request<BagIndex>,
    ) -> Result<tonic::Response<BagReplayResult>, tonic::Status> {
        todo!()
    }

    /// Stop replay of a bag file.
    async fn bag_stop_replay(
        &self,
        _request: tonic::Request<BagIndex>,
    ) -> Result<tonic::Response<BagReplayResult>, tonic::Status> {
        todo!()
    }

    /// Get broker statics.
    async fn get_statics(
        &self,
        _request: tonic::Request<ConnectionIndex>,
    ) -> Result<tonic::Response<StaticsReply>, tonic::Status> {
        todo!()
    }

    /// Run benchmark on this broker.
    async fn run_benchmark(
        &self,
        _request: tonic::Request<ConnectionIndex>,
    ) -> Result<tonic::Response<BenchmarkReply>, tonic::Status> {
        todo!()
    }

    /// Get benchmark state.
    async fn get_benchmark_reply(
        &self,
        _request: tonic::Request<ConnectionIndex>,
    ) -> Result<tonic::Response<BenchmarkReply>, tonic::Status> {
        todo!()
    }

    /// Stop a benchmark work.
    async fn stop_benchmark(
        &self,
        _request: tonic::Request<ConnectionIndex>,
    ) -> Result<tonic::Response<BenchmarkReply>, tonic::Status> {
        todo!()
    }
}
