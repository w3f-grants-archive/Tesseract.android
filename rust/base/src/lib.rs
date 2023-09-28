//===------------ lib.rs --------------------------------------------===//
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

//#![feature(proc_macro_is_available)]
//#![feature(once_cell)]

#![feature(result_option_inspect)]

#[macro_use]
extern crate log;
extern crate android_log;

pub mod context;
pub mod error;

// #[cfg(feature = "client")]
// pub mod client;

// #[cfg(feature = "service")]
// pub mod service;