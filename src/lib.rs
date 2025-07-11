#![doc = include_str!("../README.md")]
#![no_std]
#![cfg(feature = "alloc")]
extern crate alloc;
/// If you are lazy or because of performance did not  specified different cases.
#[derive(Debug, Clone, Copy)]
pub struct MathError;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum MathErrors {
    InvalidInput,
    Overflow,
    Underflow,
    DivisionByZero,
}

#[derive(Debug, Clone, Copy)]
pub struct MathErrorsInvalidInput;

#[derive(Debug, Clone, Copy)]
pub struct MathErrorsOverflow;

#[derive(Debug, Clone, Copy)]
pub struct MathErrorsUnderflow;
#[derive(Debug, Clone, Copy)]
pub struct MathErrorsDivisionByZero;

impl From<MathErrorsInvalidInput> for MathError {
    fn from(_: MathErrorsInvalidInput) -> Self {
        MathError
    }
}

impl From<MathErrorsInvalidInput> for MathErrors {
    fn from(_: MathErrorsInvalidInput) -> Self {
        MathErrors::InvalidInput
    }
}

impl From<MathErrorsOverflow> for MathErrors {
    fn from(_: MathErrorsOverflow) -> Self {
        MathErrors::Overflow
    }
}

impl From<MathErrorsUnderflow> for MathErrors {
    fn from(_: MathErrorsUnderflow) -> Self {
        MathErrors::Underflow
    }
}

impl From<MathErrorsDivisionByZero> for MathErrors {
    fn from(_: MathErrorsDivisionByZero) -> Self {
        MathErrors::DivisionByZero
    }
}

impl From<MathErrorsOverflow> for MathError {
    fn from(_: MathErrorsOverflow) -> Self {
        MathError
    }
}

impl From<MathErrorsUnderflow> for MathError {
    fn from(_: MathErrorsUnderflow) -> Self {
        MathError
    }
}

impl From<MathErrorsDivisionByZero> for MathError {
    fn from(_: MathErrorsDivisionByZero) -> Self {
        MathError
    }
}

impl core::fmt::Display for MathErrors {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MathErrors::InvalidInput => write!(f, "InvalidInput"),
            MathErrors::Overflow => write!(f, "Overflow"),
            MathErrors::Underflow => write!(f, "Underflow"),
            MathErrors::DivisionByZero => write!(f, "Division"),
        }
    }
}

impl Into<u8> for MathErrors {
    fn into(self) -> u8 {
        self.to_primitive()
    }
}

impl MathErrors {
    pub const fn is_invalid_input(&self) -> bool {
        matches!(self, MathErrors::InvalidInput)
    }

    pub const fn is_overflow(&self) -> bool {
        matches!(self, MathErrors::Overflow)
    }

    pub const fn is_underflow(&self) -> bool {
        matches!(self, MathErrors::Underflow)
    }

    pub const fn is_division_by_zero(&self) -> bool {
        matches!(self, MathErrors::DivisionByZero)
    }

    pub const fn to_primitive(&self) -> u8 {
        match self {
            MathErrors::InvalidInput => 0,
            MathErrors::Overflow => 1,
            MathErrors::Underflow => 2,
            MathErrors::DivisionByZero => 3,
        }
    }

    pub const fn try_from_primitive(value: u8) -> Option<Self> {
        match value {
            0 => Some(MathErrors::InvalidInput),
            1 => Some(MathErrors::Overflow),
            2 => Some(MathErrors::Underflow),
            3 => Some(MathErrors::DivisionByZero),
            _ => None,
        }
    }

    pub const fn to_str(&self) -> &'static str {
        match self {
            MathErrors::InvalidInput => "InvalidInput",
            MathErrors::Overflow => "Overflow",
            MathErrors::Underflow => "Underflow",
            MathErrors::DivisionByZero => "DivisionByZero",
        }
    }
}

impl TryFrom<u8> for MathErrors {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from_primitive(value).ok_or("Must be in range 0..=3")
    }
}

impl From<MathErrors> for MathError {
    fn from(_: MathErrors) -> Self {
        MathError
    }
}

impl core::str::FromStr for MathErrors {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "InvalidInput" => Ok(MathErrors::InvalidInput),
            "Overflow" => Ok(MathErrors::Overflow),
            "Underflow" => Ok(MathErrors::Underflow),
            "DivisionByZero" => Ok(MathErrors::DivisionByZero),
            _ => Err("InvalidInput, Overlow, Underflow, or DivisionByZero"),
        }
    }
}

/// Serializes to and from string.
#[cfg(feature = "serde1")]
mod serde1_impl {
    use super::*;
    use serde1::{de::Visitor, Deserialize, Serialize};

    impl Serialize for MathErrors {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde1::Serializer,
        {
            serializer.serialize_str(self.to_str())
        }
    }

    impl<'de> Deserialize<'de> for MathErrors {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde1::Deserializer<'de>,
        {
            use core::fmt;
            struct SelfVisitor;
            impl Visitor<'_> for SelfVisitor {
                type Value = MathErrors;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    // exepcts one of enum string values
                    write!(
                        formatter,
                        "InvalidInput, Overflow, Underflow, or DivisionByZero"
                    )
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde1::de::Error,
                {
                    match v {
                        "InvalidInput" => Ok(MathErrors::InvalidInput),
                        "Overflow" => Ok(MathErrors::Overflow),
                        "Underflow" => Ok(MathErrors::Underflow),
                        "DivisionByZero" => Ok(MathErrors::DivisionByZero),
                        _ => Err(E::invalid_value(
                            serde1::de::Unexpected::Str(v),
                            &"InvalidInput, Overlow, Underflow, or DivisionByZero",
                        )),
                    }
                }
            }

            deserializer.deserialize_str(SelfVisitor)
        }
    }

    #[cfg(feature = "schemars1")]
    mod schemars1_impl {
        use super::*;
        use schemars1::{json_schema, JsonSchema};

        impl JsonSchema for MathErrors {
            fn schema_name() -> alloc::borrow::Cow<'static, str> {
                "MathErrors".into()
            }

            fn json_schema(_generator: &mut schemars1::SchemaGenerator) -> schemars1::Schema {
                json_schema!(
                    {
                        "description": "Simple math error",
                        "type": ["string"],
                        "enum": [
                            "InvalidInput",
                            "Overflow",
                            "Underflow",
                            "DivisionByZero"
                            ],
                        "maxLength": 14,
                        "minLength": 8,
                    }
                )
            }
        }
    }
}

#[cfg(feature = "borsh1")]
mod borsh1 {
    use borsh1::{BorshDeserialize, BorshSerialize};

    use crate::MathErrors;

    impl BorshDeserialize for MathErrors {
        fn deserialize_reader<R: borsh1::io::Read>(reader: &mut R) -> borsh1::io::Result<Self> {
            let value = u8::deserialize_reader(reader)?;
            Self::try_from_primitive(value).ok_or(borsh1::io::ErrorKind::InvalidData.into())
        }
    }

    impl BorshSerialize for MathErrors {
        fn serialize<W: borsh1::io::Write>(&self, writer: &mut W) -> borsh1::io::Result<()> {
            self.to_primitive().serialize(writer)
        }
    }

    #[cfg(feature = "borsh1_unstable__schema")]
    #[allow(non_snake_case)]
    mod borsh1_unstable__schema_impl {
        use super::super::{
            MathErrorsDivisionByZero, MathErrorsInvalidInput, MathErrorsOverflow,
            MathErrorsUnderflow,
        };
        use super::*;
        use alloc::string::ToString;
        use borsh1::{
            schema::{Definition, Fields},
            BorshSchema,
        };
        impl BorshSchema for MathErrorsInvalidInput {
            fn add_definitions_recursively(
                definitions: &mut alloc::collections::btree_map::BTreeMap<
                    borsh1::schema::Declaration,
                    Definition,
                >,
            ) {
                definitions.insert(
                    Self::declaration(),
                    Definition::Struct {
                        fields: Fields::Empty,
                    },
                );
            }

            fn declaration() -> borsh1::schema::Declaration {
                "MathErrorsInvalidInput".to_string()
            }
        }

        impl BorshSchema for MathErrorsOverflow {
            fn add_definitions_recursively(
                definitions: &mut alloc::collections::btree_map::BTreeMap<
                    borsh1::schema::Declaration,
                    Definition,
                >,
            ) {
                definitions.insert(
                    Self::declaration(),
                    Definition::Struct {
                        fields: Fields::Empty,
                    },
                );
            }

            fn declaration() -> borsh1::schema::Declaration {
                "MathErrorsOverflow".to_string()
            }
        }

        impl BorshSchema for MathErrorsUnderflow {
            fn add_definitions_recursively(
                definitions: &mut alloc::collections::btree_map::BTreeMap<
                    borsh1::schema::Declaration,
                    Definition,
                >,
            ) {
                definitions.insert(
                    Self::declaration(),
                    Definition::Struct {
                        fields: Fields::Empty,
                    },
                );
            }

            fn declaration() -> borsh1::schema::Declaration {
                "MathErrorsUnderflow".to_string()
            }
        }

        impl BorshSchema for MathErrorsDivisionByZero {
            fn add_definitions_recursively(
                definitions: &mut alloc::collections::btree_map::BTreeMap<
                    borsh1::schema::Declaration,
                    Definition,
                >,
            ) {
                definitions.insert(
                    Self::declaration(),
                    Definition::Struct {
                        fields: Fields::Empty,
                    },
                );
            }

            fn declaration() -> borsh1::schema::Declaration {
                "MathErrorsDivisionByZero".to_string()
            }
        }

        impl BorshSchema for MathErrors {
            fn add_definitions_recursively(
                definitions: &mut alloc::collections::btree_map::BTreeMap<
                    borsh1::schema::Declaration,
                    borsh1::schema::Definition,
                >,
            ) {
                MathErrorsInvalidInput::add_definitions_recursively(definitions);
                MathErrorsOverflow::add_definitions_recursively(definitions);
                MathErrorsUnderflow::add_definitions_recursively(definitions);
                MathErrorsDivisionByZero::add_definitions_recursively(definitions);

                let definition = Definition::Enum {
                    tag_width: 1,
                    variants: alloc::vec![
                        (
                            0,
                            "InvalidInput".to_string(),
                            <MathErrorsInvalidInput as BorshSchema>::declaration()
                        ),
                        (
                            1,
                            "Overflow".to_string(),
                            <MathErrorsOverflow as BorshSchema>::declaration()
                        ),
                        (
                            2,
                            "Underflow".to_string(),
                            <MathErrorsUnderflow as BorshSchema>::declaration()
                        ),
                        (
                            3,
                            "DivisionByZero".to_string(),
                            <MathErrorsDivisionByZero as BorshSchema>::declaration()
                        ),
                    ],
                };

                match definitions.entry(Self::declaration()) {
                    alloc::collections::btree_map::Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(definition);
                    }
                    alloc::collections::btree_map::Entry::Occupied(occupied_entry) => {
                        let other = occupied_entry.get();
                        assert_eq!(&definition, other, "definition must be same shape");
                    }
                }
            }

            fn declaration() -> borsh1::schema::Declaration {
                "MathErrors".to_string()
            }
        }
    }
}

#[cfg(feature = "protobuf3")]
mod protobuf3_impl {
    use crate::*;
    use protobuf3::Enum;

    #[allow(clippy::derivable_impls)]
    impl Default for MathErrors {
        fn default() -> Self {
            MathErrors::InvalidInput
        }
    }

    impl Enum for MathErrors {
        const NAME: &'static str = "MathErrors";

        fn value(&self) -> i32 {
            self.to_primitive().into()
        }

        fn from_i32(v: i32) -> Option<Self> {
            match v {
                0 => Some(Self::InvalidInput),
                2 => Some(Self::Overflow),
                3 => Some(Self::Underflow),
                4 => Some(Self::DivisionByZero),
                _ => None,
            }
        }

        fn from_str(s: &str) -> Option<Self> {
            match s {
                "InvalidInput" => Some(Self::InvalidInput),
                "Overflow" => Some(Self::Overflow),
                "Underflow" => Some(Self::Underflow),
                "DivisionByZero" => Some(Self::DivisionByZero),
                _ => None,
            }
        }
        const VALUES: &'static [Self] = &[
            Self::InvalidInput,
            Self::Overflow,
            Self::Underflow,
            Self::DivisionByZero,
        ];
    }
}

#[cfg(feature = "parity-scale-codec3")]
mod parity_scale_codec3_impl {
    use super::*;
    use parity_scale_codec3::Encode;

    impl Encode for MathErrors {
        fn size_hint(&self) -> usize {
            1
        }

        fn encode_to<T: parity_scale_codec3::Output + ?Sized>(&self, dest: &mut T) {
            dest.push_byte(self.to_primitive());
        }

        fn encoded_size(&self) -> usize {
            1
        }
    }
}

#[cfg(feature = "proptest1")]
mod proptest1_impl {
    use super::*;
    use proptest1::{
        prelude::{Arbitrary, BoxedStrategy, Just, Strategy},
        prop_oneof,
    };

    impl Arbitrary for MathErrors {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            prop_oneof![
                Just(Self::Overflow),
                Just(Self::Underflow),
                Just(Self::DivisionByZero),
            ]
            .boxed()
        }
    }
}

#[cfg(feature = "arbitrary1")]
mod arbitrary1_impl {
    use super::*;
    use arbitrary1::*;

    impl<'a> Arbitrary<'a> for MathErrors {
        fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
            let value = u.int_in_range(0..=3)?;
            Ok(Self::try_from_primitive(value).expect("with in range"))
        }
    }
}

#[cfg(feature = "arbitrary-int1")]
mod arbitrary_int1_impl {
    use super::*;
    use arbitrary_int1::u2;

    impl MathErrors {
        pub const fn from_u2(value: u2) -> Self {
            match value.value() {
                0 => MathErrors::InvalidInput,
                1 => MathErrors::Overflow,
                2 => MathErrors::Underflow,
                _ => MathErrors::DivisionByZero,                
            }
        }

        pub const fn to_primitive_u2(&self) -> u2 {
            match self {
                MathErrors::InvalidInput => u2::new(0),
                MathErrors::Overflow => u2::new(1),
                MathErrors::Underflow => u2::new(2),
                MathErrors::DivisionByZero => u2::new(3),
            }
        }
    }

    impl Into<u2> for MathErrors {
        fn into(self) -> u2 {
            self.to_primitive_u2()
        }
    }

    impl From<u2> for MathErrors {
        fn from(value: u2) -> Self {
            Self::from_u2(value)
        }
    }
}
