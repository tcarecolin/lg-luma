use crate::system::{self, System};
use crate::Result;
use serde_json::{json, Value};

/// Root-only power actions for the home power button. Suspend is the same
/// standby the remote's power key triggers with Quick Start+ on; shutdown is a
/// real poweroff, like the remote's power key with Quick Start+ off. The exact
/// tvpower methods/payloads must be confirmed on the device.
pub fn dispatch(system: &impl System, action: &str) -> Result<Value> {
    system::root(system)?;
    let (method, payload) = match action {
        "suspend" => ("turnOffScreen", json!({})),
        "shutdown" => ("shutdown", json!({"standbyOff": true})),
        other => return Err(format!("Unknown power action: {other}")),
    };
    system::luna(system, "com.webos.service.tvpower", method, &payload)
}
