pub struct SipParm {
    name: String,
    value: String,
}

pub enum UrlContext {
    UriInReqUrl(String),
    UriInFromToHdr(String),
    UriInContactHdr(String),
    UriInRoutingHdr(String),
    UriInOther(String)
}

pub struct Url {

}

impl Url {
    pub fn get_scheme(){

    }

    pub fn get_url() {

    }
    pub fn compare() {

    }

}

pub fn is_sip () {

}
pub fn is_sips() {
}
pub fn is_tel() {

}

pub struct SipUrl {
    user: String,
    password: String,
    host: String,
    port: i32,
    user_param: Option<String>,
    method_param: Option<String>,
    transport_param: Option<String>,
    ttl: Option<i32>,
    lr: Option<String>,
    maddr_param: Option<String>,
    other_param: Option<SipParm>,
    header_param: Option<SipParm>
}

// SIP 名称地址，通常出现在 From、To 和 Contact 标头中。
//   SIP name-addr 包含一个通用 URI 和一个显示名称。

pub struct SipNameAddr {
    display: String,
    url: Url
}

pub fn sip_uri_create() {

}
// 根据安全标志将 SIP URI 方案更改为 sip 或 sips
// Change the SIP URI scheme to sip or sips based on the secure flag.
pub fn sip_uri_set_secure() {

}

pub fn sip_uri_init() {

}
pub fn sip_uri_assign() {

}

pub fn name_addr_create() {

}
pub fn name_addr_init() {

}
pub fn name_addr_assign() {

}
pub struct OtherUrl{
 scheme: String,
    content: String
}

pub fn other_url_create(){

}