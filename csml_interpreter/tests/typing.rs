mod support;

use csmlinterpreter::data::context::ContextJson;
use csmlinterpreter::data::event::Event;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn typing_0() {
    let data = r#"{"messages":[ {"content":{"error": "Builtin Typing expect one argument of type int or float | example: Typing(3, ..) at line 5, column 6 in step [typing_0] from flow [flow]"},"content_type":"error"} ],"memories":[]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "typing_0",
            "flow",
        ),
        "CSML/basic_test/built-in/typing.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn typing_1() {
    let data =
        r#"{"messages":[ {"content":{"duration": 10},"content_type":"typing"} ],"memories":[]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "typing_1",
            "flow",
        ),
        "CSML/basic_test/built-in/typing.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
