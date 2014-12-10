// Copyright 2014 Alexis Mousset. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! SMTP sendable email

/// Converts to an `Header`
pub trait SendableEmail {
    /// Converts to an `Header` struct
    fn from_address(&self) -> String;
    fn to_addresses(&self) -> Vec<String>;
    fn message(&self) -> String;
}