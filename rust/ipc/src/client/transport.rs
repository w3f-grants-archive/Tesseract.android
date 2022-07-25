//===------------ transport.rs --------------------------------------------===//
//  Copyright 2022, Tesseract Systems, Inc.
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//===----------------------------------------------------------------------===//

use std::sync::Arc;

use async_trait::async_trait;

use jni::errors::Result;
use jni::objects::JObject;
use jni::JNIEnv;

use tesseract::client::transport::Status;
use tesseract::client::{Connection, Transport};

use interop_android::ContextedGlobal;

use super::connection::TransportIPCAndroidConnection;
use super::transceiver::Transceiver;

pub struct TransportIPCAndroid {
    transceiver: Result<ContextedGlobal>,
}

impl TransportIPCAndroid {
    pub fn new<'a: 'b, 'b>(env: &'b JNIEnv<'a>, application: JObject<'a>) -> Self {
        let transceiver = Transceiver::new(env, application)
            .and_then(|transceiver| ContextedGlobal::from_local(env, transceiver.into()));

        Self {
            transceiver: transceiver,
        }
    }

    pub const ID: &'static str = "IPC";
}

#[async_trait]
impl Transport for TransportIPCAndroid {
    fn id(&self) -> std::string::String {
        Self::ID.to_owned()
    }

    async fn status(self: Arc<Self>) -> Status {
        match &self.transceiver {
            Ok(_) => Status::Ready,
            Err(error) => {
                let reason = format!(
                    "IPC transport is unavailable due to some JNI error: {}",
                    error
                );
                Status::Unavailable(reason)
            }
        }
    }

    fn connect(&self) -> Box<dyn Connection + Sync + Send> {
        match &self.transceiver {
            Ok(transceiver) => Box::new(TransportIPCAndroidConnection::new(transceiver.clone())),
            Err(error) => {
                debug!("Android IPC transport returned that it's not ready. Please, don't ignore.\nDescription: {}", error);
                //panic is not ideal, but effective ;) in the future we might change the APIs so that the connect method generates an error
                panic!()
            }
        }
    }
}