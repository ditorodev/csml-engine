mod support;

use csmlinterpreter::data::context::ContextJson;
use csmlinterpreter::data::event::Event;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn int_step_0() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "1764"}, "content_type":"text"},
            {"content":{"text": "int"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "int_0",
            "flow",
        ),
        "CSML/basic_test/stdlib/number.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn int_step_1() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "3725.1900894013565"}, "content_type":"text"},
            {"content":{"text": "float"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "int_1",
            "flow",
        ),
        "CSML/basic_test/stdlib/number.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn int_step_2() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "3725.1900894013565"}, "content_type":"text"},
            {"content":{"text": "float"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "int_2",
            "flow",
        ),
        "CSML/basic_test/stdlib/number.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn float_step_0() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "1764"}, "content_type":"text"},
            {"content":{"text": "float"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "float_0",
            "flow",
        ),
        "CSML/basic_test/stdlib/number.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn float_step_1() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "3725.1900894013565"}, "content_type":"text"},
            {"content":{"text": "float"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "float_1",
            "flow",
        ),
        "CSML/basic_test/stdlib/number.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn string_step_0() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "1764"}, "content_type":"text"},
            {"content":{"text": "int"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "string_0",
            "flow",
        ),
        "CSML/basic_test/stdlib/number.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn string_step_1() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "3725.1900894013565"}, "content_type":"text"},
            {"content":{"text": "float"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "string_1",
            "flow",
        ),
        "CSML/basic_test/stdlib/number.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
