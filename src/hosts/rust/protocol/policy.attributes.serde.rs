// @generated
impl serde::Serialize for AssignKeyAccessServerToAttributeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.attribute_key_access_server.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.AssignKeyAccessServerToAttributeRequest", len)?;
        if let Some(v) = self.attribute_key_access_server.as_ref() {
            struct_ser.serialize_field("attributeKeyAccessServer", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssignKeyAccessServerToAttributeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attribute_key_access_server",
            "attributeKeyAccessServer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AttributeKeyAccessServer,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "attributeKeyAccessServer" | "attribute_key_access_server" => Ok(GeneratedField::AttributeKeyAccessServer),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssignKeyAccessServerToAttributeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.AssignKeyAccessServerToAttributeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AssignKeyAccessServerToAttributeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attribute_key_access_server__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AttributeKeyAccessServer => {
                            if attribute_key_access_server__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeKeyAccessServer"));
                            }
                            attribute_key_access_server__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AssignKeyAccessServerToAttributeRequest {
                    attribute_key_access_server: attribute_key_access_server__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.AssignKeyAccessServerToAttributeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AssignKeyAccessServerToAttributeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.attribute_key_access_server.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.AssignKeyAccessServerToAttributeResponse", len)?;
        if let Some(v) = self.attribute_key_access_server.as_ref() {
            struct_ser.serialize_field("attributeKeyAccessServer", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssignKeyAccessServerToAttributeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attribute_key_access_server",
            "attributeKeyAccessServer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AttributeKeyAccessServer,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "attributeKeyAccessServer" | "attribute_key_access_server" => Ok(GeneratedField::AttributeKeyAccessServer),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssignKeyAccessServerToAttributeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.AssignKeyAccessServerToAttributeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AssignKeyAccessServerToAttributeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attribute_key_access_server__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AttributeKeyAccessServer => {
                            if attribute_key_access_server__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeKeyAccessServer"));
                            }
                            attribute_key_access_server__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AssignKeyAccessServerToAttributeResponse {
                    attribute_key_access_server: attribute_key_access_server__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.AssignKeyAccessServerToAttributeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AssignKeyAccessServerToValueRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value_key_access_server.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.AssignKeyAccessServerToValueRequest", len)?;
        if let Some(v) = self.value_key_access_server.as_ref() {
            struct_ser.serialize_field("valueKeyAccessServer", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssignKeyAccessServerToValueRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value_key_access_server",
            "valueKeyAccessServer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValueKeyAccessServer,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "valueKeyAccessServer" | "value_key_access_server" => Ok(GeneratedField::ValueKeyAccessServer),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssignKeyAccessServerToValueRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.AssignKeyAccessServerToValueRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AssignKeyAccessServerToValueRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value_key_access_server__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ValueKeyAccessServer => {
                            if value_key_access_server__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueKeyAccessServer"));
                            }
                            value_key_access_server__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AssignKeyAccessServerToValueRequest {
                    value_key_access_server: value_key_access_server__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.AssignKeyAccessServerToValueRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AssignKeyAccessServerToValueResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value_key_access_server.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.AssignKeyAccessServerToValueResponse", len)?;
        if let Some(v) = self.value_key_access_server.as_ref() {
            struct_ser.serialize_field("valueKeyAccessServer", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssignKeyAccessServerToValueResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value_key_access_server",
            "valueKeyAccessServer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValueKeyAccessServer,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "valueKeyAccessServer" | "value_key_access_server" => Ok(GeneratedField::ValueKeyAccessServer),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssignKeyAccessServerToValueResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.AssignKeyAccessServerToValueResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AssignKeyAccessServerToValueResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value_key_access_server__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ValueKeyAccessServer => {
                            if value_key_access_server__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueKeyAccessServer"));
                            }
                            value_key_access_server__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AssignKeyAccessServerToValueResponse {
                    value_key_access_server: value_key_access_server__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.AssignKeyAccessServerToValueResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AssignPublicKeyToAttributeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.attribute_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.AssignPublicKeyToAttributeRequest", len)?;
        if let Some(v) = self.attribute_key.as_ref() {
            struct_ser.serialize_field("attributeKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssignPublicKeyToAttributeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attribute_key",
            "attributeKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AttributeKey,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "attributeKey" | "attribute_key" => Ok(GeneratedField::AttributeKey),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssignPublicKeyToAttributeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.AssignPublicKeyToAttributeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AssignPublicKeyToAttributeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attribute_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AttributeKey => {
                            if attribute_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeKey"));
                            }
                            attribute_key__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AssignPublicKeyToAttributeRequest {
                    attribute_key: attribute_key__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.AssignPublicKeyToAttributeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AssignPublicKeyToAttributeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.attribute_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.AssignPublicKeyToAttributeResponse", len)?;
        if let Some(v) = self.attribute_key.as_ref() {
            struct_ser.serialize_field("attributeKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssignPublicKeyToAttributeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attribute_key",
            "attributeKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AttributeKey,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "attributeKey" | "attribute_key" => Ok(GeneratedField::AttributeKey),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssignPublicKeyToAttributeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.AssignPublicKeyToAttributeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AssignPublicKeyToAttributeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attribute_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AttributeKey => {
                            if attribute_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeKey"));
                            }
                            attribute_key__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AssignPublicKeyToAttributeResponse {
                    attribute_key: attribute_key__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.AssignPublicKeyToAttributeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AssignPublicKeyToValueRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.AssignPublicKeyToValueRequest", len)?;
        if let Some(v) = self.value_key.as_ref() {
            struct_ser.serialize_field("valueKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssignPublicKeyToValueRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value_key",
            "valueKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValueKey,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "valueKey" | "value_key" => Ok(GeneratedField::ValueKey),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssignPublicKeyToValueRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.AssignPublicKeyToValueRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AssignPublicKeyToValueRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ValueKey => {
                            if value_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueKey"));
                            }
                            value_key__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AssignPublicKeyToValueRequest {
                    value_key: value_key__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.AssignPublicKeyToValueRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AssignPublicKeyToValueResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.AssignPublicKeyToValueResponse", len)?;
        if let Some(v) = self.value_key.as_ref() {
            struct_ser.serialize_field("valueKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssignPublicKeyToValueResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value_key",
            "valueKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValueKey,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "valueKey" | "value_key" => Ok(GeneratedField::ValueKey),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssignPublicKeyToValueResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.AssignPublicKeyToValueResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AssignPublicKeyToValueResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ValueKey => {
                            if value_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueKey"));
                            }
                            value_key__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AssignPublicKeyToValueResponse {
                    value_key: value_key__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.AssignPublicKeyToValueResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AttributeKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.attribute_id.is_empty() {
            len += 1;
        }
        if !self.key_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.AttributeKey", len)?;
        if !self.attribute_id.is_empty() {
            struct_ser.serialize_field("attributeId", &self.attribute_id)?;
        }
        if !self.key_id.is_empty() {
            struct_ser.serialize_field("keyId", &self.key_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AttributeKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attribute_id",
            "attributeId",
            "key_id",
            "keyId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AttributeId,
            KeyId,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "attributeId" | "attribute_id" => Ok(GeneratedField::AttributeId),
                            "keyId" | "key_id" => Ok(GeneratedField::KeyId),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AttributeKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.AttributeKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AttributeKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attribute_id__ = None;
                let mut key_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AttributeId => {
                            if attribute_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeId"));
                            }
                            attribute_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KeyId => {
                            if key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyId"));
                            }
                            key_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AttributeKey {
                    attribute_id: attribute_id__.unwrap_or_default(),
                    key_id: key_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.AttributeKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AttributeKeyAccessServer {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.attribute_id.is_empty() {
            len += 1;
        }
        if !self.key_access_server_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.AttributeKeyAccessServer", len)?;
        if !self.attribute_id.is_empty() {
            struct_ser.serialize_field("attributeId", &self.attribute_id)?;
        }
        if !self.key_access_server_id.is_empty() {
            struct_ser.serialize_field("keyAccessServerId", &self.key_access_server_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AttributeKeyAccessServer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attribute_id",
            "attributeId",
            "key_access_server_id",
            "keyAccessServerId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AttributeId,
            KeyAccessServerId,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "attributeId" | "attribute_id" => Ok(GeneratedField::AttributeId),
                            "keyAccessServerId" | "key_access_server_id" => Ok(GeneratedField::KeyAccessServerId),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AttributeKeyAccessServer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.AttributeKeyAccessServer")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AttributeKeyAccessServer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attribute_id__ = None;
                let mut key_access_server_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AttributeId => {
                            if attribute_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeId"));
                            }
                            attribute_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KeyAccessServerId => {
                            if key_access_server_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyAccessServerId"));
                            }
                            key_access_server_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AttributeKeyAccessServer {
                    attribute_id: attribute_id__.unwrap_or_default(),
                    key_access_server_id: key_access_server_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.AttributeKeyAccessServer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AttributeValueObligationTriggerRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.obligation_value.is_some() {
            len += 1;
        }
        if self.action.is_some() {
            len += 1;
        }
        if self.context.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.AttributeValueObligationTriggerRequest", len)?;
        if let Some(v) = self.obligation_value.as_ref() {
            struct_ser.serialize_field("obligationValue", v)?;
        }
        if let Some(v) = self.action.as_ref() {
            struct_ser.serialize_field("action", v)?;
        }
        if let Some(v) = self.context.as_ref() {
            struct_ser.serialize_field("context", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AttributeValueObligationTriggerRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "obligation_value",
            "obligationValue",
            "action",
            "context",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ObligationValue,
            Action,
            Context,
            Metadata,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "obligationValue" | "obligation_value" => Ok(GeneratedField::ObligationValue),
                            "action" => Ok(GeneratedField::Action),
                            "context" => Ok(GeneratedField::Context),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AttributeValueObligationTriggerRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.AttributeValueObligationTriggerRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AttributeValueObligationTriggerRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut obligation_value__ = None;
                let mut action__ = None;
                let mut context__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ObligationValue => {
                            if obligation_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("obligationValue"));
                            }
                            obligation_value__ = map_.next_value()?;
                        }
                        GeneratedField::Action => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            action__ = map_.next_value()?;
                        }
                        GeneratedField::Context => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            context__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AttributeValueObligationTriggerRequest {
                    obligation_value: obligation_value__,
                    action: action__,
                    context: context__,
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.AttributeValueObligationTriggerRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AttributeValueSubjectMappingRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.actions.is_empty() {
            len += 1;
        }
        if !self.existing_subject_condition_set_id.is_empty() {
            len += 1;
        }
        if self.new_subject_condition_set.is_some() {
            len += 1;
        }
        if !self.namespace_id.is_empty() {
            len += 1;
        }
        if !self.namespace_fqn.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.AttributeValueSubjectMappingRequest", len)?;
        if !self.actions.is_empty() {
            struct_ser.serialize_field("actions", &self.actions)?;
        }
        if !self.existing_subject_condition_set_id.is_empty() {
            struct_ser.serialize_field("existingSubjectConditionSetId", &self.existing_subject_condition_set_id)?;
        }
        if let Some(v) = self.new_subject_condition_set.as_ref() {
            struct_ser.serialize_field("newSubjectConditionSet", v)?;
        }
        if !self.namespace_id.is_empty() {
            struct_ser.serialize_field("namespaceId", &self.namespace_id)?;
        }
        if !self.namespace_fqn.is_empty() {
            struct_ser.serialize_field("namespaceFqn", &self.namespace_fqn)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AttributeValueSubjectMappingRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "actions",
            "existing_subject_condition_set_id",
            "existingSubjectConditionSetId",
            "new_subject_condition_set",
            "newSubjectConditionSet",
            "namespace_id",
            "namespaceId",
            "namespace_fqn",
            "namespaceFqn",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Actions,
            ExistingSubjectConditionSetId,
            NewSubjectConditionSet,
            NamespaceId,
            NamespaceFqn,
            Metadata,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "actions" => Ok(GeneratedField::Actions),
                            "existingSubjectConditionSetId" | "existing_subject_condition_set_id" => Ok(GeneratedField::ExistingSubjectConditionSetId),
                            "newSubjectConditionSet" | "new_subject_condition_set" => Ok(GeneratedField::NewSubjectConditionSet),
                            "namespaceId" | "namespace_id" => Ok(GeneratedField::NamespaceId),
                            "namespaceFqn" | "namespace_fqn" => Ok(GeneratedField::NamespaceFqn),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AttributeValueSubjectMappingRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.AttributeValueSubjectMappingRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AttributeValueSubjectMappingRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut actions__ = None;
                let mut existing_subject_condition_set_id__ = None;
                let mut new_subject_condition_set__ = None;
                let mut namespace_id__ = None;
                let mut namespace_fqn__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Actions => {
                            if actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            actions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExistingSubjectConditionSetId => {
                            if existing_subject_condition_set_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("existingSubjectConditionSetId"));
                            }
                            existing_subject_condition_set_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewSubjectConditionSet => {
                            if new_subject_condition_set__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newSubjectConditionSet"));
                            }
                            new_subject_condition_set__ = map_.next_value()?;
                        }
                        GeneratedField::NamespaceId => {
                            if namespace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaceId"));
                            }
                            namespace_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NamespaceFqn => {
                            if namespace_fqn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaceFqn"));
                            }
                            namespace_fqn__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AttributeValueSubjectMappingRequest {
                    actions: actions__.unwrap_or_default(),
                    existing_subject_condition_set_id: existing_subject_condition_set_id__.unwrap_or_default(),
                    new_subject_condition_set: new_subject_condition_set__,
                    namespace_id: namespace_id__.unwrap_or_default(),
                    namespace_fqn: namespace_fqn__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.AttributeValueSubjectMappingRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AttributesSort {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.field != 0 {
            len += 1;
        }
        if self.direction != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.AttributesSort", len)?;
        if self.field != 0 {
            let v = SortAttributesType::try_from(self.field)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.field)))?;
            struct_ser.serialize_field("field", &v)?;
        }
        if self.direction != 0 {
            let v = super::SortDirection::try_from(self.direction)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.direction)))?;
            struct_ser.serialize_field("direction", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AttributesSort {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "field",
            "direction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Field,
            Direction,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "field" => Ok(GeneratedField::Field),
                            "direction" => Ok(GeneratedField::Direction),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AttributesSort;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.AttributesSort")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AttributesSort, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut field__ = None;
                let mut direction__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Field => {
                            if field__.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            field__ = Some(map_.next_value::<SortAttributesType>()? as i32);
                        }
                        GeneratedField::Direction => {
                            if direction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("direction"));
                            }
                            direction__ = Some(map_.next_value::<super::SortDirection>()? as i32);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AttributesSort {
                    field: field__.unwrap_or_default(),
                    direction: direction__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.AttributesSort", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateAttributeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.namespace_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.rule != 0 {
            len += 1;
        }
        if !self.values.is_empty() {
            len += 1;
        }
        if self.allow_traversal.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.CreateAttributeRequest", len)?;
        if !self.namespace_id.is_empty() {
            struct_ser.serialize_field("namespaceId", &self.namespace_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.rule != 0 {
            let v = super::AttributeRuleTypeEnum::try_from(self.rule)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.rule)))?;
            struct_ser.serialize_field("rule", &v)?;
        }
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        if let Some(v) = self.allow_traversal.as_ref() {
            struct_ser.serialize_field("allowTraversal", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateAttributeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "namespace_id",
            "namespaceId",
            "name",
            "rule",
            "values",
            "allow_traversal",
            "allowTraversal",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NamespaceId,
            Name,
            Rule,
            Values,
            AllowTraversal,
            Metadata,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "namespaceId" | "namespace_id" => Ok(GeneratedField::NamespaceId),
                            "name" => Ok(GeneratedField::Name),
                            "rule" => Ok(GeneratedField::Rule),
                            "values" => Ok(GeneratedField::Values),
                            "allowTraversal" | "allow_traversal" => Ok(GeneratedField::AllowTraversal),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateAttributeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.CreateAttributeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateAttributeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut namespace_id__ = None;
                let mut name__ = None;
                let mut rule__ = None;
                let mut values__ = None;
                let mut allow_traversal__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NamespaceId => {
                            if namespace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaceId"));
                            }
                            namespace_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Rule => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rule"));
                            }
                            rule__ = Some(map_.next_value::<super::AttributeRuleTypeEnum>()? as i32);
                        }
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllowTraversal => {
                            if allow_traversal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowTraversal"));
                            }
                            allow_traversal__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(CreateAttributeRequest {
                    namespace_id: namespace_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    rule: rule__.unwrap_or_default(),
                    values: values__.unwrap_or_default(),
                    allow_traversal: allow_traversal__,
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.CreateAttributeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateAttributeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.attribute.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.CreateAttributeResponse", len)?;
        if let Some(v) = self.attribute.as_ref() {
            struct_ser.serialize_field("attribute", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateAttributeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attribute",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Attribute,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "attribute" => Ok(GeneratedField::Attribute),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateAttributeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.CreateAttributeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateAttributeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attribute__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Attribute => {
                            if attribute__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attribute"));
                            }
                            attribute__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(CreateAttributeResponse {
                    attribute: attribute__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.CreateAttributeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateAttributeValueRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.attribute_id.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        if !self.obligation_triggers.is_empty() {
            len += 1;
        }
        if !self.subject_mappings.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.CreateAttributeValueRequest", len)?;
        if !self.attribute_id.is_empty() {
            struct_ser.serialize_field("attributeId", &self.attribute_id)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        if !self.obligation_triggers.is_empty() {
            struct_ser.serialize_field("obligationTriggers", &self.obligation_triggers)?;
        }
        if !self.subject_mappings.is_empty() {
            struct_ser.serialize_field("subjectMappings", &self.subject_mappings)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateAttributeValueRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attribute_id",
            "attributeId",
            "value",
            "obligation_triggers",
            "obligationTriggers",
            "subject_mappings",
            "subjectMappings",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AttributeId,
            Value,
            ObligationTriggers,
            SubjectMappings,
            Metadata,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "attributeId" | "attribute_id" => Ok(GeneratedField::AttributeId),
                            "value" => Ok(GeneratedField::Value),
                            "obligationTriggers" | "obligation_triggers" => Ok(GeneratedField::ObligationTriggers),
                            "subjectMappings" | "subject_mappings" => Ok(GeneratedField::SubjectMappings),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateAttributeValueRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.CreateAttributeValueRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateAttributeValueRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attribute_id__ = None;
                let mut value__ = None;
                let mut obligation_triggers__ = None;
                let mut subject_mappings__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AttributeId => {
                            if attribute_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeId"));
                            }
                            attribute_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ObligationTriggers => {
                            if obligation_triggers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("obligationTriggers"));
                            }
                            obligation_triggers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SubjectMappings => {
                            if subject_mappings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectMappings"));
                            }
                            subject_mappings__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(CreateAttributeValueRequest {
                    attribute_id: attribute_id__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                    obligation_triggers: obligation_triggers__.unwrap_or_default(),
                    subject_mappings: subject_mappings__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.CreateAttributeValueRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateAttributeValueResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.CreateAttributeValueResponse", len)?;
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateAttributeValueResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "value" => Ok(GeneratedField::Value),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateAttributeValueResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.CreateAttributeValueResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateAttributeValueResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(CreateAttributeValueResponse {
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.CreateAttributeValueResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeactivateAttributeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.DeactivateAttributeRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeactivateAttributeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeactivateAttributeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.DeactivateAttributeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeactivateAttributeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DeactivateAttributeRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.DeactivateAttributeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeactivateAttributeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.attribute.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.DeactivateAttributeResponse", len)?;
        if let Some(v) = self.attribute.as_ref() {
            struct_ser.serialize_field("attribute", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeactivateAttributeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attribute",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Attribute,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "attribute" => Ok(GeneratedField::Attribute),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeactivateAttributeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.DeactivateAttributeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeactivateAttributeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attribute__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Attribute => {
                            if attribute__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attribute"));
                            }
                            attribute__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DeactivateAttributeResponse {
                    attribute: attribute__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.DeactivateAttributeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeactivateAttributeValueRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.DeactivateAttributeValueRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeactivateAttributeValueRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeactivateAttributeValueRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.DeactivateAttributeValueRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeactivateAttributeValueRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DeactivateAttributeValueRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.DeactivateAttributeValueRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeactivateAttributeValueResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.DeactivateAttributeValueResponse", len)?;
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeactivateAttributeValueResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "value" => Ok(GeneratedField::Value),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeactivateAttributeValueResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.DeactivateAttributeValueResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeactivateAttributeValueResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DeactivateAttributeValueResponse {
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.DeactivateAttributeValueResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAttributeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.identifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.GetAttributeRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.identifier.as_ref() {
            match v {
                get_attribute_request::Identifier::AttributeId(v) => {
                    struct_ser.serialize_field("attributeId", v)?;
                }
                get_attribute_request::Identifier::Fqn(v) => {
                    struct_ser.serialize_field("fqn", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAttributeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "attribute_id",
            "attributeId",
            "fqn",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            AttributeId,
            Fqn,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "attributeId" | "attribute_id" => Ok(GeneratedField::AttributeId),
                            "fqn" => Ok(GeneratedField::Fqn),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAttributeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.GetAttributeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetAttributeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut identifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AttributeId => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeId"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(get_attribute_request::Identifier::AttributeId);
                        }
                        GeneratedField::Fqn => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fqn"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(get_attribute_request::Identifier::Fqn);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetAttributeRequest {
                    id: id__.unwrap_or_default(),
                    identifier: identifier__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.GetAttributeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAttributeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.attribute.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.GetAttributeResponse", len)?;
        if let Some(v) = self.attribute.as_ref() {
            struct_ser.serialize_field("attribute", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAttributeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attribute",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Attribute,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "attribute" => Ok(GeneratedField::Attribute),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAttributeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.GetAttributeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetAttributeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attribute__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Attribute => {
                            if attribute__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attribute"));
                            }
                            attribute__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetAttributeResponse {
                    attribute: attribute__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.GetAttributeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAttributeValueRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.identifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.GetAttributeValueRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.identifier.as_ref() {
            match v {
                get_attribute_value_request::Identifier::ValueId(v) => {
                    struct_ser.serialize_field("valueId", v)?;
                }
                get_attribute_value_request::Identifier::Fqn(v) => {
                    struct_ser.serialize_field("fqn", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAttributeValueRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "value_id",
            "valueId",
            "fqn",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ValueId,
            Fqn,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "valueId" | "value_id" => Ok(GeneratedField::ValueId),
                            "fqn" => Ok(GeneratedField::Fqn),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAttributeValueRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.GetAttributeValueRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetAttributeValueRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut identifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValueId => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueId"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(get_attribute_value_request::Identifier::ValueId);
                        }
                        GeneratedField::Fqn => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fqn"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(get_attribute_value_request::Identifier::Fqn);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetAttributeValueRequest {
                    id: id__.unwrap_or_default(),
                    identifier: identifier__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.GetAttributeValueRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAttributeValueResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.GetAttributeValueResponse", len)?;
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAttributeValueResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "value" => Ok(GeneratedField::Value),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAttributeValueResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.GetAttributeValueResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetAttributeValueResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetAttributeValueResponse {
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.GetAttributeValueResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAttributeValuesByFqnsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.fqns.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.GetAttributeValuesByFqnsRequest", len)?;
        if !self.fqns.is_empty() {
            struct_ser.serialize_field("fqns", &self.fqns)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAttributeValuesByFqnsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fqns",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Fqns,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "fqns" => Ok(GeneratedField::Fqns),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAttributeValuesByFqnsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.GetAttributeValuesByFqnsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetAttributeValuesByFqnsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fqns__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Fqns => {
                            if fqns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fqns"));
                            }
                            fqns__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetAttributeValuesByFqnsRequest {
                    fqns: fqns__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.GetAttributeValuesByFqnsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAttributeValuesByFqnsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.fqn_attribute_values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.GetAttributeValuesByFqnsResponse", len)?;
        if !self.fqn_attribute_values.is_empty() {
            struct_ser.serialize_field("fqnAttributeValues", &self.fqn_attribute_values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAttributeValuesByFqnsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fqn_attribute_values",
            "fqnAttributeValues",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FqnAttributeValues,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "fqnAttributeValues" | "fqn_attribute_values" => Ok(GeneratedField::FqnAttributeValues),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAttributeValuesByFqnsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.GetAttributeValuesByFqnsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetAttributeValuesByFqnsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fqn_attribute_values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FqnAttributeValues => {
                            if fqn_attribute_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fqnAttributeValues"));
                            }
                            fqn_attribute_values__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetAttributeValuesByFqnsResponse {
                    fqn_attribute_values: fqn_attribute_values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.GetAttributeValuesByFqnsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for get_attribute_values_by_fqns_response::AttributeAndValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.attribute.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.GetAttributeValuesByFqnsResponse.AttributeAndValue", len)?;
        if let Some(v) = self.attribute.as_ref() {
            struct_ser.serialize_field("attribute", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for get_attribute_values_by_fqns_response::AttributeAndValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attribute",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Attribute,
            Value,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "attribute" => Ok(GeneratedField::Attribute),
                            "value" => Ok(GeneratedField::Value),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = get_attribute_values_by_fqns_response::AttributeAndValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.GetAttributeValuesByFqnsResponse.AttributeAndValue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<get_attribute_values_by_fqns_response::AttributeAndValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attribute__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Attribute => {
                            if attribute__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attribute"));
                            }
                            attribute__ = map_.next_value()?;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(get_attribute_values_by_fqns_response::AttributeAndValue {
                    attribute: attribute__,
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.GetAttributeValuesByFqnsResponse.AttributeAndValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetEntitleableAttributesByFqnsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.fqns.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.GetEntitleableAttributesByFqnsRequest", len)?;
        if !self.fqns.is_empty() {
            struct_ser.serialize_field("fqns", &self.fqns)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetEntitleableAttributesByFqnsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fqns",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Fqns,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "fqns" => Ok(GeneratedField::Fqns),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetEntitleableAttributesByFqnsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.GetEntitleableAttributesByFqnsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetEntitleableAttributesByFqnsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fqns__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Fqns => {
                            if fqns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fqns"));
                            }
                            fqns__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetEntitleableAttributesByFqnsRequest {
                    fqns: fqns__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.GetEntitleableAttributesByFqnsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetEntitleableAttributesByFqnsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.definitions.is_empty() {
            len += 1;
        }
        if !self.fqn_entitleable_attributes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.GetEntitleableAttributesByFqnsResponse", len)?;
        if !self.definitions.is_empty() {
            struct_ser.serialize_field("definitions", &self.definitions)?;
        }
        if !self.fqn_entitleable_attributes.is_empty() {
            struct_ser.serialize_field("fqnEntitleableAttributes", &self.fqn_entitleable_attributes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetEntitleableAttributesByFqnsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "definitions",
            "fqn_entitleable_attributes",
            "fqnEntitleableAttributes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Definitions,
            FqnEntitleableAttributes,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "definitions" => Ok(GeneratedField::Definitions),
                            "fqnEntitleableAttributes" | "fqn_entitleable_attributes" => Ok(GeneratedField::FqnEntitleableAttributes),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetEntitleableAttributesByFqnsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.GetEntitleableAttributesByFqnsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetEntitleableAttributesByFqnsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut definitions__ = None;
                let mut fqn_entitleable_attributes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Definitions => {
                            if definitions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("definitions"));
                            }
                            definitions__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::FqnEntitleableAttributes => {
                            if fqn_entitleable_attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fqnEntitleableAttributes"));
                            }
                            fqn_entitleable_attributes__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetEntitleableAttributesByFqnsResponse {
                    definitions: definitions__.unwrap_or_default(),
                    fqn_entitleable_attributes: fqn_entitleable_attributes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.GetEntitleableAttributesByFqnsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for get_entitleable_attributes_by_fqns_response::EntitleableAttribute {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.definition_fqn.is_empty() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.GetEntitleableAttributesByFqnsResponse.EntitleableAttribute", len)?;
        if !self.definition_fqn.is_empty() {
            struct_ser.serialize_field("definitionFqn", &self.definition_fqn)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for get_entitleable_attributes_by_fqns_response::EntitleableAttribute {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "definition_fqn",
            "definitionFqn",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DefinitionFqn,
            Value,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "definitionFqn" | "definition_fqn" => Ok(GeneratedField::DefinitionFqn),
                            "value" => Ok(GeneratedField::Value),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = get_entitleable_attributes_by_fqns_response::EntitleableAttribute;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.GetEntitleableAttributesByFqnsResponse.EntitleableAttribute")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<get_entitleable_attributes_by_fqns_response::EntitleableAttribute, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut definition_fqn__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DefinitionFqn => {
                            if definition_fqn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("definitionFqn"));
                            }
                            definition_fqn__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(get_entitleable_attributes_by_fqns_response::EntitleableAttribute {
                    definition_fqn: definition_fqn__.unwrap_or_default(),
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.GetEntitleableAttributesByFqnsResponse.EntitleableAttribute", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for get_entitleable_attributes_by_fqns_response::EntitleableDefinition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rule != 0 {
            len += 1;
        }
        if !self.values.is_empty() {
            len += 1;
        }
        if self.namespace.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.GetEntitleableAttributesByFqnsResponse.EntitleableDefinition", len)?;
        if self.rule != 0 {
            let v = super::AttributeRuleTypeEnum::try_from(self.rule)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.rule)))?;
            struct_ser.serialize_field("rule", &v)?;
        }
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        if let Some(v) = self.namespace.as_ref() {
            struct_ser.serialize_field("namespace", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for get_entitleable_attributes_by_fqns_response::EntitleableDefinition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule",
            "values",
            "namespace",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rule,
            Values,
            Namespace,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rule" => Ok(GeneratedField::Rule),
                            "values" => Ok(GeneratedField::Values),
                            "namespace" => Ok(GeneratedField::Namespace),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = get_entitleable_attributes_by_fqns_response::EntitleableDefinition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.GetEntitleableAttributesByFqnsResponse.EntitleableDefinition")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<get_entitleable_attributes_by_fqns_response::EntitleableDefinition, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule__ = None;
                let mut values__ = None;
                let mut namespace__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rule => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rule"));
                            }
                            rule__ = Some(map_.next_value::<super::AttributeRuleTypeEnum>()? as i32);
                        }
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Namespace => {
                            if namespace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespace"));
                            }
                            namespace__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(get_entitleable_attributes_by_fqns_response::EntitleableDefinition {
                    rule: rule__.unwrap_or_default(),
                    values: values__.unwrap_or_default(),
                    namespace: namespace__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.GetEntitleableAttributesByFqnsResponse.EntitleableDefinition", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for get_entitleable_attributes_by_fqns_response::EntitleableValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.fqn.is_empty() {
            len += 1;
        }
        if !self.value_id.is_empty() {
            len += 1;
        }
        if !self.subject_mappings.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.GetEntitleableAttributesByFqnsResponse.EntitleableValue", len)?;
        if !self.fqn.is_empty() {
            struct_ser.serialize_field("fqn", &self.fqn)?;
        }
        if !self.value_id.is_empty() {
            struct_ser.serialize_field("valueId", &self.value_id)?;
        }
        if !self.subject_mappings.is_empty() {
            struct_ser.serialize_field("subjectMappings", &self.subject_mappings)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for get_entitleable_attributes_by_fqns_response::EntitleableValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fqn",
            "value_id",
            "valueId",
            "subject_mappings",
            "subjectMappings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Fqn,
            ValueId,
            SubjectMappings,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "fqn" => Ok(GeneratedField::Fqn),
                            "valueId" | "value_id" => Ok(GeneratedField::ValueId),
                            "subjectMappings" | "subject_mappings" => Ok(GeneratedField::SubjectMappings),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = get_entitleable_attributes_by_fqns_response::EntitleableValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.GetEntitleableAttributesByFqnsResponse.EntitleableValue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<get_entitleable_attributes_by_fqns_response::EntitleableValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fqn__ = None;
                let mut value_id__ = None;
                let mut subject_mappings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Fqn => {
                            if fqn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fqn"));
                            }
                            fqn__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValueId => {
                            if value_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueId"));
                            }
                            value_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SubjectMappings => {
                            if subject_mappings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectMappings"));
                            }
                            subject_mappings__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(get_entitleable_attributes_by_fqns_response::EntitleableValue {
                    fqn: fqn__.unwrap_or_default(),
                    value_id: value_id__.unwrap_or_default(),
                    subject_mappings: subject_mappings__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.GetEntitleableAttributesByFqnsResponse.EntitleableValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetKeyMappingsByFqnsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.fqns.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.GetKeyMappingsByFqnsRequest", len)?;
        if !self.fqns.is_empty() {
            struct_ser.serialize_field("fqns", &self.fqns)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetKeyMappingsByFqnsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fqns",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Fqns,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "fqns" => Ok(GeneratedField::Fqns),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetKeyMappingsByFqnsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.GetKeyMappingsByFqnsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetKeyMappingsByFqnsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fqns__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Fqns => {
                            if fqns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fqns"));
                            }
                            fqns__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetKeyMappingsByFqnsRequest {
                    fqns: fqns__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.GetKeyMappingsByFqnsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetKeyMappingsByFqnsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.fqn_key_mappings.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.GetKeyMappingsByFqnsResponse", len)?;
        if !self.fqn_key_mappings.is_empty() {
            struct_ser.serialize_field("fqnKeyMappings", &self.fqn_key_mappings)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetKeyMappingsByFqnsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fqn_key_mappings",
            "fqnKeyMappings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FqnKeyMappings,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "fqnKeyMappings" | "fqn_key_mappings" => Ok(GeneratedField::FqnKeyMappings),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetKeyMappingsByFqnsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.GetKeyMappingsByFqnsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetKeyMappingsByFqnsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fqn_key_mappings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FqnKeyMappings => {
                            if fqn_key_mappings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fqnKeyMappings"));
                            }
                            fqn_key_mappings__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetKeyMappingsByFqnsResponse {
                    fqn_key_mappings: fqn_key_mappings__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.GetKeyMappingsByFqnsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for get_key_mappings_by_fqns_response::AttributeKeyMapping {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rule != 0 {
            len += 1;
        }
        if !self.keys.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.GetKeyMappingsByFqnsResponse.AttributeKeyMapping", len)?;
        if self.rule != 0 {
            let v = super::AttributeRuleTypeEnum::try_from(self.rule)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.rule)))?;
            struct_ser.serialize_field("rule", &v)?;
        }
        if !self.keys.is_empty() {
            struct_ser.serialize_field("keys", &self.keys)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for get_key_mappings_by_fqns_response::AttributeKeyMapping {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule",
            "keys",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rule,
            Keys,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rule" => Ok(GeneratedField::Rule),
                            "keys" => Ok(GeneratedField::Keys),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = get_key_mappings_by_fqns_response::AttributeKeyMapping;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.GetKeyMappingsByFqnsResponse.AttributeKeyMapping")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<get_key_mappings_by_fqns_response::AttributeKeyMapping, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule__ = None;
                let mut keys__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rule => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rule"));
                            }
                            rule__ = Some(map_.next_value::<super::AttributeRuleTypeEnum>()? as i32);
                        }
                        GeneratedField::Keys => {
                            if keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keys"));
                            }
                            keys__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(get_key_mappings_by_fqns_response::AttributeKeyMapping {
                    rule: rule__.unwrap_or_default(),
                    keys: keys__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.GetKeyMappingsByFqnsResponse.AttributeKeyMapping", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAttributeValuesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.attribute_id.is_empty() {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.ListAttributeValuesRequest", len)?;
        if !self.attribute_id.is_empty() {
            struct_ser.serialize_field("attributeId", &self.attribute_id)?;
        }
        if self.state != 0 {
            let v = super::super::common::ActiveStateEnum::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListAttributeValuesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attribute_id",
            "attributeId",
            "state",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AttributeId,
            State,
            Pagination,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "attributeId" | "attribute_id" => Ok(GeneratedField::AttributeId),
                            "state" => Ok(GeneratedField::State),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListAttributeValuesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.ListAttributeValuesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAttributeValuesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attribute_id__ = None;
                let mut state__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AttributeId => {
                            if attribute_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeId"));
                            }
                            attribute_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<super::super::common::ActiveStateEnum>()? as i32);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ListAttributeValuesRequest {
                    attribute_id: attribute_id__.unwrap_or_default(),
                    state: state__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.ListAttributeValuesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAttributeValuesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.values.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.ListAttributeValuesResponse", len)?;
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListAttributeValuesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "values",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Values,
            Pagination,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "values" => Ok(GeneratedField::Values),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListAttributeValuesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.ListAttributeValuesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAttributeValuesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut values__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ListAttributeValuesResponse {
                    values: values__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.ListAttributeValuesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAttributesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.state != 0 {
            len += 1;
        }
        if !self.namespace.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        if !self.sort.is_empty() {
            len += 1;
        }
        if self.search.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.ListAttributesRequest", len)?;
        if self.state != 0 {
            let v = super::super::common::ActiveStateEnum::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if !self.namespace.is_empty() {
            struct_ser.serialize_field("namespace", &self.namespace)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        if !self.sort.is_empty() {
            struct_ser.serialize_field("sort", &self.sort)?;
        }
        if let Some(v) = self.search.as_ref() {
            struct_ser.serialize_field("search", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListAttributesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "state",
            "namespace",
            "pagination",
            "sort",
            "search",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            State,
            Namespace,
            Pagination,
            Sort,
            Search,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "state" => Ok(GeneratedField::State),
                            "namespace" => Ok(GeneratedField::Namespace),
                            "pagination" => Ok(GeneratedField::Pagination),
                            "sort" => Ok(GeneratedField::Sort),
                            "search" => Ok(GeneratedField::Search),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListAttributesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.ListAttributesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAttributesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut state__ = None;
                let mut namespace__ = None;
                let mut pagination__ = None;
                let mut sort__ = None;
                let mut search__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<super::super::common::ActiveStateEnum>()? as i32);
                        }
                        GeneratedField::Namespace => {
                            if namespace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespace"));
                            }
                            namespace__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                        GeneratedField::Sort => {
                            if sort__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sort"));
                            }
                            sort__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Search => {
                            if search__.is_some() {
                                return Err(serde::de::Error::duplicate_field("search"));
                            }
                            search__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ListAttributesRequest {
                    state: state__.unwrap_or_default(),
                    namespace: namespace__.unwrap_or_default(),
                    pagination: pagination__,
                    sort: sort__.unwrap_or_default(),
                    search: search__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.ListAttributesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAttributesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.attributes.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.ListAttributesResponse", len)?;
        if !self.attributes.is_empty() {
            struct_ser.serialize_field("attributes", &self.attributes)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListAttributesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attributes",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Attributes,
            Pagination,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "attributes" => Ok(GeneratedField::Attributes),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListAttributesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.ListAttributesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAttributesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attributes__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Attributes => {
                            if attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributes"));
                            }
                            attributes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ListAttributesResponse {
                    attributes: attributes__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.ListAttributesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveKeyAccessServerFromAttributeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.attribute_key_access_server.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.RemoveKeyAccessServerFromAttributeRequest", len)?;
        if let Some(v) = self.attribute_key_access_server.as_ref() {
            struct_ser.serialize_field("attributeKeyAccessServer", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveKeyAccessServerFromAttributeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attribute_key_access_server",
            "attributeKeyAccessServer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AttributeKeyAccessServer,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "attributeKeyAccessServer" | "attribute_key_access_server" => Ok(GeneratedField::AttributeKeyAccessServer),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoveKeyAccessServerFromAttributeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.RemoveKeyAccessServerFromAttributeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveKeyAccessServerFromAttributeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attribute_key_access_server__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AttributeKeyAccessServer => {
                            if attribute_key_access_server__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeKeyAccessServer"));
                            }
                            attribute_key_access_server__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(RemoveKeyAccessServerFromAttributeRequest {
                    attribute_key_access_server: attribute_key_access_server__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.RemoveKeyAccessServerFromAttributeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveKeyAccessServerFromAttributeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.attribute_key_access_server.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.RemoveKeyAccessServerFromAttributeResponse", len)?;
        if let Some(v) = self.attribute_key_access_server.as_ref() {
            struct_ser.serialize_field("attributeKeyAccessServer", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveKeyAccessServerFromAttributeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attribute_key_access_server",
            "attributeKeyAccessServer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AttributeKeyAccessServer,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "attributeKeyAccessServer" | "attribute_key_access_server" => Ok(GeneratedField::AttributeKeyAccessServer),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoveKeyAccessServerFromAttributeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.RemoveKeyAccessServerFromAttributeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveKeyAccessServerFromAttributeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attribute_key_access_server__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AttributeKeyAccessServer => {
                            if attribute_key_access_server__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeKeyAccessServer"));
                            }
                            attribute_key_access_server__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(RemoveKeyAccessServerFromAttributeResponse {
                    attribute_key_access_server: attribute_key_access_server__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.RemoveKeyAccessServerFromAttributeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveKeyAccessServerFromValueRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value_key_access_server.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.RemoveKeyAccessServerFromValueRequest", len)?;
        if let Some(v) = self.value_key_access_server.as_ref() {
            struct_ser.serialize_field("valueKeyAccessServer", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveKeyAccessServerFromValueRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value_key_access_server",
            "valueKeyAccessServer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValueKeyAccessServer,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "valueKeyAccessServer" | "value_key_access_server" => Ok(GeneratedField::ValueKeyAccessServer),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoveKeyAccessServerFromValueRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.RemoveKeyAccessServerFromValueRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveKeyAccessServerFromValueRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value_key_access_server__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ValueKeyAccessServer => {
                            if value_key_access_server__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueKeyAccessServer"));
                            }
                            value_key_access_server__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(RemoveKeyAccessServerFromValueRequest {
                    value_key_access_server: value_key_access_server__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.RemoveKeyAccessServerFromValueRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveKeyAccessServerFromValueResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value_key_access_server.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.RemoveKeyAccessServerFromValueResponse", len)?;
        if let Some(v) = self.value_key_access_server.as_ref() {
            struct_ser.serialize_field("valueKeyAccessServer", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveKeyAccessServerFromValueResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value_key_access_server",
            "valueKeyAccessServer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValueKeyAccessServer,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "valueKeyAccessServer" | "value_key_access_server" => Ok(GeneratedField::ValueKeyAccessServer),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoveKeyAccessServerFromValueResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.RemoveKeyAccessServerFromValueResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveKeyAccessServerFromValueResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value_key_access_server__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ValueKeyAccessServer => {
                            if value_key_access_server__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueKeyAccessServer"));
                            }
                            value_key_access_server__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(RemoveKeyAccessServerFromValueResponse {
                    value_key_access_server: value_key_access_server__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.RemoveKeyAccessServerFromValueResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemovePublicKeyFromAttributeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.attribute_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.RemovePublicKeyFromAttributeRequest", len)?;
        if let Some(v) = self.attribute_key.as_ref() {
            struct_ser.serialize_field("attributeKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemovePublicKeyFromAttributeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attribute_key",
            "attributeKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AttributeKey,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "attributeKey" | "attribute_key" => Ok(GeneratedField::AttributeKey),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemovePublicKeyFromAttributeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.RemovePublicKeyFromAttributeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemovePublicKeyFromAttributeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attribute_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AttributeKey => {
                            if attribute_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeKey"));
                            }
                            attribute_key__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(RemovePublicKeyFromAttributeRequest {
                    attribute_key: attribute_key__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.RemovePublicKeyFromAttributeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemovePublicKeyFromAttributeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.attribute_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.RemovePublicKeyFromAttributeResponse", len)?;
        if let Some(v) = self.attribute_key.as_ref() {
            struct_ser.serialize_field("attributeKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemovePublicKeyFromAttributeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attribute_key",
            "attributeKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AttributeKey,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "attributeKey" | "attribute_key" => Ok(GeneratedField::AttributeKey),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemovePublicKeyFromAttributeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.RemovePublicKeyFromAttributeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemovePublicKeyFromAttributeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attribute_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AttributeKey => {
                            if attribute_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeKey"));
                            }
                            attribute_key__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(RemovePublicKeyFromAttributeResponse {
                    attribute_key: attribute_key__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.RemovePublicKeyFromAttributeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemovePublicKeyFromValueRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.RemovePublicKeyFromValueRequest", len)?;
        if let Some(v) = self.value_key.as_ref() {
            struct_ser.serialize_field("valueKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemovePublicKeyFromValueRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value_key",
            "valueKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValueKey,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "valueKey" | "value_key" => Ok(GeneratedField::ValueKey),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemovePublicKeyFromValueRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.RemovePublicKeyFromValueRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemovePublicKeyFromValueRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ValueKey => {
                            if value_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueKey"));
                            }
                            value_key__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(RemovePublicKeyFromValueRequest {
                    value_key: value_key__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.RemovePublicKeyFromValueRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemovePublicKeyFromValueResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.RemovePublicKeyFromValueResponse", len)?;
        if let Some(v) = self.value_key.as_ref() {
            struct_ser.serialize_field("valueKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemovePublicKeyFromValueResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value_key",
            "valueKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValueKey,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "valueKey" | "value_key" => Ok(GeneratedField::ValueKey),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemovePublicKeyFromValueResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.RemovePublicKeyFromValueResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemovePublicKeyFromValueResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ValueKey => {
                            if value_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueKey"));
                            }
                            value_key__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(RemovePublicKeyFromValueResponse {
                    value_key: value_key__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.RemovePublicKeyFromValueResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SortAttributesType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SORT_ATTRIBUTES_TYPE_UNSPECIFIED",
            Self::Name => "SORT_ATTRIBUTES_TYPE_NAME",
            Self::CreatedAt => "SORT_ATTRIBUTES_TYPE_CREATED_AT",
            Self::UpdatedAt => "SORT_ATTRIBUTES_TYPE_UPDATED_AT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SortAttributesType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SORT_ATTRIBUTES_TYPE_UNSPECIFIED",
            "SORT_ATTRIBUTES_TYPE_NAME",
            "SORT_ATTRIBUTES_TYPE_CREATED_AT",
            "SORT_ATTRIBUTES_TYPE_UPDATED_AT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SortAttributesType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SORT_ATTRIBUTES_TYPE_UNSPECIFIED" => Ok(SortAttributesType::Unspecified),
                    "SORT_ATTRIBUTES_TYPE_NAME" => Ok(SortAttributesType::Name),
                    "SORT_ATTRIBUTES_TYPE_CREATED_AT" => Ok(SortAttributesType::CreatedAt),
                    "SORT_ATTRIBUTES_TYPE_UPDATED_AT" => Ok(SortAttributesType::UpdatedAt),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateAttributeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        if self.metadata_update_behavior != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.UpdateAttributeRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if self.metadata_update_behavior != 0 {
            let v = super::super::common::MetadataUpdateEnum::try_from(self.metadata_update_behavior)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.metadata_update_behavior)))?;
            struct_ser.serialize_field("metadataUpdateBehavior", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAttributeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "metadata",
            "metadata_update_behavior",
            "metadataUpdateBehavior",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Metadata,
            MetadataUpdateBehavior,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "metadataUpdateBehavior" | "metadata_update_behavior" => Ok(GeneratedField::MetadataUpdateBehavior),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateAttributeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.UpdateAttributeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateAttributeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut metadata__ = None;
                let mut metadata_update_behavior__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::MetadataUpdateBehavior => {
                            if metadata_update_behavior__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataUpdateBehavior"));
                            }
                            metadata_update_behavior__ = Some(map_.next_value::<super::super::common::MetadataUpdateEnum>()? as i32);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(UpdateAttributeRequest {
                    id: id__.unwrap_or_default(),
                    metadata: metadata__,
                    metadata_update_behavior: metadata_update_behavior__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.UpdateAttributeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateAttributeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.attribute.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.UpdateAttributeResponse", len)?;
        if let Some(v) = self.attribute.as_ref() {
            struct_ser.serialize_field("attribute", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAttributeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attribute",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Attribute,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "attribute" => Ok(GeneratedField::Attribute),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateAttributeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.UpdateAttributeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateAttributeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attribute__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Attribute => {
                            if attribute__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attribute"));
                            }
                            attribute__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(UpdateAttributeResponse {
                    attribute: attribute__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.UpdateAttributeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateAttributeValueRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        if self.metadata_update_behavior != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.UpdateAttributeValueRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if self.metadata_update_behavior != 0 {
            let v = super::super::common::MetadataUpdateEnum::try_from(self.metadata_update_behavior)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.metadata_update_behavior)))?;
            struct_ser.serialize_field("metadataUpdateBehavior", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAttributeValueRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "metadata",
            "metadata_update_behavior",
            "metadataUpdateBehavior",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Metadata,
            MetadataUpdateBehavior,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "metadataUpdateBehavior" | "metadata_update_behavior" => Ok(GeneratedField::MetadataUpdateBehavior),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateAttributeValueRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.UpdateAttributeValueRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateAttributeValueRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut metadata__ = None;
                let mut metadata_update_behavior__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::MetadataUpdateBehavior => {
                            if metadata_update_behavior__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataUpdateBehavior"));
                            }
                            metadata_update_behavior__ = Some(map_.next_value::<super::super::common::MetadataUpdateEnum>()? as i32);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(UpdateAttributeValueRequest {
                    id: id__.unwrap_or_default(),
                    metadata: metadata__,
                    metadata_update_behavior: metadata_update_behavior__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.UpdateAttributeValueRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateAttributeValueResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.UpdateAttributeValueResponse", len)?;
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAttributeValueResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "value" => Ok(GeneratedField::Value),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateAttributeValueResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.UpdateAttributeValueResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateAttributeValueResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(UpdateAttributeValueResponse {
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.UpdateAttributeValueResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValueKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.value_id.is_empty() {
            len += 1;
        }
        if !self.key_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.ValueKey", len)?;
        if !self.value_id.is_empty() {
            struct_ser.serialize_field("valueId", &self.value_id)?;
        }
        if !self.key_id.is_empty() {
            struct_ser.serialize_field("keyId", &self.key_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValueKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value_id",
            "valueId",
            "key_id",
            "keyId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValueId,
            KeyId,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "valueId" | "value_id" => Ok(GeneratedField::ValueId),
                            "keyId" | "key_id" => Ok(GeneratedField::KeyId),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValueKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.ValueKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ValueKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value_id__ = None;
                let mut key_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ValueId => {
                            if value_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueId"));
                            }
                            value_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KeyId => {
                            if key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyId"));
                            }
                            key_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ValueKey {
                    value_id: value_id__.unwrap_or_default(),
                    key_id: key_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.ValueKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValueKeyAccessServer {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.value_id.is_empty() {
            len += 1;
        }
        if !self.key_access_server_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.attributes.ValueKeyAccessServer", len)?;
        if !self.value_id.is_empty() {
            struct_ser.serialize_field("valueId", &self.value_id)?;
        }
        if !self.key_access_server_id.is_empty() {
            struct_ser.serialize_field("keyAccessServerId", &self.key_access_server_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValueKeyAccessServer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value_id",
            "valueId",
            "key_access_server_id",
            "keyAccessServerId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValueId,
            KeyAccessServerId,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "valueId" | "value_id" => Ok(GeneratedField::ValueId),
                            "keyAccessServerId" | "key_access_server_id" => Ok(GeneratedField::KeyAccessServerId),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValueKeyAccessServer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.attributes.ValueKeyAccessServer")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ValueKeyAccessServer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value_id__ = None;
                let mut key_access_server_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ValueId => {
                            if value_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueId"));
                            }
                            value_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KeyAccessServerId => {
                            if key_access_server_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyAccessServerId"));
                            }
                            key_access_server_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ValueKeyAccessServer {
                    value_id: value_id__.unwrap_or_default(),
                    key_access_server_id: key_access_server_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.attributes.ValueKeyAccessServer", FIELDS, GeneratedVisitor)
    }
}
