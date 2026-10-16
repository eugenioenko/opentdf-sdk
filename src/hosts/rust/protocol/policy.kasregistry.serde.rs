// @generated
impl serde::Serialize for ActivatePublicKeyRequest {
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
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.ActivatePublicKeyRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ActivatePublicKeyRequest {
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
            type Value = ActivatePublicKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.ActivatePublicKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ActivatePublicKeyRequest, V::Error>
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
                Ok(ActivatePublicKeyRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.ActivatePublicKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ActivatePublicKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.ActivatePublicKeyResponse", len)?;
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ActivatePublicKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
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
                            "key" => Ok(GeneratedField::Key),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ActivatePublicKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.ActivatePublicKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ActivatePublicKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ActivatePublicKeyResponse {
                    key: key__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.ActivatePublicKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ChangeMappings {
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
        if !self.fqn.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.ChangeMappings", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.fqn.is_empty() {
            struct_ser.serialize_field("fqn", &self.fqn)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChangeMappings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "fqn",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
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
            type Value = ChangeMappings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.ChangeMappings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ChangeMappings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut fqn__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fqn => {
                            if fqn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fqn"));
                            }
                            fqn__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ChangeMappings {
                    id: id__.unwrap_or_default(),
                    fqn: fqn__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.ChangeMappings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateKeyAccessServerRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.uri.is_empty() {
            len += 1;
        }
        if self.public_key.is_some() {
            len += 1;
        }
        if self.source_type != 0 {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.CreateKeyAccessServerRequest", len)?;
        if !self.uri.is_empty() {
            struct_ser.serialize_field("uri", &self.uri)?;
        }
        if let Some(v) = self.public_key.as_ref() {
            struct_ser.serialize_field("publicKey", v)?;
        }
        if self.source_type != 0 {
            let v = super::SourceType::try_from(self.source_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.source_type)))?;
            struct_ser.serialize_field("sourceType", &v)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateKeyAccessServerRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uri",
            "public_key",
            "publicKey",
            "source_type",
            "sourceType",
            "name",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uri,
            PublicKey,
            SourceType,
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
                            "uri" => Ok(GeneratedField::Uri),
                            "publicKey" | "public_key" => Ok(GeneratedField::PublicKey),
                            "sourceType" | "source_type" => Ok(GeneratedField::SourceType),
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
            type Value = CreateKeyAccessServerRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.CreateKeyAccessServerRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateKeyAccessServerRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uri__ = None;
                let mut public_key__ = None;
                let mut source_type__ = None;
                let mut name__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Uri => {
                            if uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uri"));
                            }
                            uri__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PublicKey => {
                            if public_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicKey"));
                            }
                            public_key__ = map_.next_value()?;
                        }
                        GeneratedField::SourceType => {
                            if source_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceType"));
                            }
                            source_type__ = Some(map_.next_value::<super::SourceType>()? as i32);
                        }
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
                Ok(CreateKeyAccessServerRequest {
                    uri: uri__.unwrap_or_default(),
                    public_key: public_key__,
                    source_type: source_type__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.CreateKeyAccessServerRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateKeyAccessServerResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key_access_server.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.CreateKeyAccessServerResponse", len)?;
        if let Some(v) = self.key_access_server.as_ref() {
            struct_ser.serialize_field("keyAccessServer", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateKeyAccessServerResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key_access_server",
            "keyAccessServer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KeyAccessServer,
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
                            "keyAccessServer" | "key_access_server" => Ok(GeneratedField::KeyAccessServer),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateKeyAccessServerResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.CreateKeyAccessServerResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateKeyAccessServerResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key_access_server__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KeyAccessServer => {
                            if key_access_server__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyAccessServer"));
                            }
                            key_access_server__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(CreateKeyAccessServerResponse {
                    key_access_server: key_access_server__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.CreateKeyAccessServerResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.kas_id.is_empty() {
            len += 1;
        }
        if !self.key_id.is_empty() {
            len += 1;
        }
        if self.key_algorithm != 0 {
            len += 1;
        }
        if self.key_mode != 0 {
            len += 1;
        }
        if self.public_key_ctx.is_some() {
            len += 1;
        }
        if self.private_key_ctx.is_some() {
            len += 1;
        }
        if !self.provider_config_id.is_empty() {
            len += 1;
        }
        if self.legacy {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.CreateKeyRequest", len)?;
        if !self.kas_id.is_empty() {
            struct_ser.serialize_field("kasId", &self.kas_id)?;
        }
        if !self.key_id.is_empty() {
            struct_ser.serialize_field("keyId", &self.key_id)?;
        }
        if self.key_algorithm != 0 {
            let v = super::Algorithm::try_from(self.key_algorithm)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.key_algorithm)))?;
            struct_ser.serialize_field("keyAlgorithm", &v)?;
        }
        if self.key_mode != 0 {
            let v = super::KeyMode::try_from(self.key_mode)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.key_mode)))?;
            struct_ser.serialize_field("keyMode", &v)?;
        }
        if let Some(v) = self.public_key_ctx.as_ref() {
            struct_ser.serialize_field("publicKeyCtx", v)?;
        }
        if let Some(v) = self.private_key_ctx.as_ref() {
            struct_ser.serialize_field("privateKeyCtx", v)?;
        }
        if !self.provider_config_id.is_empty() {
            struct_ser.serialize_field("providerConfigId", &self.provider_config_id)?;
        }
        if self.legacy {
            struct_ser.serialize_field("legacy", &self.legacy)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kas_id",
            "kasId",
            "key_id",
            "keyId",
            "key_algorithm",
            "keyAlgorithm",
            "key_mode",
            "keyMode",
            "public_key_ctx",
            "publicKeyCtx",
            "private_key_ctx",
            "privateKeyCtx",
            "provider_config_id",
            "providerConfigId",
            "legacy",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KasId,
            KeyId,
            KeyAlgorithm,
            KeyMode,
            PublicKeyCtx,
            PrivateKeyCtx,
            ProviderConfigId,
            Legacy,
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
                            "kasId" | "kas_id" => Ok(GeneratedField::KasId),
                            "keyId" | "key_id" => Ok(GeneratedField::KeyId),
                            "keyAlgorithm" | "key_algorithm" => Ok(GeneratedField::KeyAlgorithm),
                            "keyMode" | "key_mode" => Ok(GeneratedField::KeyMode),
                            "publicKeyCtx" | "public_key_ctx" => Ok(GeneratedField::PublicKeyCtx),
                            "privateKeyCtx" | "private_key_ctx" => Ok(GeneratedField::PrivateKeyCtx),
                            "providerConfigId" | "provider_config_id" => Ok(GeneratedField::ProviderConfigId),
                            "legacy" => Ok(GeneratedField::Legacy),
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
            type Value = CreateKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.CreateKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kas_id__ = None;
                let mut key_id__ = None;
                let mut key_algorithm__ = None;
                let mut key_mode__ = None;
                let mut public_key_ctx__ = None;
                let mut private_key_ctx__ = None;
                let mut provider_config_id__ = None;
                let mut legacy__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KasId => {
                            if kas_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasId"));
                            }
                            kas_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KeyId => {
                            if key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyId"));
                            }
                            key_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KeyAlgorithm => {
                            if key_algorithm__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyAlgorithm"));
                            }
                            key_algorithm__ = Some(map_.next_value::<super::Algorithm>()? as i32);
                        }
                        GeneratedField::KeyMode => {
                            if key_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyMode"));
                            }
                            key_mode__ = Some(map_.next_value::<super::KeyMode>()? as i32);
                        }
                        GeneratedField::PublicKeyCtx => {
                            if public_key_ctx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicKeyCtx"));
                            }
                            public_key_ctx__ = map_.next_value()?;
                        }
                        GeneratedField::PrivateKeyCtx => {
                            if private_key_ctx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privateKeyCtx"));
                            }
                            private_key_ctx__ = map_.next_value()?;
                        }
                        GeneratedField::ProviderConfigId => {
                            if provider_config_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerConfigId"));
                            }
                            provider_config_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Legacy => {
                            if legacy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("legacy"));
                            }
                            legacy__ = Some(map_.next_value()?);
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
                Ok(CreateKeyRequest {
                    kas_id: kas_id__.unwrap_or_default(),
                    key_id: key_id__.unwrap_or_default(),
                    key_algorithm: key_algorithm__.unwrap_or_default(),
                    key_mode: key_mode__.unwrap_or_default(),
                    public_key_ctx: public_key_ctx__,
                    private_key_ctx: private_key_ctx__,
                    provider_config_id: provider_config_id__.unwrap_or_default(),
                    legacy: legacy__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.CreateKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.kas_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.CreateKeyResponse", len)?;
        if let Some(v) = self.kas_key.as_ref() {
            struct_ser.serialize_field("kasKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kas_key",
            "kasKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KasKey,
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
                            "kasKey" | "kas_key" => Ok(GeneratedField::KasKey),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.CreateKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kas_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KasKey => {
                            if kas_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasKey"));
                            }
                            kas_key__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(CreateKeyResponse {
                    kas_key: kas_key__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.CreateKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreatePublicKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.kas_id.is_empty() {
            len += 1;
        }
        if self.key.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.CreatePublicKeyRequest", len)?;
        if !self.kas_id.is_empty() {
            struct_ser.serialize_field("kasId", &self.kas_id)?;
        }
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreatePublicKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kas_id",
            "kasId",
            "key",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KasId,
            Key,
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
                            "kasId" | "kas_id" => Ok(GeneratedField::KasId),
                            "key" => Ok(GeneratedField::Key),
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
            type Value = CreatePublicKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.CreatePublicKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreatePublicKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kas_id__ = None;
                let mut key__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KasId => {
                            if kas_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasId"));
                            }
                            kas_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map_.next_value()?;
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
                Ok(CreatePublicKeyRequest {
                    kas_id: kas_id__.unwrap_or_default(),
                    key: key__,
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.CreatePublicKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreatePublicKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.CreatePublicKeyResponse", len)?;
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreatePublicKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
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
                            "key" => Ok(GeneratedField::Key),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreatePublicKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.CreatePublicKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreatePublicKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(CreatePublicKeyResponse {
                    key: key__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.CreatePublicKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeactivatePublicKeyRequest {
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
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.DeactivatePublicKeyRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeactivatePublicKeyRequest {
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
            type Value = DeactivatePublicKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.DeactivatePublicKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeactivatePublicKeyRequest, V::Error>
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
                Ok(DeactivatePublicKeyRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.DeactivatePublicKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeactivatePublicKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.DeactivatePublicKeyResponse", len)?;
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeactivatePublicKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
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
                            "key" => Ok(GeneratedField::Key),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeactivatePublicKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.DeactivatePublicKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeactivatePublicKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DeactivatePublicKeyResponse {
                    key: key__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.DeactivatePublicKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteKeyAccessServerRequest {
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
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.DeleteKeyAccessServerRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteKeyAccessServerRequest {
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
            type Value = DeleteKeyAccessServerRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.DeleteKeyAccessServerRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteKeyAccessServerRequest, V::Error>
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
                Ok(DeleteKeyAccessServerRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.DeleteKeyAccessServerRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteKeyAccessServerResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key_access_server.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.DeleteKeyAccessServerResponse", len)?;
        if let Some(v) = self.key_access_server.as_ref() {
            struct_ser.serialize_field("keyAccessServer", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteKeyAccessServerResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key_access_server",
            "keyAccessServer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KeyAccessServer,
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
                            "keyAccessServer" | "key_access_server" => Ok(GeneratedField::KeyAccessServer),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteKeyAccessServerResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.DeleteKeyAccessServerResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteKeyAccessServerResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key_access_server__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KeyAccessServer => {
                            if key_access_server__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyAccessServer"));
                            }
                            key_access_server__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DeleteKeyAccessServerResponse {
                    key_access_server: key_access_server__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.DeleteKeyAccessServerResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetBaseKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("policy.kasregistry.GetBaseKeyRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetBaseKeyRequest {
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
            type Value = GetBaseKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.GetBaseKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetBaseKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GetBaseKeyRequest {
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.GetBaseKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetBaseKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.base_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.GetBaseKeyResponse", len)?;
        if let Some(v) = self.base_key.as_ref() {
            struct_ser.serialize_field("baseKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetBaseKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base_key",
            "baseKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseKey,
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
                            "baseKey" | "base_key" => Ok(GeneratedField::BaseKey),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetBaseKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.GetBaseKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetBaseKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BaseKey => {
                            if base_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseKey"));
                            }
                            base_key__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetBaseKeyResponse {
                    base_key: base_key__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.GetBaseKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetKeyAccessServerRequest {
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
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.GetKeyAccessServerRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.identifier.as_ref() {
            match v {
                get_key_access_server_request::Identifier::KasId(v) => {
                    struct_ser.serialize_field("kasId", v)?;
                }
                get_key_access_server_request::Identifier::Name(v) => {
                    struct_ser.serialize_field("name", v)?;
                }
                get_key_access_server_request::Identifier::Uri(v) => {
                    struct_ser.serialize_field("uri", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetKeyAccessServerRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "kas_id",
            "kasId",
            "name",
            "uri",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            KasId,
            Name,
            Uri,
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
                            "kasId" | "kas_id" => Ok(GeneratedField::KasId),
                            "name" => Ok(GeneratedField::Name),
                            "uri" => Ok(GeneratedField::Uri),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetKeyAccessServerRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.GetKeyAccessServerRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetKeyAccessServerRequest, V::Error>
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
                        GeneratedField::KasId => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasId"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(get_key_access_server_request::Identifier::KasId);
                        }
                        GeneratedField::Name => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(get_key_access_server_request::Identifier::Name);
                        }
                        GeneratedField::Uri => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uri"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(get_key_access_server_request::Identifier::Uri);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetKeyAccessServerRequest {
                    id: id__.unwrap_or_default(),
                    identifier: identifier__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.GetKeyAccessServerRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetKeyAccessServerResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key_access_server.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.GetKeyAccessServerResponse", len)?;
        if let Some(v) = self.key_access_server.as_ref() {
            struct_ser.serialize_field("keyAccessServer", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetKeyAccessServerResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key_access_server",
            "keyAccessServer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KeyAccessServer,
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
                            "keyAccessServer" | "key_access_server" => Ok(GeneratedField::KeyAccessServer),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetKeyAccessServerResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.GetKeyAccessServerResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetKeyAccessServerResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key_access_server__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KeyAccessServer => {
                            if key_access_server__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyAccessServer"));
                            }
                            key_access_server__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetKeyAccessServerResponse {
                    key_access_server: key_access_server__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.GetKeyAccessServerResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.identifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.GetKeyRequest", len)?;
        if let Some(v) = self.identifier.as_ref() {
            match v {
                get_key_request::Identifier::Id(v) => {
                    struct_ser.serialize_field("id", v)?;
                }
                get_key_request::Identifier::Key(v) => {
                    struct_ser.serialize_field("key", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Key,
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
                            "key" => Ok(GeneratedField::Key),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.GetKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut identifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(get_key_request::Identifier::Id);
                        }
                        GeneratedField::Key => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(get_key_request::Identifier::Key)
;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetKeyRequest {
                    identifier: identifier__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.GetKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.kas_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.GetKeyResponse", len)?;
        if let Some(v) = self.kas_key.as_ref() {
            struct_ser.serialize_field("kasKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kas_key",
            "kasKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KasKey,
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
                            "kasKey" | "kas_key" => Ok(GeneratedField::KasKey),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.GetKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kas_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KasKey => {
                            if kas_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasKey"));
                            }
                            kas_key__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetKeyResponse {
                    kas_key: kas_key__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.GetKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPublicKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.identifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.GetPublicKeyRequest", len)?;
        if let Some(v) = self.identifier.as_ref() {
            match v {
                get_public_key_request::Identifier::Id(v) => {
                    struct_ser.serialize_field("id", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPublicKeyRequest {
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
            type Value = GetPublicKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.GetPublicKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPublicKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut identifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(get_public_key_request::Identifier::Id);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetPublicKeyRequest {
                    identifier: identifier__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.GetPublicKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPublicKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.GetPublicKeyResponse", len)?;
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPublicKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
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
                            "key" => Ok(GeneratedField::Key),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPublicKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.GetPublicKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPublicKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetPublicKeyResponse {
                    key: key__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.GetPublicKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GrantedPolicyObject {
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
        if !self.fqn.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.GrantedPolicyObject", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.fqn.is_empty() {
            struct_ser.serialize_field("fqn", &self.fqn)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GrantedPolicyObject {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "fqn",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
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
            type Value = GrantedPolicyObject;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.GrantedPolicyObject")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GrantedPolicyObject, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut fqn__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fqn => {
                            if fqn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fqn"));
                            }
                            fqn__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GrantedPolicyObject {
                    id: id__.unwrap_or_default(),
                    fqn: fqn__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.GrantedPolicyObject", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KasKeyIdentifier {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.kid.is_empty() {
            len += 1;
        }
        if self.identifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.KasKeyIdentifier", len)?;
        if !self.kid.is_empty() {
            struct_ser.serialize_field("kid", &self.kid)?;
        }
        if let Some(v) = self.identifier.as_ref() {
            match v {
                kas_key_identifier::Identifier::KasId(v) => {
                    struct_ser.serialize_field("kasId", v)?;
                }
                kas_key_identifier::Identifier::Name(v) => {
                    struct_ser.serialize_field("name", v)?;
                }
                kas_key_identifier::Identifier::Uri(v) => {
                    struct_ser.serialize_field("uri", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for KasKeyIdentifier {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kid",
            "kas_id",
            "kasId",
            "name",
            "uri",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Kid,
            KasId,
            Name,
            Uri,
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
                            "kid" => Ok(GeneratedField::Kid),
                            "kasId" | "kas_id" => Ok(GeneratedField::KasId),
                            "name" => Ok(GeneratedField::Name),
                            "uri" => Ok(GeneratedField::Uri),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = KasKeyIdentifier;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.KasKeyIdentifier")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<KasKeyIdentifier, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kid__ = None;
                let mut identifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Kid => {
                            if kid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kid"));
                            }
                            kid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KasId => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasId"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(kas_key_identifier::Identifier::KasId);
                        }
                        GeneratedField::Name => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(kas_key_identifier::Identifier::Name);
                        }
                        GeneratedField::Uri => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uri"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(kas_key_identifier::Identifier::Uri);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(KasKeyIdentifier {
                    kid: kid__.unwrap_or_default(),
                    identifier: identifier__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.KasKeyIdentifier", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KasKeysSort {
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
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.KasKeysSort", len)?;
        if self.field != 0 {
            let v = SortKasKeysType::try_from(self.field)
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
impl<'de> serde::Deserialize<'de> for KasKeysSort {
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
            type Value = KasKeysSort;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.KasKeysSort")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<KasKeysSort, V::Error>
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
                            field__ = Some(map_.next_value::<SortKasKeysType>()? as i32);
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
                Ok(KasKeysSort {
                    field: field__.unwrap_or_default(),
                    direction: direction__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.KasKeysSort", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KeyAccessServerGrants {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key_access_server.is_some() {
            len += 1;
        }
        if !self.namespace_grants.is_empty() {
            len += 1;
        }
        if !self.attribute_grants.is_empty() {
            len += 1;
        }
        if !self.value_grants.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.KeyAccessServerGrants", len)?;
        if let Some(v) = self.key_access_server.as_ref() {
            struct_ser.serialize_field("keyAccessServer", v)?;
        }
        if !self.namespace_grants.is_empty() {
            struct_ser.serialize_field("namespaceGrants", &self.namespace_grants)?;
        }
        if !self.attribute_grants.is_empty() {
            struct_ser.serialize_field("attributeGrants", &self.attribute_grants)?;
        }
        if !self.value_grants.is_empty() {
            struct_ser.serialize_field("valueGrants", &self.value_grants)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for KeyAccessServerGrants {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key_access_server",
            "keyAccessServer",
            "namespace_grants",
            "namespaceGrants",
            "attribute_grants",
            "attributeGrants",
            "value_grants",
            "valueGrants",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KeyAccessServer,
            NamespaceGrants,
            AttributeGrants,
            ValueGrants,
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
                            "keyAccessServer" | "key_access_server" => Ok(GeneratedField::KeyAccessServer),
                            "namespaceGrants" | "namespace_grants" => Ok(GeneratedField::NamespaceGrants),
                            "attributeGrants" | "attribute_grants" => Ok(GeneratedField::AttributeGrants),
                            "valueGrants" | "value_grants" => Ok(GeneratedField::ValueGrants),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = KeyAccessServerGrants;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.KeyAccessServerGrants")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<KeyAccessServerGrants, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key_access_server__ = None;
                let mut namespace_grants__ = None;
                let mut attribute_grants__ = None;
                let mut value_grants__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KeyAccessServer => {
                            if key_access_server__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyAccessServer"));
                            }
                            key_access_server__ = map_.next_value()?;
                        }
                        GeneratedField::NamespaceGrants => {
                            if namespace_grants__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaceGrants"));
                            }
                            namespace_grants__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AttributeGrants => {
                            if attribute_grants__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeGrants"));
                            }
                            attribute_grants__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValueGrants => {
                            if value_grants__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueGrants"));
                            }
                            value_grants__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(KeyAccessServerGrants {
                    key_access_server: key_access_server__,
                    namespace_grants: namespace_grants__.unwrap_or_default(),
                    attribute_grants: attribute_grants__.unwrap_or_default(),
                    value_grants: value_grants__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.KeyAccessServerGrants", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KeyAccessServersSort {
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
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.KeyAccessServersSort", len)?;
        if self.field != 0 {
            let v = SortKeyAccessServersType::try_from(self.field)
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
impl<'de> serde::Deserialize<'de> for KeyAccessServersSort {
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
            type Value = KeyAccessServersSort;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.KeyAccessServersSort")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<KeyAccessServersSort, V::Error>
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
                            field__ = Some(map_.next_value::<SortKeyAccessServersType>()? as i32);
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
                Ok(KeyAccessServersSort {
                    field: field__.unwrap_or_default(),
                    direction: direction__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.KeyAccessServersSort", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KeyMapping {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.kid.is_empty() {
            len += 1;
        }
        if !self.kas_uri.is_empty() {
            len += 1;
        }
        if !self.namespace_mappings.is_empty() {
            len += 1;
        }
        if !self.attribute_mappings.is_empty() {
            len += 1;
        }
        if !self.value_mappings.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.KeyMapping", len)?;
        if !self.kid.is_empty() {
            struct_ser.serialize_field("kid", &self.kid)?;
        }
        if !self.kas_uri.is_empty() {
            struct_ser.serialize_field("kasUri", &self.kas_uri)?;
        }
        if !self.namespace_mappings.is_empty() {
            struct_ser.serialize_field("namespaceMappings", &self.namespace_mappings)?;
        }
        if !self.attribute_mappings.is_empty() {
            struct_ser.serialize_field("attributeMappings", &self.attribute_mappings)?;
        }
        if !self.value_mappings.is_empty() {
            struct_ser.serialize_field("valueMappings", &self.value_mappings)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for KeyMapping {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kid",
            "kas_uri",
            "kasUri",
            "namespace_mappings",
            "namespaceMappings",
            "attribute_mappings",
            "attributeMappings",
            "value_mappings",
            "valueMappings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Kid,
            KasUri,
            NamespaceMappings,
            AttributeMappings,
            ValueMappings,
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
                            "kid" => Ok(GeneratedField::Kid),
                            "kasUri" | "kas_uri" => Ok(GeneratedField::KasUri),
                            "namespaceMappings" | "namespace_mappings" => Ok(GeneratedField::NamespaceMappings),
                            "attributeMappings" | "attribute_mappings" => Ok(GeneratedField::AttributeMappings),
                            "valueMappings" | "value_mappings" => Ok(GeneratedField::ValueMappings),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = KeyMapping;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.KeyMapping")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<KeyMapping, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kid__ = None;
                let mut kas_uri__ = None;
                let mut namespace_mappings__ = None;
                let mut attribute_mappings__ = None;
                let mut value_mappings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Kid => {
                            if kid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kid"));
                            }
                            kid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KasUri => {
                            if kas_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasUri"));
                            }
                            kas_uri__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NamespaceMappings => {
                            if namespace_mappings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaceMappings"));
                            }
                            namespace_mappings__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AttributeMappings => {
                            if attribute_mappings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeMappings"));
                            }
                            attribute_mappings__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValueMappings => {
                            if value_mappings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueMappings"));
                            }
                            value_mappings__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(KeyMapping {
                    kid: kid__.unwrap_or_default(),
                    kas_uri: kas_uri__.unwrap_or_default(),
                    namespace_mappings: namespace_mappings__.unwrap_or_default(),
                    attribute_mappings: attribute_mappings__.unwrap_or_default(),
                    value_mappings: value_mappings__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.KeyMapping", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListKeyAccessServerGrantsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.kas_id.is_empty() {
            len += 1;
        }
        if !self.kas_uri.is_empty() {
            len += 1;
        }
        if !self.kas_name.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.ListKeyAccessServerGrantsRequest", len)?;
        if !self.kas_id.is_empty() {
            struct_ser.serialize_field("kasId", &self.kas_id)?;
        }
        if !self.kas_uri.is_empty() {
            struct_ser.serialize_field("kasUri", &self.kas_uri)?;
        }
        if !self.kas_name.is_empty() {
            struct_ser.serialize_field("kasName", &self.kas_name)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListKeyAccessServerGrantsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kas_id",
            "kasId",
            "kas_uri",
            "kasUri",
            "kas_name",
            "kasName",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KasId,
            KasUri,
            KasName,
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
                            "kasId" | "kas_id" => Ok(GeneratedField::KasId),
                            "kasUri" | "kas_uri" => Ok(GeneratedField::KasUri),
                            "kasName" | "kas_name" => Ok(GeneratedField::KasName),
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
            type Value = ListKeyAccessServerGrantsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.ListKeyAccessServerGrantsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListKeyAccessServerGrantsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kas_id__ = None;
                let mut kas_uri__ = None;
                let mut kas_name__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KasId => {
                            if kas_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasId"));
                            }
                            kas_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KasUri => {
                            if kas_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasUri"));
                            }
                            kas_uri__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KasName => {
                            if kas_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasName"));
                            }
                            kas_name__ = Some(map_.next_value()?);
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
                Ok(ListKeyAccessServerGrantsRequest {
                    kas_id: kas_id__.unwrap_or_default(),
                    kas_uri: kas_uri__.unwrap_or_default(),
                    kas_name: kas_name__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.ListKeyAccessServerGrantsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListKeyAccessServerGrantsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.grants.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.ListKeyAccessServerGrantsResponse", len)?;
        if !self.grants.is_empty() {
            struct_ser.serialize_field("grants", &self.grants)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListKeyAccessServerGrantsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "grants",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Grants,
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
                            "grants" => Ok(GeneratedField::Grants),
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
            type Value = ListKeyAccessServerGrantsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.ListKeyAccessServerGrantsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListKeyAccessServerGrantsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut grants__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Grants => {
                            if grants__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grants"));
                            }
                            grants__ = Some(map_.next_value()?);
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
                Ok(ListKeyAccessServerGrantsResponse {
                    grants: grants__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.ListKeyAccessServerGrantsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListKeyAccessServersRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pagination.is_some() {
            len += 1;
        }
        if !self.sort.is_empty() {
            len += 1;
        }
        if self.search.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.ListKeyAccessServersRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListKeyAccessServersRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pagination",
            "sort",
            "search",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = ListKeyAccessServersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.ListKeyAccessServersRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListKeyAccessServersRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pagination__ = None;
                let mut sort__ = None;
                let mut search__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                Ok(ListKeyAccessServersRequest {
                    pagination: pagination__,
                    sort: sort__.unwrap_or_default(),
                    search: search__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.ListKeyAccessServersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListKeyAccessServersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key_access_servers.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.ListKeyAccessServersResponse", len)?;
        if !self.key_access_servers.is_empty() {
            struct_ser.serialize_field("keyAccessServers", &self.key_access_servers)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListKeyAccessServersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key_access_servers",
            "keyAccessServers",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KeyAccessServers,
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
                            "keyAccessServers" | "key_access_servers" => Ok(GeneratedField::KeyAccessServers),
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
            type Value = ListKeyAccessServersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.ListKeyAccessServersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListKeyAccessServersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key_access_servers__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KeyAccessServers => {
                            if key_access_servers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyAccessServers"));
                            }
                            key_access_servers__ = Some(map_.next_value()?);
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
                Ok(ListKeyAccessServersResponse {
                    key_access_servers: key_access_servers__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.ListKeyAccessServersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListKeyMappingsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pagination.is_some() {
            len += 1;
        }
        if self.identifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.ListKeyMappingsRequest", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        if let Some(v) = self.identifier.as_ref() {
            match v {
                list_key_mappings_request::Identifier::Id(v) => {
                    struct_ser.serialize_field("id", v)?;
                }
                list_key_mappings_request::Identifier::Key(v) => {
                    struct_ser.serialize_field("key", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListKeyMappingsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pagination",
            "id",
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pagination,
            Id,
            Key,
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
                            "pagination" => Ok(GeneratedField::Pagination),
                            "id" => Ok(GeneratedField::Id),
                            "key" => Ok(GeneratedField::Key),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListKeyMappingsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.ListKeyMappingsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListKeyMappingsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pagination__ = None;
                let mut identifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                        GeneratedField::Id => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(list_key_mappings_request::Identifier::Id);
                        }
                        GeneratedField::Key => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(list_key_mappings_request::Identifier::Key)
;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ListKeyMappingsRequest {
                    pagination: pagination__,
                    identifier: identifier__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.ListKeyMappingsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListKeyMappingsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key_mappings.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.ListKeyMappingsResponse", len)?;
        if !self.key_mappings.is_empty() {
            struct_ser.serialize_field("keyMappings", &self.key_mappings)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListKeyMappingsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key_mappings",
            "keyMappings",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KeyMappings,
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
                            "keyMappings" | "key_mappings" => Ok(GeneratedField::KeyMappings),
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
            type Value = ListKeyMappingsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.ListKeyMappingsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListKeyMappingsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key_mappings__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KeyMappings => {
                            if key_mappings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyMappings"));
                            }
                            key_mappings__ = Some(map_.next_value()?);
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
                Ok(ListKeyMappingsResponse {
                    key_mappings: key_mappings__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.ListKeyMappingsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListKeysRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key_algorithm != 0 {
            len += 1;
        }
        if self.legacy.is_some() {
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
        if self.kas_filter.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.ListKeysRequest", len)?;
        if self.key_algorithm != 0 {
            let v = super::Algorithm::try_from(self.key_algorithm)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.key_algorithm)))?;
            struct_ser.serialize_field("keyAlgorithm", &v)?;
        }
        if let Some(v) = self.legacy.as_ref() {
            struct_ser.serialize_field("legacy", v)?;
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
        if let Some(v) = self.kas_filter.as_ref() {
            match v {
                list_keys_request::KasFilter::KasId(v) => {
                    struct_ser.serialize_field("kasId", v)?;
                }
                list_keys_request::KasFilter::KasName(v) => {
                    struct_ser.serialize_field("kasName", v)?;
                }
                list_keys_request::KasFilter::KasUri(v) => {
                    struct_ser.serialize_field("kasUri", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListKeysRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key_algorithm",
            "keyAlgorithm",
            "legacy",
            "pagination",
            "sort",
            "search",
            "kas_id",
            "kasId",
            "kas_name",
            "kasName",
            "kas_uri",
            "kasUri",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KeyAlgorithm,
            Legacy,
            Pagination,
            Sort,
            Search,
            KasId,
            KasName,
            KasUri,
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
                            "keyAlgorithm" | "key_algorithm" => Ok(GeneratedField::KeyAlgorithm),
                            "legacy" => Ok(GeneratedField::Legacy),
                            "pagination" => Ok(GeneratedField::Pagination),
                            "sort" => Ok(GeneratedField::Sort),
                            "search" => Ok(GeneratedField::Search),
                            "kasId" | "kas_id" => Ok(GeneratedField::KasId),
                            "kasName" | "kas_name" => Ok(GeneratedField::KasName),
                            "kasUri" | "kas_uri" => Ok(GeneratedField::KasUri),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListKeysRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.ListKeysRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListKeysRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key_algorithm__ = None;
                let mut legacy__ = None;
                let mut pagination__ = None;
                let mut sort__ = None;
                let mut search__ = None;
                let mut kas_filter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KeyAlgorithm => {
                            if key_algorithm__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyAlgorithm"));
                            }
                            key_algorithm__ = Some(map_.next_value::<super::Algorithm>()? as i32);
                        }
                        GeneratedField::Legacy => {
                            if legacy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("legacy"));
                            }
                            legacy__ = map_.next_value()?;
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
                        GeneratedField::KasId => {
                            if kas_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasId"));
                            }
                            kas_filter__ = map_.next_value::<::std::option::Option<_>>()?.map(list_keys_request::KasFilter::KasId);
                        }
                        GeneratedField::KasName => {
                            if kas_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasName"));
                            }
                            kas_filter__ = map_.next_value::<::std::option::Option<_>>()?.map(list_keys_request::KasFilter::KasName);
                        }
                        GeneratedField::KasUri => {
                            if kas_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasUri"));
                            }
                            kas_filter__ = map_.next_value::<::std::option::Option<_>>()?.map(list_keys_request::KasFilter::KasUri);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ListKeysRequest {
                    key_algorithm: key_algorithm__.unwrap_or_default(),
                    legacy: legacy__,
                    pagination: pagination__,
                    sort: sort__.unwrap_or_default(),
                    search: search__,
                    kas_filter: kas_filter__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.ListKeysRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListKeysResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.kas_keys.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.ListKeysResponse", len)?;
        if !self.kas_keys.is_empty() {
            struct_ser.serialize_field("kasKeys", &self.kas_keys)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListKeysResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kas_keys",
            "kasKeys",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KasKeys,
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
                            "kasKeys" | "kas_keys" => Ok(GeneratedField::KasKeys),
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
            type Value = ListKeysResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.ListKeysResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListKeysResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kas_keys__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KasKeys => {
                            if kas_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasKeys"));
                            }
                            kas_keys__ = Some(map_.next_value()?);
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
                Ok(ListKeysResponse {
                    kas_keys: kas_keys__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.ListKeysResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPublicKeyMappingRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.public_key_id.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        if self.kas_filter.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.ListPublicKeyMappingRequest", len)?;
        if !self.public_key_id.is_empty() {
            struct_ser.serialize_field("publicKeyId", &self.public_key_id)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        if let Some(v) = self.kas_filter.as_ref() {
            match v {
                list_public_key_mapping_request::KasFilter::KasId(v) => {
                    struct_ser.serialize_field("kasId", v)?;
                }
                list_public_key_mapping_request::KasFilter::KasName(v) => {
                    struct_ser.serialize_field("kasName", v)?;
                }
                list_public_key_mapping_request::KasFilter::KasUri(v) => {
                    struct_ser.serialize_field("kasUri", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPublicKeyMappingRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "public_key_id",
            "publicKeyId",
            "pagination",
            "kas_id",
            "kasId",
            "kas_name",
            "kasName",
            "kas_uri",
            "kasUri",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PublicKeyId,
            Pagination,
            KasId,
            KasName,
            KasUri,
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
                            "publicKeyId" | "public_key_id" => Ok(GeneratedField::PublicKeyId),
                            "pagination" => Ok(GeneratedField::Pagination),
                            "kasId" | "kas_id" => Ok(GeneratedField::KasId),
                            "kasName" | "kas_name" => Ok(GeneratedField::KasName),
                            "kasUri" | "kas_uri" => Ok(GeneratedField::KasUri),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListPublicKeyMappingRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.ListPublicKeyMappingRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListPublicKeyMappingRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut public_key_id__ = None;
                let mut pagination__ = None;
                let mut kas_filter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PublicKeyId => {
                            if public_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicKeyId"));
                            }
                            public_key_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                        GeneratedField::KasId => {
                            if kas_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasId"));
                            }
                            kas_filter__ = map_.next_value::<::std::option::Option<_>>()?.map(list_public_key_mapping_request::KasFilter::KasId);
                        }
                        GeneratedField::KasName => {
                            if kas_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasName"));
                            }
                            kas_filter__ = map_.next_value::<::std::option::Option<_>>()?.map(list_public_key_mapping_request::KasFilter::KasName);
                        }
                        GeneratedField::KasUri => {
                            if kas_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasUri"));
                            }
                            kas_filter__ = map_.next_value::<::std::option::Option<_>>()?.map(list_public_key_mapping_request::KasFilter::KasUri);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ListPublicKeyMappingRequest {
                    public_key_id: public_key_id__.unwrap_or_default(),
                    pagination: pagination__,
                    kas_filter: kas_filter__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.ListPublicKeyMappingRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPublicKeyMappingResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.public_key_mappings.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.ListPublicKeyMappingResponse", len)?;
        if !self.public_key_mappings.is_empty() {
            struct_ser.serialize_field("publicKeyMappings", &self.public_key_mappings)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPublicKeyMappingResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "public_key_mappings",
            "publicKeyMappings",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PublicKeyMappings,
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
                            "publicKeyMappings" | "public_key_mappings" => Ok(GeneratedField::PublicKeyMappings),
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
            type Value = ListPublicKeyMappingResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.ListPublicKeyMappingResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListPublicKeyMappingResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut public_key_mappings__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PublicKeyMappings => {
                            if public_key_mappings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicKeyMappings"));
                            }
                            public_key_mappings__ = Some(map_.next_value()?);
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
                Ok(ListPublicKeyMappingResponse {
                    public_key_mappings: public_key_mappings__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.ListPublicKeyMappingResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for list_public_key_mapping_response::Association {
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
        if !self.fqn.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.ListPublicKeyMappingResponse.Association", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.fqn.is_empty() {
            struct_ser.serialize_field("fqn", &self.fqn)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for list_public_key_mapping_response::Association {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "fqn",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
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
            type Value = list_public_key_mapping_response::Association;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.ListPublicKeyMappingResponse.Association")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<list_public_key_mapping_response::Association, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut fqn__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fqn => {
                            if fqn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fqn"));
                            }
                            fqn__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(list_public_key_mapping_response::Association {
                    id: id__.unwrap_or_default(),
                    fqn: fqn__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.ListPublicKeyMappingResponse.Association", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for list_public_key_mapping_response::PublicKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key.is_some() {
            len += 1;
        }
        if !self.values.is_empty() {
            len += 1;
        }
        if !self.definitions.is_empty() {
            len += 1;
        }
        if !self.namespaces.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.ListPublicKeyMappingResponse.PublicKey", len)?;
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        if !self.definitions.is_empty() {
            struct_ser.serialize_field("definitions", &self.definitions)?;
        }
        if !self.namespaces.is_empty() {
            struct_ser.serialize_field("namespaces", &self.namespaces)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for list_public_key_mapping_response::PublicKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "values",
            "definitions",
            "namespaces",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Values,
            Definitions,
            Namespaces,
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
                            "key" => Ok(GeneratedField::Key),
                            "values" => Ok(GeneratedField::Values),
                            "definitions" => Ok(GeneratedField::Definitions),
                            "namespaces" => Ok(GeneratedField::Namespaces),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = list_public_key_mapping_response::PublicKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.ListPublicKeyMappingResponse.PublicKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<list_public_key_mapping_response::PublicKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut values__ = None;
                let mut definitions__ = None;
                let mut namespaces__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map_.next_value()?;
                        }
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Definitions => {
                            if definitions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("definitions"));
                            }
                            definitions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Namespaces => {
                            if namespaces__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaces"));
                            }
                            namespaces__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(list_public_key_mapping_response::PublicKey {
                    key: key__,
                    values: values__.unwrap_or_default(),
                    definitions: definitions__.unwrap_or_default(),
                    namespaces: namespaces__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.ListPublicKeyMappingResponse.PublicKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for list_public_key_mapping_response::PublicKeyMapping {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.kas_id.is_empty() {
            len += 1;
        }
        if !self.kas_name.is_empty() {
            len += 1;
        }
        if !self.kas_uri.is_empty() {
            len += 1;
        }
        if !self.public_keys.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.ListPublicKeyMappingResponse.PublicKeyMapping", len)?;
        if !self.kas_id.is_empty() {
            struct_ser.serialize_field("kasId", &self.kas_id)?;
        }
        if !self.kas_name.is_empty() {
            struct_ser.serialize_field("kasName", &self.kas_name)?;
        }
        if !self.kas_uri.is_empty() {
            struct_ser.serialize_field("kasUri", &self.kas_uri)?;
        }
        if !self.public_keys.is_empty() {
            struct_ser.serialize_field("publicKeys", &self.public_keys)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for list_public_key_mapping_response::PublicKeyMapping {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kas_id",
            "kasId",
            "kas_name",
            "kasName",
            "kas_uri",
            "kasUri",
            "public_keys",
            "publicKeys",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KasId,
            KasName,
            KasUri,
            PublicKeys,
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
                            "kasId" | "kas_id" => Ok(GeneratedField::KasId),
                            "kasName" | "kas_name" => Ok(GeneratedField::KasName),
                            "kasUri" | "kas_uri" => Ok(GeneratedField::KasUri),
                            "publicKeys" | "public_keys" => Ok(GeneratedField::PublicKeys),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = list_public_key_mapping_response::PublicKeyMapping;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.ListPublicKeyMappingResponse.PublicKeyMapping")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<list_public_key_mapping_response::PublicKeyMapping, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kas_id__ = None;
                let mut kas_name__ = None;
                let mut kas_uri__ = None;
                let mut public_keys__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KasId => {
                            if kas_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasId"));
                            }
                            kas_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KasName => {
                            if kas_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasName"));
                            }
                            kas_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KasUri => {
                            if kas_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasUri"));
                            }
                            kas_uri__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PublicKeys => {
                            if public_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicKeys"));
                            }
                            public_keys__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(list_public_key_mapping_response::PublicKeyMapping {
                    kas_id: kas_id__.unwrap_or_default(),
                    kas_name: kas_name__.unwrap_or_default(),
                    kas_uri: kas_uri__.unwrap_or_default(),
                    public_keys: public_keys__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.ListPublicKeyMappingResponse.PublicKeyMapping", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPublicKeysRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pagination.is_some() {
            len += 1;
        }
        if self.kas_filter.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.ListPublicKeysRequest", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        if let Some(v) = self.kas_filter.as_ref() {
            match v {
                list_public_keys_request::KasFilter::KasId(v) => {
                    struct_ser.serialize_field("kasId", v)?;
                }
                list_public_keys_request::KasFilter::KasName(v) => {
                    struct_ser.serialize_field("kasName", v)?;
                }
                list_public_keys_request::KasFilter::KasUri(v) => {
                    struct_ser.serialize_field("kasUri", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPublicKeysRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pagination",
            "kas_id",
            "kasId",
            "kas_name",
            "kasName",
            "kas_uri",
            "kasUri",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pagination,
            KasId,
            KasName,
            KasUri,
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
                            "pagination" => Ok(GeneratedField::Pagination),
                            "kasId" | "kas_id" => Ok(GeneratedField::KasId),
                            "kasName" | "kas_name" => Ok(GeneratedField::KasName),
                            "kasUri" | "kas_uri" => Ok(GeneratedField::KasUri),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListPublicKeysRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.ListPublicKeysRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListPublicKeysRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pagination__ = None;
                let mut kas_filter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                        GeneratedField::KasId => {
                            if kas_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasId"));
                            }
                            kas_filter__ = map_.next_value::<::std::option::Option<_>>()?.map(list_public_keys_request::KasFilter::KasId);
                        }
                        GeneratedField::KasName => {
                            if kas_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasName"));
                            }
                            kas_filter__ = map_.next_value::<::std::option::Option<_>>()?.map(list_public_keys_request::KasFilter::KasName);
                        }
                        GeneratedField::KasUri => {
                            if kas_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasUri"));
                            }
                            kas_filter__ = map_.next_value::<::std::option::Option<_>>()?.map(list_public_keys_request::KasFilter::KasUri);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ListPublicKeysRequest {
                    pagination: pagination__,
                    kas_filter: kas_filter__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.ListPublicKeysRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPublicKeysResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.keys.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.ListPublicKeysResponse", len)?;
        if !self.keys.is_empty() {
            struct_ser.serialize_field("keys", &self.keys)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPublicKeysResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "keys",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Keys,
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
                            "keys" => Ok(GeneratedField::Keys),
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
            type Value = ListPublicKeysResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.ListPublicKeysResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListPublicKeysResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut keys__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Keys => {
                            if keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keys"));
                            }
                            keys__ = Some(map_.next_value()?);
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
                Ok(ListPublicKeysResponse {
                    keys: keys__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.ListPublicKeysResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MappedPolicyObject {
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
        if !self.fqn.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.MappedPolicyObject", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.fqn.is_empty() {
            struct_ser.serialize_field("fqn", &self.fqn)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MappedPolicyObject {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "fqn",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
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
            type Value = MappedPolicyObject;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.MappedPolicyObject")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MappedPolicyObject, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut fqn__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fqn => {
                            if fqn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fqn"));
                            }
                            fqn__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MappedPolicyObject {
                    id: id__.unwrap_or_default(),
                    fqn: fqn__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.MappedPolicyObject", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RotateKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.new_key.is_some() {
            len += 1;
        }
        if self.active_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.RotateKeyRequest", len)?;
        if let Some(v) = self.new_key.as_ref() {
            struct_ser.serialize_field("newKey", v)?;
        }
        if let Some(v) = self.active_key.as_ref() {
            match v {
                rotate_key_request::ActiveKey::Id(v) => {
                    struct_ser.serialize_field("id", v)?;
                }
                rotate_key_request::ActiveKey::Key(v) => {
                    struct_ser.serialize_field("key", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RotateKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "new_key",
            "newKey",
            "id",
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NewKey,
            Id,
            Key,
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
                            "newKey" | "new_key" => Ok(GeneratedField::NewKey),
                            "id" => Ok(GeneratedField::Id),
                            "key" => Ok(GeneratedField::Key),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RotateKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.RotateKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RotateKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut new_key__ = None;
                let mut active_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NewKey => {
                            if new_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newKey"));
                            }
                            new_key__ = map_.next_value()?;
                        }
                        GeneratedField::Id => {
                            if active_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            active_key__ = map_.next_value::<::std::option::Option<_>>()?.map(rotate_key_request::ActiveKey::Id);
                        }
                        GeneratedField::Key => {
                            if active_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            active_key__ = map_.next_value::<::std::option::Option<_>>()?.map(rotate_key_request::ActiveKey::Key)
;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(RotateKeyRequest {
                    new_key: new_key__,
                    active_key: active_key__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.RotateKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rotate_key_request::NewKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key_id.is_empty() {
            len += 1;
        }
        if self.algorithm != 0 {
            len += 1;
        }
        if self.key_mode != 0 {
            len += 1;
        }
        if self.public_key_ctx.is_some() {
            len += 1;
        }
        if self.private_key_ctx.is_some() {
            len += 1;
        }
        if !self.provider_config_id.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.RotateKeyRequest.NewKey", len)?;
        if !self.key_id.is_empty() {
            struct_ser.serialize_field("keyId", &self.key_id)?;
        }
        if self.algorithm != 0 {
            let v = super::Algorithm::try_from(self.algorithm)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.algorithm)))?;
            struct_ser.serialize_field("algorithm", &v)?;
        }
        if self.key_mode != 0 {
            let v = super::KeyMode::try_from(self.key_mode)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.key_mode)))?;
            struct_ser.serialize_field("keyMode", &v)?;
        }
        if let Some(v) = self.public_key_ctx.as_ref() {
            struct_ser.serialize_field("publicKeyCtx", v)?;
        }
        if let Some(v) = self.private_key_ctx.as_ref() {
            struct_ser.serialize_field("privateKeyCtx", v)?;
        }
        if !self.provider_config_id.is_empty() {
            struct_ser.serialize_field("providerConfigId", &self.provider_config_id)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rotate_key_request::NewKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key_id",
            "keyId",
            "algorithm",
            "key_mode",
            "keyMode",
            "public_key_ctx",
            "publicKeyCtx",
            "private_key_ctx",
            "privateKeyCtx",
            "provider_config_id",
            "providerConfigId",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KeyId,
            Algorithm,
            KeyMode,
            PublicKeyCtx,
            PrivateKeyCtx,
            ProviderConfigId,
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
                            "keyId" | "key_id" => Ok(GeneratedField::KeyId),
                            "algorithm" => Ok(GeneratedField::Algorithm),
                            "keyMode" | "key_mode" => Ok(GeneratedField::KeyMode),
                            "publicKeyCtx" | "public_key_ctx" => Ok(GeneratedField::PublicKeyCtx),
                            "privateKeyCtx" | "private_key_ctx" => Ok(GeneratedField::PrivateKeyCtx),
                            "providerConfigId" | "provider_config_id" => Ok(GeneratedField::ProviderConfigId),
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
            type Value = rotate_key_request::NewKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.RotateKeyRequest.NewKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<rotate_key_request::NewKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key_id__ = None;
                let mut algorithm__ = None;
                let mut key_mode__ = None;
                let mut public_key_ctx__ = None;
                let mut private_key_ctx__ = None;
                let mut provider_config_id__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KeyId => {
                            if key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyId"));
                            }
                            key_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Algorithm => {
                            if algorithm__.is_some() {
                                return Err(serde::de::Error::duplicate_field("algorithm"));
                            }
                            algorithm__ = Some(map_.next_value::<super::Algorithm>()? as i32);
                        }
                        GeneratedField::KeyMode => {
                            if key_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyMode"));
                            }
                            key_mode__ = Some(map_.next_value::<super::KeyMode>()? as i32);
                        }
                        GeneratedField::PublicKeyCtx => {
                            if public_key_ctx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicKeyCtx"));
                            }
                            public_key_ctx__ = map_.next_value()?;
                        }
                        GeneratedField::PrivateKeyCtx => {
                            if private_key_ctx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privateKeyCtx"));
                            }
                            private_key_ctx__ = map_.next_value()?;
                        }
                        GeneratedField::ProviderConfigId => {
                            if provider_config_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerConfigId"));
                            }
                            provider_config_id__ = Some(map_.next_value()?);
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
                Ok(rotate_key_request::NewKey {
                    key_id: key_id__.unwrap_or_default(),
                    algorithm: algorithm__.unwrap_or_default(),
                    key_mode: key_mode__.unwrap_or_default(),
                    public_key_ctx: public_key_ctx__,
                    private_key_ctx: private_key_ctx__,
                    provider_config_id: provider_config_id__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.RotateKeyRequest.NewKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RotateKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.kas_key.is_some() {
            len += 1;
        }
        if self.rotated_resources.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.RotateKeyResponse", len)?;
        if let Some(v) = self.kas_key.as_ref() {
            struct_ser.serialize_field("kasKey", v)?;
        }
        if let Some(v) = self.rotated_resources.as_ref() {
            struct_ser.serialize_field("rotatedResources", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RotateKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kas_key",
            "kasKey",
            "rotated_resources",
            "rotatedResources",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KasKey,
            RotatedResources,
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
                            "kasKey" | "kas_key" => Ok(GeneratedField::KasKey),
                            "rotatedResources" | "rotated_resources" => Ok(GeneratedField::RotatedResources),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RotateKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.RotateKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RotateKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kas_key__ = None;
                let mut rotated_resources__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KasKey => {
                            if kas_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasKey"));
                            }
                            kas_key__ = map_.next_value()?;
                        }
                        GeneratedField::RotatedResources => {
                            if rotated_resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rotatedResources"));
                            }
                            rotated_resources__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(RotateKeyResponse {
                    kas_key: kas_key__,
                    rotated_resources: rotated_resources__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.RotateKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RotatedResources {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rotated_out_key.is_some() {
            len += 1;
        }
        if !self.attribute_definition_mappings.is_empty() {
            len += 1;
        }
        if !self.attribute_value_mappings.is_empty() {
            len += 1;
        }
        if !self.namespace_mappings.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.RotatedResources", len)?;
        if let Some(v) = self.rotated_out_key.as_ref() {
            struct_ser.serialize_field("rotatedOutKey", v)?;
        }
        if !self.attribute_definition_mappings.is_empty() {
            struct_ser.serialize_field("attributeDefinitionMappings", &self.attribute_definition_mappings)?;
        }
        if !self.attribute_value_mappings.is_empty() {
            struct_ser.serialize_field("attributeValueMappings", &self.attribute_value_mappings)?;
        }
        if !self.namespace_mappings.is_empty() {
            struct_ser.serialize_field("namespaceMappings", &self.namespace_mappings)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RotatedResources {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rotated_out_key",
            "rotatedOutKey",
            "attribute_definition_mappings",
            "attributeDefinitionMappings",
            "attribute_value_mappings",
            "attributeValueMappings",
            "namespace_mappings",
            "namespaceMappings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RotatedOutKey,
            AttributeDefinitionMappings,
            AttributeValueMappings,
            NamespaceMappings,
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
                            "rotatedOutKey" | "rotated_out_key" => Ok(GeneratedField::RotatedOutKey),
                            "attributeDefinitionMappings" | "attribute_definition_mappings" => Ok(GeneratedField::AttributeDefinitionMappings),
                            "attributeValueMappings" | "attribute_value_mappings" => Ok(GeneratedField::AttributeValueMappings),
                            "namespaceMappings" | "namespace_mappings" => Ok(GeneratedField::NamespaceMappings),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RotatedResources;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.RotatedResources")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RotatedResources, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rotated_out_key__ = None;
                let mut attribute_definition_mappings__ = None;
                let mut attribute_value_mappings__ = None;
                let mut namespace_mappings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RotatedOutKey => {
                            if rotated_out_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rotatedOutKey"));
                            }
                            rotated_out_key__ = map_.next_value()?;
                        }
                        GeneratedField::AttributeDefinitionMappings => {
                            if attribute_definition_mappings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeDefinitionMappings"));
                            }
                            attribute_definition_mappings__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AttributeValueMappings => {
                            if attribute_value_mappings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeValueMappings"));
                            }
                            attribute_value_mappings__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NamespaceMappings => {
                            if namespace_mappings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespaceMappings"));
                            }
                            namespace_mappings__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(RotatedResources {
                    rotated_out_key: rotated_out_key__,
                    attribute_definition_mappings: attribute_definition_mappings__.unwrap_or_default(),
                    attribute_value_mappings: attribute_value_mappings__.unwrap_or_default(),
                    namespace_mappings: namespace_mappings__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.RotatedResources", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetBaseKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.active_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.SetBaseKeyRequest", len)?;
        if let Some(v) = self.active_key.as_ref() {
            match v {
                set_base_key_request::ActiveKey::Id(v) => {
                    struct_ser.serialize_field("id", v)?;
                }
                set_base_key_request::ActiveKey::Key(v) => {
                    struct_ser.serialize_field("key", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetBaseKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Key,
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
                            "key" => Ok(GeneratedField::Key),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetBaseKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.SetBaseKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetBaseKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut active_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if active_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            active_key__ = map_.next_value::<::std::option::Option<_>>()?.map(set_base_key_request::ActiveKey::Id);
                        }
                        GeneratedField::Key => {
                            if active_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            active_key__ = map_.next_value::<::std::option::Option<_>>()?.map(set_base_key_request::ActiveKey::Key)
;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(SetBaseKeyRequest {
                    active_key: active_key__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.SetBaseKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetBaseKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.new_base_key.is_some() {
            len += 1;
        }
        if self.previous_base_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.SetBaseKeyResponse", len)?;
        if let Some(v) = self.new_base_key.as_ref() {
            struct_ser.serialize_field("newBaseKey", v)?;
        }
        if let Some(v) = self.previous_base_key.as_ref() {
            struct_ser.serialize_field("previousBaseKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetBaseKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "new_base_key",
            "newBaseKey",
            "previous_base_key",
            "previousBaseKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NewBaseKey,
            PreviousBaseKey,
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
                            "newBaseKey" | "new_base_key" => Ok(GeneratedField::NewBaseKey),
                            "previousBaseKey" | "previous_base_key" => Ok(GeneratedField::PreviousBaseKey),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetBaseKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.SetBaseKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetBaseKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut new_base_key__ = None;
                let mut previous_base_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NewBaseKey => {
                            if new_base_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newBaseKey"));
                            }
                            new_base_key__ = map_.next_value()?;
                        }
                        GeneratedField::PreviousBaseKey => {
                            if previous_base_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("previousBaseKey"));
                            }
                            previous_base_key__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(SetBaseKeyResponse {
                    new_base_key: new_base_key__,
                    previous_base_key: previous_base_key__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.SetBaseKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SortKasKeysType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SORT_KAS_KEYS_TYPE_UNSPECIFIED",
            Self::KeyId => "SORT_KAS_KEYS_TYPE_KEY_ID",
            Self::CreatedAt => "SORT_KAS_KEYS_TYPE_CREATED_AT",
            Self::UpdatedAt => "SORT_KAS_KEYS_TYPE_UPDATED_AT",
            Self::KeyStatus => "SORT_KAS_KEYS_TYPE_KEY_STATUS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SortKasKeysType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SORT_KAS_KEYS_TYPE_UNSPECIFIED",
            "SORT_KAS_KEYS_TYPE_KEY_ID",
            "SORT_KAS_KEYS_TYPE_CREATED_AT",
            "SORT_KAS_KEYS_TYPE_UPDATED_AT",
            "SORT_KAS_KEYS_TYPE_KEY_STATUS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SortKasKeysType;

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
                    "SORT_KAS_KEYS_TYPE_UNSPECIFIED" => Ok(SortKasKeysType::Unspecified),
                    "SORT_KAS_KEYS_TYPE_KEY_ID" => Ok(SortKasKeysType::KeyId),
                    "SORT_KAS_KEYS_TYPE_CREATED_AT" => Ok(SortKasKeysType::CreatedAt),
                    "SORT_KAS_KEYS_TYPE_UPDATED_AT" => Ok(SortKasKeysType::UpdatedAt),
                    "SORT_KAS_KEYS_TYPE_KEY_STATUS" => Ok(SortKasKeysType::KeyStatus),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SortKeyAccessServersType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SORT_KEY_ACCESS_SERVERS_TYPE_UNSPECIFIED",
            Self::Name => "SORT_KEY_ACCESS_SERVERS_TYPE_NAME",
            Self::Uri => "SORT_KEY_ACCESS_SERVERS_TYPE_URI",
            Self::CreatedAt => "SORT_KEY_ACCESS_SERVERS_TYPE_CREATED_AT",
            Self::UpdatedAt => "SORT_KEY_ACCESS_SERVERS_TYPE_UPDATED_AT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SortKeyAccessServersType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SORT_KEY_ACCESS_SERVERS_TYPE_UNSPECIFIED",
            "SORT_KEY_ACCESS_SERVERS_TYPE_NAME",
            "SORT_KEY_ACCESS_SERVERS_TYPE_URI",
            "SORT_KEY_ACCESS_SERVERS_TYPE_CREATED_AT",
            "SORT_KEY_ACCESS_SERVERS_TYPE_UPDATED_AT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SortKeyAccessServersType;

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
                    "SORT_KEY_ACCESS_SERVERS_TYPE_UNSPECIFIED" => Ok(SortKeyAccessServersType::Unspecified),
                    "SORT_KEY_ACCESS_SERVERS_TYPE_NAME" => Ok(SortKeyAccessServersType::Name),
                    "SORT_KEY_ACCESS_SERVERS_TYPE_URI" => Ok(SortKeyAccessServersType::Uri),
                    "SORT_KEY_ACCESS_SERVERS_TYPE_CREATED_AT" => Ok(SortKeyAccessServersType::CreatedAt),
                    "SORT_KEY_ACCESS_SERVERS_TYPE_UPDATED_AT" => Ok(SortKeyAccessServersType::UpdatedAt),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateKeyAccessServerRequest {
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
        if !self.uri.is_empty() {
            len += 1;
        }
        if self.public_key.is_some() {
            len += 1;
        }
        if self.source_type != 0 {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        if self.metadata_update_behavior != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.UpdateKeyAccessServerRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.uri.is_empty() {
            struct_ser.serialize_field("uri", &self.uri)?;
        }
        if let Some(v) = self.public_key.as_ref() {
            struct_ser.serialize_field("publicKey", v)?;
        }
        if self.source_type != 0 {
            let v = super::SourceType::try_from(self.source_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.source_type)))?;
            struct_ser.serialize_field("sourceType", &v)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
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
impl<'de> serde::Deserialize<'de> for UpdateKeyAccessServerRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "uri",
            "public_key",
            "publicKey",
            "source_type",
            "sourceType",
            "name",
            "metadata",
            "metadata_update_behavior",
            "metadataUpdateBehavior",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Uri,
            PublicKey,
            SourceType,
            Name,
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
                            "uri" => Ok(GeneratedField::Uri),
                            "publicKey" | "public_key" => Ok(GeneratedField::PublicKey),
                            "sourceType" | "source_type" => Ok(GeneratedField::SourceType),
                            "name" => Ok(GeneratedField::Name),
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
            type Value = UpdateKeyAccessServerRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.UpdateKeyAccessServerRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateKeyAccessServerRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut uri__ = None;
                let mut public_key__ = None;
                let mut source_type__ = None;
                let mut name__ = None;
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
                        GeneratedField::Uri => {
                            if uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uri"));
                            }
                            uri__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PublicKey => {
                            if public_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicKey"));
                            }
                            public_key__ = map_.next_value()?;
                        }
                        GeneratedField::SourceType => {
                            if source_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceType"));
                            }
                            source_type__ = Some(map_.next_value::<super::SourceType>()? as i32);
                        }
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
                Ok(UpdateKeyAccessServerRequest {
                    id: id__.unwrap_or_default(),
                    uri: uri__.unwrap_or_default(),
                    public_key: public_key__,
                    source_type: source_type__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    metadata: metadata__,
                    metadata_update_behavior: metadata_update_behavior__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.UpdateKeyAccessServerRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateKeyAccessServerResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key_access_server.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.UpdateKeyAccessServerResponse", len)?;
        if let Some(v) = self.key_access_server.as_ref() {
            struct_ser.serialize_field("keyAccessServer", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateKeyAccessServerResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key_access_server",
            "keyAccessServer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KeyAccessServer,
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
                            "keyAccessServer" | "key_access_server" => Ok(GeneratedField::KeyAccessServer),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateKeyAccessServerResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.UpdateKeyAccessServerResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateKeyAccessServerResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key_access_server__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KeyAccessServer => {
                            if key_access_server__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyAccessServer"));
                            }
                            key_access_server__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(UpdateKeyAccessServerResponse {
                    key_access_server: key_access_server__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.UpdateKeyAccessServerResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateKeyRequest {
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
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.UpdateKeyRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for UpdateKeyRequest {
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
            type Value = UpdateKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.UpdateKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateKeyRequest, V::Error>
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
                Ok(UpdateKeyRequest {
                    id: id__.unwrap_or_default(),
                    metadata: metadata__,
                    metadata_update_behavior: metadata_update_behavior__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.UpdateKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.kas_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.UpdateKeyResponse", len)?;
        if let Some(v) = self.kas_key.as_ref() {
            struct_ser.serialize_field("kasKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kas_key",
            "kasKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KasKey,
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
                            "kasKey" | "kas_key" => Ok(GeneratedField::KasKey),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.UpdateKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kas_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KasKey => {
                            if kas_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasKey"));
                            }
                            kas_key__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(UpdateKeyResponse {
                    kas_key: kas_key__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.UpdateKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdatePublicKeyRequest {
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
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.UpdatePublicKeyRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for UpdatePublicKeyRequest {
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
            type Value = UpdatePublicKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.UpdatePublicKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdatePublicKeyRequest, V::Error>
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
                Ok(UpdatePublicKeyRequest {
                    id: id__.unwrap_or_default(),
                    metadata: metadata__,
                    metadata_update_behavior: metadata_update_behavior__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.UpdatePublicKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdatePublicKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.kasregistry.UpdatePublicKeyResponse", len)?;
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdatePublicKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
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
                            "key" => Ok(GeneratedField::Key),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdatePublicKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.kasregistry.UpdatePublicKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdatePublicKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(UpdatePublicKeyResponse {
                    key: key__,
                })
            }
        }
        deserializer.deserialize_struct("policy.kasregistry.UpdatePublicKeyResponse", FIELDS, GeneratedVisitor)
    }
}
