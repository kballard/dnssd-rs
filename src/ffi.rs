#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::{
    c_char,
    c_void,
    c_int,
    c_uchar
};

// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct _DNSServiceRef_t {
//     _unused: [u8; 0],
// }
// pub type DNSServiceRef = *mut _DNSServiceRef_t;
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct _DNSRecordRef_t {
//     _unused: [u8; 0],
// }
// pub type DNSRecordRef = *mut _DNSRecordRef_t;
pub enum DNSServiceT {}
pub type DNSServiceRef = *mut DNSServiceT;

// type without an instance
pub enum DNSRecordT {}
pub type DNSRecordRef = *mut DNSRecordT;

pub const kDNSServiceFlagsMoreComing: _bindgen_ty_1 = 1;
pub const kDNSServiceFlagsAdd: _bindgen_ty_1 = 2;
pub const kDNSServiceFlagsDefault: _bindgen_ty_1 = 4;
pub const kDNSServiceFlagsNoAutoRename: _bindgen_ty_1 = 8;
pub const kDNSServiceFlagsShared: _bindgen_ty_1 = 16;
pub const kDNSServiceFlagsUnique: _bindgen_ty_1 = 32;
pub const kDNSServiceFlagsBrowseDomains: _bindgen_ty_1 = 64;
pub const kDNSServiceFlagsRegistrationDomains: _bindgen_ty_1 = 128;
pub const kDNSServiceFlagsLongLivedQuery: _bindgen_ty_1 = 256;
pub const kDNSServiceFlagsAllowRemoteQuery: _bindgen_ty_1 = 512;
pub const kDNSServiceFlagsForceMulticast: _bindgen_ty_1 = 1024;
pub const kDNSServiceFlagsReturnCNAME: _bindgen_ty_1 = 2048;
pub type _bindgen_ty_1 = u32;
pub const kDNSServiceClass_IN: _bindgen_ty_2 = 1;
pub type _bindgen_ty_2 = u32;
pub const kDNSServiceType_A: _bindgen_ty_3 = 1;
pub const kDNSServiceType_NS: _bindgen_ty_3 = 2;
pub const kDNSServiceType_MD: _bindgen_ty_3 = 3;
pub const kDNSServiceType_MF: _bindgen_ty_3 = 4;
pub const kDNSServiceType_CNAME: _bindgen_ty_3 = 5;
pub const kDNSServiceType_SOA: _bindgen_ty_3 = 6;
pub const kDNSServiceType_MB: _bindgen_ty_3 = 7;
pub const kDNSServiceType_MG: _bindgen_ty_3 = 8;
pub const kDNSServiceType_MR: _bindgen_ty_3 = 9;
pub const kDNSServiceType_NULL: _bindgen_ty_3 = 10;
pub const kDNSServiceType_WKS: _bindgen_ty_3 = 11;
pub const kDNSServiceType_PTR: _bindgen_ty_3 = 12;
pub const kDNSServiceType_HINFO: _bindgen_ty_3 = 13;
pub const kDNSServiceType_MINFO: _bindgen_ty_3 = 14;
pub const kDNSServiceType_MX: _bindgen_ty_3 = 15;
pub const kDNSServiceType_TXT: _bindgen_ty_3 = 16;
pub const kDNSServiceType_RP: _bindgen_ty_3 = 17;
pub const kDNSServiceType_AFSDB: _bindgen_ty_3 = 18;
pub const kDNSServiceType_X25: _bindgen_ty_3 = 19;
pub const kDNSServiceType_ISDN: _bindgen_ty_3 = 20;
pub const kDNSServiceType_RT: _bindgen_ty_3 = 21;
pub const kDNSServiceType_NSAP: _bindgen_ty_3 = 22;
pub const kDNSServiceType_NSAP_PTR: _bindgen_ty_3 = 23;
pub const kDNSServiceType_SIG: _bindgen_ty_3 = 24;
pub const kDNSServiceType_KEY: _bindgen_ty_3 = 25;
pub const kDNSServiceType_PX: _bindgen_ty_3 = 26;
pub const kDNSServiceType_GPOS: _bindgen_ty_3 = 27;
pub const kDNSServiceType_AAAA: _bindgen_ty_3 = 28;
pub const kDNSServiceType_LOC: _bindgen_ty_3 = 29;
pub const kDNSServiceType_NXT: _bindgen_ty_3 = 30;
pub const kDNSServiceType_EID: _bindgen_ty_3 = 31;
pub const kDNSServiceType_NIMLOC: _bindgen_ty_3 = 32;
pub const kDNSServiceType_SRV: _bindgen_ty_3 = 33;
pub const kDNSServiceType_ATMA: _bindgen_ty_3 = 34;
pub const kDNSServiceType_NAPTR: _bindgen_ty_3 = 35;
pub const kDNSServiceType_KX: _bindgen_ty_3 = 36;
pub const kDNSServiceType_CERT: _bindgen_ty_3 = 37;
pub const kDNSServiceType_A6: _bindgen_ty_3 = 38;
pub const kDNSServiceType_DNAME: _bindgen_ty_3 = 39;
pub const kDNSServiceType_SINK: _bindgen_ty_3 = 40;
pub const kDNSServiceType_OPT: _bindgen_ty_3 = 41;
pub const kDNSServiceType_TKEY: _bindgen_ty_3 = 249;
pub const kDNSServiceType_TSIG: _bindgen_ty_3 = 250;
pub const kDNSServiceType_IXFR: _bindgen_ty_3 = 251;
pub const kDNSServiceType_AXFR: _bindgen_ty_3 = 252;
pub const kDNSServiceType_MAILB: _bindgen_ty_3 = 253;
pub const kDNSServiceType_MAILA: _bindgen_ty_3 = 254;
pub const kDNSServiceType_ANY: _bindgen_ty_3 = 255;
pub type _bindgen_ty_3 = u32;
pub const kDNSServiceErr_NoError: _bindgen_ty_4 = 0;
pub const kDNSServiceErr_Unknown: _bindgen_ty_4 = -65537;
pub const kDNSServiceErr_NoSuchName: _bindgen_ty_4 = -65538;
pub const kDNSServiceErr_NoMemory: _bindgen_ty_4 = -65539;
pub const kDNSServiceErr_BadParam: _bindgen_ty_4 = -65540;
pub const kDNSServiceErr_BadReference: _bindgen_ty_4 = -65541;
pub const kDNSServiceErr_BadState: _bindgen_ty_4 = -65542;
pub const kDNSServiceErr_BadFlags: _bindgen_ty_4 = -65543;
pub const kDNSServiceErr_Unsupported: _bindgen_ty_4 = -65544;
pub const kDNSServiceErr_NotInitialized: _bindgen_ty_4 = -65545;
pub const kDNSServiceErr_AlreadyRegistered: _bindgen_ty_4 = -65547;
pub const kDNSServiceErr_NameConflict: _bindgen_ty_4 = -65548;
pub const kDNSServiceErr_Invalid: _bindgen_ty_4 = -65549;
pub const kDNSServiceErr_Firewall: _bindgen_ty_4 = -65550;
pub const kDNSServiceErr_Incompatible: _bindgen_ty_4 = -65551;
pub const kDNSServiceErr_BadInterfaceIndex: _bindgen_ty_4 = -65552;
pub const kDNSServiceErr_Refused: _bindgen_ty_4 = -65553;
pub const kDNSServiceErr_NoSuchRecord: _bindgen_ty_4 = -65554;
pub const kDNSServiceErr_NoAuth: _bindgen_ty_4 = -65555;
pub const kDNSServiceErr_NoSuchKey: _bindgen_ty_4 = -65556;
pub const kDNSServiceErr_NATTraversal: _bindgen_ty_4 = -65557;
pub const kDNSServiceErr_DoubleNAT: _bindgen_ty_4 = -65558;
pub const kDNSServiceErr_BadTime: _bindgen_ty_4 = -65559;
pub type _bindgen_ty_4 = i32;
pub type DNSServiceFlags = u32;
pub type DNSServiceErrorType = i32;
extern "C" {
    /// Unix Domain Socket access, DNSServiceRef deallocation, and data processing functions
    ///
    pub fn DNSServiceRefSockFD(sdRef: DNSServiceRef) -> c_int;
    pub fn DNSServiceProcessResult(sdRef: DNSServiceRef) -> DNSServiceErrorType;
    pub fn DNSServiceRefDeallocate(sdRef: DNSServiceRef);
}
/// Domain Enumeration
///
pub type DNSServiceDomainEnumReply = Option<
    unsafe extern "C" fn(
        sdRef: DNSServiceRef,
        flags: DNSServiceFlags,
        interfaceIndex: u32,
        errorCode: DNSServiceErrorType,
        replyDomain: *const c_char,
        context: *mut c_void,
    ),
>;
extern "C" {
    pub fn DNSServiceEnumerateDomains(
        sdRef: *mut DNSServiceRef,
        flags: DNSServiceFlags,
        interfaceIndex: u32,
        callBack: DNSServiceDomainEnumReply,
        context: *mut c_void,
    ) -> DNSServiceErrorType;
}
///  Service Registration
///
pub type DNSServiceRegisterReply = Option<
	extern "C" fn(
		sd_ref: DNSServiceRef,
		flags: DNSServiceFlags,
		error_code: DNSServiceErrorType,
		name: *const c_char,
		reg_type: *const c_char,
		domain: *const c_char,
		context: *mut c_void,
	),
>;
extern "C" {
    pub fn DNSServiceRegister(
        sdRef: *mut DNSServiceRef,
        flags: DNSServiceFlags,
        interfaceIndex: u32,
        name: *const c_char,
        regtype: *const c_char,
        domain: *const c_char,
        host: *const c_char,
        port: u16,
        txtLen: u16,
        txtRecord: *const c_void,
        callBack: DNSServiceRegisterReply,
        context: *mut c_void,
    ) -> DNSServiceErrorType;
    pub fn DNSServiceAddRecord(
        sdRef: DNSServiceRef,
        RecordRef: *mut DNSRecordRef,
        flags: DNSServiceFlags,
        rrtype: u16,
        rdlen: u16,
        rdata: *const c_void,
        ttl: u32,
    ) -> DNSServiceErrorType;
    pub fn DNSServiceUpdateRecord(
        sdRef: DNSServiceRef,
        RecordRef: DNSRecordRef,
        flags: DNSServiceFlags,
        rdlen: u16,
        rdata: *const c_void,
        ttl: u32,
    ) -> DNSServiceErrorType;
    pub fn DNSServiceRemoveRecord(
        sdRef: DNSServiceRef,
        RecordRef: DNSRecordRef,
        flags: DNSServiceFlags,
    ) -> DNSServiceErrorType;
}
///  Service Discovery
///
pub type DNSServiceBrowseReply = Option<
    unsafe extern "C" fn(
        sdRef: DNSServiceRef,
        flags: DNSServiceFlags,
        interfaceIndex: u32,
        errorCode: DNSServiceErrorType,
        serviceName: *const c_char,
        regtype: *const c_char,
        replyDomain: *const c_char,
        context: *mut c_void,
    ),
>;
extern "C" {
    pub fn DNSServiceBrowse(
        sdRef: *mut DNSServiceRef,
        flags: DNSServiceFlags,
        interfaceIndex: u32,
        regtype: *const c_char,
        domain: *const c_char,
        callBack: DNSServiceBrowseReply,
        context: *mut c_void,
    ) -> DNSServiceErrorType;
}
pub type DNSServiceResolveReply = Option<
    unsafe extern "C" fn(
        sdRef: DNSServiceRef,
        flags: DNSServiceFlags,
        interfaceIndex: u32,
        errorCode: DNSServiceErrorType,
        fullname: *const c_char,
        hosttarget: *const c_char,
        port: u16,
        txtLen: u16,
        txtRecord: *const c_uchar,
        context: *mut c_void,
    ),
>;
extern "C" {
    pub fn DNSServiceResolve(
        sdRef: *mut DNSServiceRef,
        flags: DNSServiceFlags,
        interfaceIndex: u32,
        name: *const c_char,
        regtype: *const c_char,
        domain: *const c_char,
        callBack: DNSServiceResolveReply,
        context: *mut c_void,
    ) -> DNSServiceErrorType;
    ///  Special Purpose Calls (most applications will not use these)
    ///
    pub fn DNSServiceCreateConnection(sdRef: *mut DNSServiceRef) -> DNSServiceErrorType;
}
pub type DNSServiceRegisterRecordReply = Option<
    unsafe extern "C" fn(
        sdRef: DNSServiceRef,
        RecordRef: DNSRecordRef,
        flags: DNSServiceFlags,
        errorCode: DNSServiceErrorType,
        context: *mut c_void,
    ),
>;
extern "C" {
    pub fn DNSServiceRegisterRecord(
        sdRef: DNSServiceRef,
        RecordRef: *mut DNSRecordRef,
        flags: DNSServiceFlags,
        interfaceIndex: u32,
        fullname: *const c_char,
        rrtype: u16,
        rrclass: u16,
        rdlen: u16,
        rdata: *const c_void,
        ttl: u32,
        callBack: DNSServiceRegisterRecordReply,
        context: *mut c_void,
    ) -> DNSServiceErrorType;
}
pub type DNSServiceQueryRecordReply = Option<
    unsafe extern "C" fn(
        DNSServiceRef: DNSServiceRef,
        flags: DNSServiceFlags,
        interfaceIndex: u32,
        errorCode: DNSServiceErrorType,
        fullname: *const c_char,
        rrtype: u16,
        rrclass: u16,
        rdlen: u16,
        rdata: *const c_void,
        ttl: u32,
        context: *mut c_void,
    ),
>;
extern "C" {
    pub fn DNSServiceQueryRecord(
        sdRef: *mut DNSServiceRef,
        flags: DNSServiceFlags,
        interfaceIndex: u32,
        fullname: *const c_char,
        rrtype: u16,
        rrclass: u16,
        callBack: DNSServiceQueryRecordReply,
        context: *mut c_void,
    ) -> DNSServiceErrorType;
    pub fn DNSServiceReconfirmRecord(
        flags: DNSServiceFlags,
        interfaceIndex: u32,
        fullname: *const c_char,
        rrtype: u16,
        rrclass: u16,
        rdlen: u16,
        rdata: *const c_void,
    ) -> DNSServiceErrorType;
    ///  General Utility Functions
    ///
    pub fn DNSServiceConstructFullName(
        fullName: *mut c_char,
        service: *const c_char,
        regtype: *const c_char,
        domain: *const c_char,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _TXTRecordRef_t {
    pub PrivateData: [c_char; 16usize],
    pub ForceNaturalAlignment: *mut c_char,
    _bindgen_union_align: [u64; 2usize],
}
#[test]
fn bindgen_test_layout__TXTRecordRef_t() {
    assert_eq!(
        ::std::mem::size_of::<_TXTRecordRef_t>(),
        16usize,
        concat!("Size of: ", stringify!(_TXTRecordRef_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_TXTRecordRef_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_TXTRecordRef_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_TXTRecordRef_t>())).PrivateData as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_TXTRecordRef_t),
            "::",
            stringify!(PrivateData)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_TXTRecordRef_t>())).ForceNaturalAlignment as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_TXTRecordRef_t),
            "::",
            stringify!(ForceNaturalAlignment)
        )
    );
}
pub type TXTRecordRef = _TXTRecordRef_t;
extern "C" {
    pub fn TXTRecordCreate(
        txtRecord: *mut TXTRecordRef,
        bufferLen: u16,
        buffer: *mut c_void,
    );
    pub fn TXTRecordDeallocate(txtRecord: *mut TXTRecordRef);
    pub fn TXTRecordSetValue(
        txtRecord: *mut TXTRecordRef,
        key: *const c_char,
        valueSize: u8,
        value: *const c_void,
    ) -> DNSServiceErrorType;
    pub fn TXTRecordRemoveValue(
        txtRecord: *mut TXTRecordRef,
        key: *const c_char,
    ) -> DNSServiceErrorType;
    pub fn TXTRecordGetLength(txtRecord: *const TXTRecordRef) -> u16;
    pub fn TXTRecordGetBytesPtr(txtRecord: *const TXTRecordRef) -> *const c_void;
    pub fn TXTRecordContainsKey(
        txtLen: u16,
        txtRecord: *const c_void,
        key: *const c_char,
    ) -> ::std::os::raw::c_int;
    pub fn TXTRecordGetValuePtr(
        txtLen: u16,
        txtRecord: *const c_void,
        key: *const c_char,
        valueLen: *mut u8,
    ) -> *const c_void;
    pub fn TXTRecordGetCount(txtLen: u16, txtRecord: *const c_void) -> u16;
    pub fn TXTRecordGetItemAtIndex(
        txtLen: u16,
        txtRecord: *const c_void,
        index: u16,
        keyBufLen: u16,
        key: *mut c_char,
        valueLen: *mut u8,
        value: *mut *const c_void,
    ) -> DNSServiceErrorType;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DNS_SD_CompileTimeAssertionChecks {
    pub assert0: [c_char; 1usize],
}
#[test]
fn bindgen_test_layout_DNS_SD_CompileTimeAssertionChecks() {
    assert_eq!(
        ::std::mem::size_of::<DNS_SD_CompileTimeAssertionChecks>(),
        1usize,
        concat!("Size of: ", stringify!(DNS_SD_CompileTimeAssertionChecks))
    );
    assert_eq!(
        ::std::mem::align_of::<DNS_SD_CompileTimeAssertionChecks>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(DNS_SD_CompileTimeAssertionChecks)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<DNS_SD_CompileTimeAssertionChecks>())).assert0 as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DNS_SD_CompileTimeAssertionChecks),
            "::",
            stringify!(assert0)
        )
    );
}
