use crate::error::UspError;
use prost::Message;

pub mod error;

// Include generated protobuf module
pub mod usp {
    include!(concat!(env!("OUT_DIR"), "/usp.rs"));
}
pub mod usp_record {
    include!(concat!(env!("OUT_DIR"), "/usp_record.rs"));
}

pub struct UspRecord {
    version: String,
    to_id: Option<String>,
    from_id: String,
    payload_security: i32,
    mac_signature: Vec<u8>,
    sender_cert: Vec<u8>,
    record_type: Option<usp_record::record::RecordType>,
}

impl UspRecord {
    /// Create a new UspRecord instance
    ///
    /// This function initializes a new UspRecord with the provided parameters.
    ///
    /// The arguments for this function are the values that should be constant for all produced records.
    ///
    /// # Arguments
    /// * `version` - The version of the USP protocol
    /// * `from_id` - The ID of the sender
    /// * `payload_security` - The security level of the payload
    /// * `mac_signature` - The MAC or signature for the record
    /// * `sender_cert` - The certificate of the sender
    pub fn new(
        version: String,
        from_id: String,
        payload_security: i32,
        mac_signature: Vec<u8>,
        sender_cert: Vec<u8>,
    ) -> Self {
        UspRecord {
            version,
            to_id: None,
            from_id,
            payload_security,
            mac_signature,
            sender_cert,
            record_type: None,
        }
    }

    fn set_to_id(&mut self, to_id: &str) {
        self.to_id = Some(to_id.to_string());
    }

    fn set_record_type(&mut self, record_type: &usp_record::record::RecordType) {
        self.record_type = Some(record_type.clone());
    }

    fn validate(&self) -> Result<(), UspError> {
        // Ensure that to_id is set before building
        if self.to_id.is_none() {
            return Err(UspError::ToIdNotSet);
        }
        if self.record_type.is_none() {
            return Err(UspError::RecordTypeNotSet);
        }
        Ok(())
    }

    pub fn encode_record(
        &mut self,
        to_id: &str,
        record_type: &usp_record::record::RecordType,
    ) -> Result<Vec<u8>, UspError> {
        self.set_to_id(to_id);
        self.set_record_type(record_type);
        self.validate()?;

        let record = usp_record::Record {
            version: self.version.clone(),
            to_id: to_id.to_string(),
            from_id: self.from_id.clone(),
            payload_security: self.payload_security,
            mac_signature: self.mac_signature.clone(),
            sender_cert: self.sender_cert.clone(),
            record_type: Some(record_type.clone()),
        };

        Ok(record.encode_to_vec())
    }

    pub fn get(&mut self, to_id: &str, path: &[&str]) -> Result<Vec<u8>, UspError> {
        let paths: Vec<String> = path.iter().map(|&p| p.to_string()).collect();
        let get = usp::Get { param_paths: paths };
        let get_msg = usp::Msg {
            header: Some(usp::Header {
                msg_id: to_id.to_string(),
                msg_type: usp::header::MsgType::Get as i32,
            }),
            body: Some(usp::Body {
                msg_body: Some(usp::body::MsgBody::Request(usp::Request {
                    req_type: Some(usp::request::ReqType::Get(get)),
                })),
            }),
        };
        let record_type =
            usp_record::record::RecordType::NoSessionContext(usp_record::NoSessionContextRecord {
                payload: get_msg.encode_to_vec(),
            });
        self.encode_record(to_id, &record_type)
    }
}

#[cfg(test)]
mod tests {
    use crate::usp::header;

    use super::*;
    use prost::Message;

    fn decode_and_assert(
        encoded: &[u8],
        expected_to_id: &str,
        expected_from_id: &str,
        expected_msg_type: header::MsgType,
        expected_msg_id: &str,
    ) -> usp::Body {
        let decoded_record = usp_record::Record::decode(&*encoded).expect("Decoding failed");
        assert_eq!(decoded_record.version, "1.0");
        assert_eq!(decoded_record.to_id, expected_to_id);
        assert_eq!(decoded_record.from_id, expected_from_id);
        assert_eq!(
            decoded_record.payload_security,
            usp_record::record::PayloadSecurity::Plaintext as i32
        );
        assert_eq!(decoded_record.mac_signature, vec![0x01, 0x02, 0x03]);
        assert_eq!(decoded_record.sender_cert, vec![0x04, 0x05, 0x06]);
        if let Some(usp_record::record::RecordType::NoSessionContext(ns_record)) =
            decoded_record.record_type
        {
            match usp::Msg::decode(&ns_record.payload[..]) {
                Ok(usp_msg) => {
                    let header = usp_msg.header.expect("Header should be present");
                    let body = usp_msg.body.expect("Body should be present");
                    assert_eq!(header.msg_type, expected_msg_type as i32);
                    assert_eq!(header.msg_id, expected_msg_id);
                    return body;
                }
                Err(e) => panic!("❌ Failed to decode USP Msg: {:?}", e),
            }
        } else {
            panic!("Expected NoSessionContextRecord");
        }
    }

    #[test]
    fn test_protobuf_inclusion() {
        // Ensure that the generated protobuf module is included
        let get = usp::Get {
            param_paths: vec!["example.path".to_string()],
        };
        let get_msg = usp::Msg {
            header: Some(usp::Header {
                msg_id: "12345".to_string(),
                msg_type: header::MsgType::Get as i32,
            }),
            body: Some(usp::Body {
                msg_body: Some(usp::body::MsgBody::Request(usp::Request {
                    req_type: Some(usp::request::ReqType::Get(get)),
                })),
            }),
        };
        let get_msg_vec = get_msg.encode_to_vec();

        let record = usp_record::Record {
            version: "1.0".to_string(),
            to_id: String::from("device123"),
            from_id: String::from("controller456"),
            payload_security: usp_record::record::PayloadSecurity::Plaintext as i32,
            mac_signature: vec![0x01, 0x02, 0x03],
            sender_cert: vec![0x04, 0x05, 0x06],
            record_type: Some(usp_record::record::RecordType::NoSessionContext(
                usp_record::NoSessionContextRecord {
                    payload: get_msg_vec.clone(),
                },
            )),
        };

        let encoded_record = record.encode_to_vec();

        let body = decode_and_assert(
            &encoded_record,
            "device123",
            "controller456",
            header::MsgType::Get,
            "12345",
        );
        assert!(body.msg_body.is_some());
        if let Some(usp::body::MsgBody::Request(req)) = body.msg_body {
            if let Some(usp::request::ReqType::Get(get)) = req.req_type {
                assert_eq!(get.param_paths, vec!["example.path".to_string()]);
            } else {
                panic!("Expected Get request type");
            }
        } else {
            panic!("Expected Request MsgBody");
        }
    }

    #[test]
    fn test_record_creation_and_encoding() {
        let version = "1.0".to_string();
        let from_id = "controller456".to_string();
        let payload_security = usp_record::record::PayloadSecurity::Plaintext as i32;
        let mac_signature = vec![0x01, 0x02, 0x03];
        let sender_cert = vec![0x04, 0x05, 0x06];
        let mut usp_record = UspRecord::new(
            version,
            from_id,
            payload_security,
            mac_signature,
            sender_cert,
        );

        let encoded_get = usp_record
            .get("device123", &["example.path"])
            .expect("Failed to encode Get request");

        let body = decode_and_assert(
            &encoded_get,
            "device123",
            "controller456",
            header::MsgType::Get,
            "device123",
        );
        assert!(body.msg_body.is_some());
        if let Some(usp::body::MsgBody::Request(req)) = body.msg_body {
            if let Some(usp::request::ReqType::Get(get)) = req.req_type {
                assert_eq!(get.param_paths, vec!["example.path".to_string()]);
            } else {
                panic!("Expected Get request type");
            }
        } else {
            panic!("Expected Request MsgBody");
        }
    }
}
