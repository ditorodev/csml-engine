use crate::data::error_info::ErrorInfo;
use crate::data::position::Position;
use crate::data::primitive::{
    boolean::PrimitiveBoolean, object::PrimitiveObject, string::PrimitiveString, Primitive,
    PrimitiveType, Right,
};
use crate::data::{ast::Interval, literal::ContentType, message::Message, tokens::NULL, Literal};
use crate::error_format::*;
use lazy_static::*;
use std::cmp::Ordering;
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

type PrimitiveMethod = fn(
    null: &mut PrimitiveNull,
    args: &[Literal],
    interval: Interval,
) -> Result<Literal, ErrorInfo>;

lazy_static! {
    static ref FUNCTIONS: HashMap<&'static str, (PrimitiveMethod, Right)> = {
        let mut map = HashMap::new();

        map.insert(
            "is_number",
            (PrimitiveNull::is_number as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "type_of",
            (PrimitiveNull::type_of as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "to_string",
            (PrimitiveNull::to_string as PrimitiveMethod, Right::Read),
        );

        map
    };
}

#[derive(PartialEq, Debug, Clone)]
pub struct PrimitiveNull {}

////////////////////////////////////////////////////////////////////////////////
// METHOD FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl PrimitiveNull {
    fn is_number(
        _null: &mut PrimitiveNull,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "is_number() => boolean";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveBoolean::get_literal(false, interval))
    }

    fn type_of(
        _null: &mut PrimitiveNull,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "type_of() => string";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveString::get_literal("Null", interval))
    }

    fn to_string(
        null: &mut PrimitiveNull,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "to_string() => string";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveString::get_literal(&null.to_string(), interval))
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl Default for PrimitiveNull {
    fn default() -> Self {
        Self {}
    }
}

impl PrimitiveNull {
    pub fn get_literal(interval: Interval) -> Literal {
        let primitive = Box::new(PrimitiveNull::default());

        Literal {
            content_type: "null".to_owned(),
            primitive,
            interval,
        }
    }
}

impl Primitive for PrimitiveNull {
    fn do_exec(
        &mut self,
        name: &str,
        args: &[Literal],
        interval: Interval,
        _content_type: &ContentType,
    ) -> Result<(Literal, Right), ErrorInfo> {
        if let Some((f, right)) = FUNCTIONS.get(name) {
            let res = f(self, args, interval)?;

            return Ok((res, *right));
        }

        Err(gen_error_info(
            Position::new(interval),
            format!("[{}] {}", name, ERROR_NULL_UNKNOWN_METHOD),
        ))
    }

    fn is_eq(&self, other: &dyn Primitive) -> bool {
        if let Some(_other) = other.as_any().downcast_ref::<Self>() {
            return true;
        }

        false
    }

    fn is_cmp(&self, _other: &dyn Primitive) -> Option<Ordering> {
        Some(Ordering::Equal)
    }

    fn do_add(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, String> {
        Err(format!(
            "{} {:?} + {:?}",
            ERROR_ILLEGAL_OPERATION,
            self.get_type(),
            other.get_type()
        ))
    }

    fn do_sub(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, String> {
        Err(format!(
            "{} {:?} - {:?}",
            ERROR_ILLEGAL_OPERATION,
            self.get_type(),
            other.get_type()
        ))
    }

    fn do_div(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, String> {
        Err(format!(
            "{} {:?} / {:?}",
            ERROR_ILLEGAL_OPERATION,
            self.get_type(),
            other.get_type()
        ))
    }

    fn do_mul(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, String> {
        Err(format!(
            "{} {:?} * {:?}",
            ERROR_ILLEGAL_OPERATION,
            self.get_type(),
            other.get_type()
        ))
    }

    fn do_rem(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, String> {
        Err(format!(
            "{} {:?} % {:?}",
            ERROR_ILLEGAL_OPERATION,
            self.get_type(),
            other.get_type()
        ))
    }

    fn as_debug(&self) -> &dyn std::fmt::Debug {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_type(&self) -> PrimitiveType {
        PrimitiveType::PrimitiveNull
    }

    fn as_box_clone(&self) -> Box<dyn Primitive> {
        Box::new((*self).clone())
    }

    fn to_json(&self) -> serde_json::Value {
        serde_json::Value::Null
    }

    fn format_mem(&self, _content_type: &str, _first: bool) -> serde_json::Value {
        serde_json::Value::Null
    }

    fn to_string(&self) -> String {
        "Null".to_owned()
    }

    fn as_bool(&self) -> bool {
        false
    }

    fn get_value(&self) -> &dyn std::any::Any {
        &NULL
    }

    fn get_mut_value(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn to_msg(&self, _content_type: String) -> Message {
        let mut hashmap: HashMap<String, Literal> = HashMap::new();

        hashmap.insert(
            "text".to_owned(),
            Literal {
                content_type: "text".to_owned(),
                primitive: Box::new(PrimitiveNull::default()),
                interval: Interval { column: 0, line: 0 },
            },
        );

        let mut result = PrimitiveObject::get_literal(&hashmap, Interval { column: 0, line: 0 });
        result.set_content_type("text");

        Message {
            content_type: result.content_type,
            content: result.primitive.to_json(),
        }
    }
}
