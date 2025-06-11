# usp

This package can decode and encode usp records.

## Usage

Create a USP record that is ready for MQTT transport.

This is just an example to show the the UspRecord struct can be created.

You are either a Controller or an Agent, never both. If you were, you would have 2 UspRecord.

```
        
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

    // Now encode a message ready for the MTP
    let encoded_get = usp_record
        .get("device123", &["example.path"])
        .expect("Failed to encode Get request");

    // this can now by sent to the MTP, i.e. MQTT or other
    mtp.send(device_channel, &encoded_get);

```

Decode a record retrieved from MQTT.

```
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
            decoded_record.record_type {
    match usp::Msg::decode(&ns_record.payload[..]) {
        Ok(usp_msg) => {
            let header = usp_msg.header.expect("Header should be present");
            let body = usp_msg.body.expect("Body should be present");
            assert_eq!(header.msg_type, header::MsgType::Get as i32);
            // it could be Response too :)
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
        Err(e) => panic!("❌ Failed to decode USP Msg: {:?}", e),
    }
} else {
    panic!("Expected NoSessionContextRecord");
}
```


## TR-369 / USP Message Types

| Message Type              | Description                                           | Direction(s)                          |
|--------------------------|-------------------------------------------------------|----------------------------------------|
| `Get`                    | Requests one or more parameter values.               | Controller → Agent                     |
| `GetResponse`            | Response to a Get request.                           | Agent → Controller                     |
| `Set`                    | Updates values of one or more parameters.            | Controller → Agent                     |
| `SetResponse`            | Response to a Set request.                           | Agent → Controller                     |
| `Add`                    | Requests creation of one or more instances.          | Controller → Agent                     |
| `AddResponse`            | Response to Add request.                             | Agent → Controller                     |
| `Delete`                 | Deletes one or more instances.                       | Controller → Agent                     |
| `DeleteResponse`         | Response to Delete request.                          | Agent → Controller                     |
| `Operate`                | Invokes a supported operation or command.            | Controller → Agent                     |
| `OperateResponse`        | Response to Operate request.                         | Agent → Controller                     |
| `GetSupportedDM`         | Requests list of supported objects/parameters.       | Controller → Agent                     |
| `GetSupportedDMResponse` | Response to GetSupportedDM request.                  | Agent → Controller                     |
| `GetInstances`           | Requests list of instances for a given object.       | Controller → Agent                     |
| `GetInstancesResponse`   | Response to GetInstances request.                    | Agent → Controller                     |
| `Notify`                 | Sends an event or parameter change notification.     | Agent → Controller                     |
| `NotifyResponse`         | Optional response to Notify (if `send_resp` is true).| Controller → Agent                     |
| `Error`                  | Reports a protocol or command error.                 | Agent ↔ Controller ↔ Controller        |

