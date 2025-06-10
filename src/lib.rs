// Include generated protobuf module
pub mod usp {
    include!(concat!(env!("OUT_DIR"), "/usp.rs"));
}
pub mod usp_record {
    include!(concat!(env!("OUT_DIR"), "/usp_record.rs"));
}

#[cfg(test)]
mod tests {
    use prost::Message;

    use super::*;

    #[test]
    fn test_protobuf_inclusion() {
        // Ensure that the generated protobuf module is included
        let get = usp::Get {
            param_paths: vec!["example.path".to_string()],
        };
        let get_vec = get.encode_to_vec();

        let record = usp_record::Record {
            version: "1.0".to_string(),
            to_id: String::from("device123"),
            from_id: String::from("controller456"),
            payload_security: usp_record::record::PayloadSecurity::Plaintext as i32,
            mac_signature: vec![0x01, 0x02, 0x03],
            sender_cert: vec![0x04, 0x05, 0x06],
            record_type: Some(usp_record::record::RecordType::NoSessionContext(
                usp_record::NoSessionContextRecord {
                    payload: get_vec.clone(),
                },
            )),
        };

        let encoded_record = record.encode_to_vec();

        let decoded_record = usp_record::Record::decode(&*encoded_record).expect("Decoding failed");
        assert_eq!(decoded_record.version, "1.0");
        assert_eq!(decoded_record.to_id, "device123");
        assert_eq!(decoded_record.from_id, "controller456");
        assert_eq!(
            decoded_record.payload_security,
            usp_record::record::PayloadSecurity::Plaintext as i32
        );
        assert_eq!(decoded_record.mac_signature, vec![0x01, 0x02, 0x03]);
        assert_eq!(decoded_record.sender_cert, vec![0x04, 0x05, 0x06]);
        if let Some(usp_record::record::RecordType::NoSessionContext(ns_record)) =
            decoded_record.record_type
        {
            assert_eq!(ns_record.payload, get_vec);
        } else {
            panic!("Expected NoSessionContextRecord");
        }
    }
}
