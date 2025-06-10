# usp

This package can decode and encode usp records.

## Usage

```
let get = usp.Msg.Get()
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

