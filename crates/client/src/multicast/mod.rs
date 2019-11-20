// Copyright 2015-2018 Benjamin Fry <benjaminfry@me.com>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! UDP protocol related components for DNS

mod mdns_client_connection;
use proto::multicast;

pub use self::mdns_client_connection::MdnsClientConnection;
pub use self::multicast::{MdnsClientStream, MdnsQueryType, MdnsStream, MDNS_IPV4, MDNS_IPV6};
