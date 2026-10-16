// @generated
impl serde::Serialize for AssignKeyAccessServerToNamespaceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.namespace_key_access_server.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.namespaces.AssignKeyAccessServerToNamespaceRequest", len)?;
        if let Some(v) = self.namespace_key_access_server.as_ref() {
            struct_ser.serialize_field("namespaceKeyAccessServer", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssignKeyAccessServerToNamespaceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "namespace_key_access_server",
            "namespaceKeyAccessServer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NamespaceKeyAccessServer,
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
                            "namespaceKeyAccessServer" | "namespace_key_access_server" => Ok(GeneratedField::NamespaceKeyAccessServer),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssignKeyAccessServerToNamespaceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.namespaces.AssignKeyAccessServerToNamespaceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AssignKeyAccessServerToNamespaceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut namespace_key_access_server__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NamespaceKeyAccessServer => {
                            if namespace_key_access_server__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaceKeyAccessServer"));
                            }
                            namespace_key_access_server__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AssignKeyAccessServerToNamespaceRequest {
                    namespace_key_access_server: namespace_key_access_server__,
                })
            }
        }
        deserializer.deserialize_struct("policy.namespaces.AssignKeyAccessServerToNamespaceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AssignKeyAccessServerToNamespaceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.namespace_key_access_server.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.namespaces.AssignKeyAccessServerToNamespaceResponse", len)?;
        if let Some(v) = self.namespace_key_access_server.as_ref() {
            struct_ser.serialize_field("namespaceKeyAccessServer", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssignKeyAccessServerToNamespaceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "namespace_key_access_server",
            "namespaceKeyAccessServer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NamespaceKeyAccessServer,
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
                            "namespaceKeyAccessServer" | "namespace_key_access_server" => Ok(GeneratedField::NamespaceKeyAccessServer),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssignKeyAccessServerToNamespaceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.namespaces.AssignKeyAccessServerToNamespaceResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AssignKeyAccessServerToNamespaceResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut namespace_key_access_server__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NamespaceKeyAccessServer => {
                            if namespace_key_access_server__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaceKeyAccessServer"));
                            }
                            namespace_key_access_server__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AssignKeyAccessServerToNamespaceResponse {
                    namespace_key_access_server: namespace_key_access_server__,
                })
            }
        }
        deserializer.deserialize_struct("policy.namespaces.AssignKeyAccessServerToNamespaceResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AssignPublicKeyToNamespaceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.namespace_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.namespaces.AssignPublicKeyToNamespaceRequest", len)?;
        if let Some(v) = self.namespace_key.as_ref() {
            struct_ser.serialize_field("namespaceKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssignPublicKeyToNamespaceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "namespace_key",
            "namespaceKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NamespaceKey,
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
                            "namespaceKey" | "namespace_key" => Ok(GeneratedField::NamespaceKey),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssignPublicKeyToNamespaceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.namespaces.AssignPublicKeyToNamespaceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AssignPublicKeyToNamespaceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut namespace_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NamespaceKey => {
                            if namespace_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaceKey"));
                            }
                            namespace_key__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AssignPublicKeyToNamespaceRequest {
                    namespace_key: namespace_key__,
                })
            }
        }
        deserializer.deserialize_struct("policy.namespaces.AssignPublicKeyToNamespaceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AssignPublicKeyToNamespaceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.namespace_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.namespaces.AssignPublicKeyToNamespaceResponse", len)?;
        if let Some(v) = self.namespace_key.as_ref() {
            struct_ser.serialize_field("namespaceKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssignPublicKeyToNamespaceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "namespace_key",
            "namespaceKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NamespaceKey,
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
                            "namespaceKey" | "namespace_key" => Ok(GeneratedField::NamespaceKey),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssignPublicKeyToNamespaceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.namespaces.AssignPublicKeyToNamespaceResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AssignPublicKeyToNamespaceResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut namespace_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NamespaceKey => {
                            if namespace_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaceKey"));
                            }
                            namespace_key__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AssignPublicKeyToNamespaceResponse {
                    namespace_key: namespace_key__,
                })
            }
        }
        deserializer.deserialize_struct("policy.namespaces.AssignPublicKeyToNamespaceResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateNamespaceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.namespaces.CreateNamespaceRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateNamespaceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            "name" => Ok(GeneratedField::Name),
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
            type Value = CreateNamespaceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.namespaces.CreateNamespaceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateNamespaceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
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
                Ok(CreateNamespaceRequest {
                    name: name__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.namespaces.CreateNamespaceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateNamespaceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.namespace.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.namespaces.CreateNamespaceResponse", len)?;
        if let Some(v) = self.namespace.as_ref() {
            struct_ser.serialize_field("namespace", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateNamespaceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "namespace",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = CreateNamespaceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.namespaces.CreateNamespaceResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateNamespaceResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut namespace__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                Ok(CreateNamespaceResponse {
                    namespace: namespace__,
                })
            }
        }
        deserializer.deserialize_struct("policy.namespaces.CreateNamespaceResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeactivateNamespaceRequest {
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
        let mut struct_ser = serializer.serialize_struct("policy.namespaces.DeactivateNamespaceRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeactivateNamespaceRequest {
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
            type Value = DeactivateNamespaceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.namespaces.DeactivateNamespaceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeactivateNamespaceRequest, V::Error>
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
                Ok(DeactivateNamespaceRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.namespaces.DeactivateNamespaceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeactivateNamespaceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("policy.namespaces.DeactivateNamespaceResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeactivateNamespaceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeactivateNamespaceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.namespaces.DeactivateNamespaceResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeactivateNamespaceResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeactivateNamespaceResponse {
                })
            }
        }
        deserializer.deserialize_struct("policy.namespaces.DeactivateNamespaceResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetNamespaceRequest {
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
        let mut struct_ser = serializer.serialize_struct("policy.namespaces.GetNamespaceRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.identifier.as_ref() {
            match v {
                get_namespace_request::Identifier::NamespaceId(v) => {
                    struct_ser.serialize_field("namespaceId", v)?;
                }
                get_namespace_request::Identifier::Fqn(v) => {
                    struct_ser.serialize_field("fqn", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetNamespaceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "namespace_id",
            "namespaceId",
            "fqn",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            NamespaceId,
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
                            "namespaceId" | "namespace_id" => Ok(GeneratedField::NamespaceId),
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
            type Value = GetNamespaceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.namespaces.GetNamespaceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetNamespaceRequest, V::Error>
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
                        GeneratedField::NamespaceId => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaceId"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(get_namespace_request::Identifier::NamespaceId);
                        }
                        GeneratedField::Fqn => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fqn"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(get_namespace_request::Identifier::Fqn);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetNamespaceRequest {
                    id: id__.unwrap_or_default(),
                    identifier: identifier__,
                })
            }
        }
        deserializer.deserialize_struct("policy.namespaces.GetNamespaceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetNamespaceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.namespace.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.namespaces.GetNamespaceResponse", len)?;
        if let Some(v) = self.namespace.as_ref() {
            struct_ser.serialize_field("namespace", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetNamespaceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "namespace",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = GetNamespaceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.namespaces.GetNamespaceResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetNamespaceResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut namespace__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                Ok(GetNamespaceResponse {
                    namespace: namespace__,
                })
            }
        }
        deserializer.deserialize_struct("policy.namespaces.GetNamespaceResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListNamespacesRequest {
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
        if self.pagination.is_some() {
            len += 1;
        }
        if !self.sort.is_empty() {
            len += 1;
        }
        if self.search.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.namespaces.ListNamespacesRequest", len)?;
        if self.state != 0 {
            let v = super::super::common::ActiveStateEnum::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
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
impl<'de> serde::Deserialize<'de> for ListNamespacesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "state",
            "pagination",
            "sort",
            "search",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            State,
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
            type Value = ListNamespacesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.namespaces.ListNamespacesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListNamespacesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut state__ = None;
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
                Ok(ListNamespacesRequest {
                    state: state__.unwrap_or_default(),
                    pagination: pagination__,
                    sort: sort__.unwrap_or_default(),
                    search: search__,
                })
            }
        }
        deserializer.deserialize_struct("policy.namespaces.ListNamespacesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListNamespacesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.namespaces.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.namespaces.ListNamespacesResponse", len)?;
        if !self.namespaces.is_empty() {
            struct_ser.serialize_field("namespaces", &self.namespaces)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListNamespacesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "namespaces",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Namespaces,
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
                            "namespaces" => Ok(GeneratedField::Namespaces),
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
            type Value = ListNamespacesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.namespaces.ListNamespacesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListNamespacesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut namespaces__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Namespaces => {
                            if namespaces__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaces"));
                            }
                            namespaces__ = Some(map_.next_value()?);
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
                Ok(ListNamespacesResponse {
                    namespaces: namespaces__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("policy.namespaces.ListNamespacesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NamespaceKey {
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
        if !self.key_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.namespaces.NamespaceKey", len)?;
        if !self.namespace_id.is_empty() {
            struct_ser.serialize_field("namespaceId", &self.namespace_id)?;
        }
        if !self.key_id.is_empty() {
            struct_ser.serialize_field("keyId", &self.key_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NamespaceKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "namespace_id",
            "namespaceId",
            "key_id",
            "keyId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NamespaceId,
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
                            "namespaceId" | "namespace_id" => Ok(GeneratedField::NamespaceId),
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
            type Value = NamespaceKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.namespaces.NamespaceKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NamespaceKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut namespace_id__ = None;
                let mut key_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NamespaceId => {
                            if namespace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaceId"));
                            }
                            namespace_id__ = Some(map_.next_value()?);
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
                Ok(NamespaceKey {
                    namespace_id: namespace_id__.unwrap_or_default(),
                    key_id: key_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.namespaces.NamespaceKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NamespaceKeyAccessServer {
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
        if !self.key_access_server_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.namespaces.NamespaceKeyAccessServer", len)?;
        if !self.namespace_id.is_empty() {
            struct_ser.serialize_field("namespaceId", &self.namespace_id)?;
        }
        if !self.key_access_server_id.is_empty() {
            struct_ser.serialize_field("keyAccessServerId", &self.key_access_server_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NamespaceKeyAccessServer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "namespace_id",
            "namespaceId",
            "key_access_server_id",
            "keyAccessServerId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NamespaceId,
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
                            "namespaceId" | "namespace_id" => Ok(GeneratedField::NamespaceId),
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
            type Value = NamespaceKeyAccessServer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.namespaces.NamespaceKeyAccessServer")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NamespaceKeyAccessServer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut namespace_id__ = None;
                let mut key_access_server_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NamespaceId => {
                            if namespace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaceId"));
                            }
                            namespace_id__ = Some(map_.next_value()?);
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
                Ok(NamespaceKeyAccessServer {
                    namespace_id: namespace_id__.unwrap_or_default(),
                    key_access_server_id: key_access_server_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.namespaces.NamespaceKeyAccessServer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NamespacesSort {
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
        let mut struct_ser = serializer.serialize_struct("policy.namespaces.NamespacesSort", len)?;
        if self.field != 0 {
            let v = SortNamespacesType::try_from(self.field)
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
impl<'de> serde::Deserialize<'de> for NamespacesSort {
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
            type Value = NamespacesSort;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.namespaces.NamespacesSort")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NamespacesSort, V::Error>
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
                            field__ = Some(map_.next_value::<SortNamespacesType>()? as i32);
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
                Ok(NamespacesSort {
                    field: field__.unwrap_or_default(),
                    direction: direction__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.namespaces.NamespacesSort", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveKeyAccessServerFromNamespaceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.namespace_key_access_server.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.namespaces.RemoveKeyAccessServerFromNamespaceRequest", len)?;
        if let Some(v) = self.namespace_key_access_server.as_ref() {
            struct_ser.serialize_field("namespaceKeyAccessServer", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveKeyAccessServerFromNamespaceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "namespace_key_access_server",
            "namespaceKeyAccessServer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NamespaceKeyAccessServer,
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
                            "namespaceKeyAccessServer" | "namespace_key_access_server" => Ok(GeneratedField::NamespaceKeyAccessServer),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoveKeyAccessServerFromNamespaceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.namespaces.RemoveKeyAccessServerFromNamespaceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveKeyAccessServerFromNamespaceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut namespace_key_access_server__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NamespaceKeyAccessServer => {
                            if namespace_key_access_server__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaceKeyAccessServer"));
                            }
                            namespace_key_access_server__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(RemoveKeyAccessServerFromNamespaceRequest {
                    namespace_key_access_server: namespace_key_access_server__,
                })
            }
        }
        deserializer.deserialize_struct("policy.namespaces.RemoveKeyAccessServerFromNamespaceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveKeyAccessServerFromNamespaceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.namespace_key_access_server.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.namespaces.RemoveKeyAccessServerFromNamespaceResponse", len)?;
        if let Some(v) = self.namespace_key_access_server.as_ref() {
            struct_ser.serialize_field("namespaceKeyAccessServer", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveKeyAccessServerFromNamespaceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "namespace_key_access_server",
            "namespaceKeyAccessServer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NamespaceKeyAccessServer,
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
                            "namespaceKeyAccessServer" | "namespace_key_access_server" => Ok(GeneratedField::NamespaceKeyAccessServer),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoveKeyAccessServerFromNamespaceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.namespaces.RemoveKeyAccessServerFromNamespaceResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveKeyAccessServerFromNamespaceResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut namespace_key_access_server__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NamespaceKeyAccessServer => {
                            if namespace_key_access_server__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaceKeyAccessServer"));
                            }
                            namespace_key_access_server__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(RemoveKeyAccessServerFromNamespaceResponse {
                    namespace_key_access_server: namespace_key_access_server__,
                })
            }
        }
        deserializer.deserialize_struct("policy.namespaces.RemoveKeyAccessServerFromNamespaceResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemovePublicKeyFromNamespaceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.namespace_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.namespaces.RemovePublicKeyFromNamespaceRequest", len)?;
        if let Some(v) = self.namespace_key.as_ref() {
            struct_ser.serialize_field("namespaceKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemovePublicKeyFromNamespaceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "namespace_key",
            "namespaceKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NamespaceKey,
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
                            "namespaceKey" | "namespace_key" => Ok(GeneratedField::NamespaceKey),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemovePublicKeyFromNamespaceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.namespaces.RemovePublicKeyFromNamespaceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemovePublicKeyFromNamespaceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut namespace_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NamespaceKey => {
                            if namespace_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaceKey"));
                            }
                            namespace_key__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(RemovePublicKeyFromNamespaceRequest {
                    namespace_key: namespace_key__,
                })
            }
        }
        deserializer.deserialize_struct("policy.namespaces.RemovePublicKeyFromNamespaceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemovePublicKeyFromNamespaceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.namespace_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.namespaces.RemovePublicKeyFromNamespaceResponse", len)?;
        if let Some(v) = self.namespace_key.as_ref() {
            struct_ser.serialize_field("namespaceKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemovePublicKeyFromNamespaceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "namespace_key",
            "namespaceKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NamespaceKey,
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
                            "namespaceKey" | "namespace_key" => Ok(GeneratedField::NamespaceKey),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemovePublicKeyFromNamespaceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.namespaces.RemovePublicKeyFromNamespaceResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemovePublicKeyFromNamespaceResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut namespace_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NamespaceKey => {
                            if namespace_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaceKey"));
                            }
                            namespace_key__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(RemovePublicKeyFromNamespaceResponse {
                    namespace_key: namespace_key__,
                })
            }
        }
        deserializer.deserialize_struct("policy.namespaces.RemovePublicKeyFromNamespaceResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SortNamespacesType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SORT_NAMESPACES_TYPE_UNSPECIFIED",
            Self::Name => "SORT_NAMESPACES_TYPE_NAME",
            Self::Fqn => "SORT_NAMESPACES_TYPE_FQN",
            Self::CreatedAt => "SORT_NAMESPACES_TYPE_CREATED_AT",
            Self::UpdatedAt => "SORT_NAMESPACES_TYPE_UPDATED_AT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SortNamespacesType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SORT_NAMESPACES_TYPE_UNSPECIFIED",
            "SORT_NAMESPACES_TYPE_NAME",
            "SORT_NAMESPACES_TYPE_FQN",
            "SORT_NAMESPACES_TYPE_CREATED_AT",
            "SORT_NAMESPACES_TYPE_UPDATED_AT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SortNamespacesType;

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
                    "SORT_NAMESPACES_TYPE_UNSPECIFIED" => Ok(SortNamespacesType::Unspecified),
                    "SORT_NAMESPACES_TYPE_NAME" => Ok(SortNamespacesType::Name),
                    "SORT_NAMESPACES_TYPE_FQN" => Ok(SortNamespacesType::Fqn),
                    "SORT_NAMESPACES_TYPE_CREATED_AT" => Ok(SortNamespacesType::CreatedAt),
                    "SORT_NAMESPACES_TYPE_UPDATED_AT" => Ok(SortNamespacesType::UpdatedAt),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateNamespaceRequest {
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
        let mut struct_ser = serializer.serialize_struct("policy.namespaces.UpdateNamespaceRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for UpdateNamespaceRequest {
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
            type Value = UpdateNamespaceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.namespaces.UpdateNamespaceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateNamespaceRequest, V::Error>
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
                Ok(UpdateNamespaceRequest {
                    id: id__.unwrap_or_default(),
                    metadata: metadata__,
                    metadata_update_behavior: metadata_update_behavior__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.namespaces.UpdateNamespaceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateNamespaceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.namespace.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.namespaces.UpdateNamespaceResponse", len)?;
        if let Some(v) = self.namespace.as_ref() {
            struct_ser.serialize_field("namespace", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateNamespaceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "namespace",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = UpdateNamespaceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.namespaces.UpdateNamespaceResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateNamespaceResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut namespace__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                Ok(UpdateNamespaceResponse {
                    namespace: namespace__,
                })
            }
        }
        deserializer.deserialize_struct("policy.namespaces.UpdateNamespaceResponse", FIELDS, GeneratedVisitor)
    }
}
