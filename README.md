# usp

This package can decode and encode usp records.

## Usage

Create a USP record that is ready for MQTT transport.

```
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

// encoded_record can not be transmitted on the MTP
let encoded_record = record.encode_to_vec();
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
if let Some(usp_record::record::RecordType::NoSessionContext(ns_record)) = decoded_record.record_type {
    let message = ns_record.decode();
    assert_eq!(ns_record.payload, get_vec);
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

