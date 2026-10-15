// @generated
impl serde::Serialize for Action {
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
        if !self.name.is_empty() {
            len += 1;
        }
        if self.namespace.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.Action", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.namespace.as_ref() {
            struct_ser.serialize_field("namespace", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            match v {
                action::Value::Standard(v) => {
                    let v = action::StandardAction::try_from(*v)
                        .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("standard", &v)?;
                }
                action::Value::Custom(v) => {
                    struct_ser.serialize_field("custom", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Action {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "namespace",
            "metadata",
            "standard",
            "custom",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Namespace,
            Metadata,
            Standard,
            Custom,
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
                            "name" => Ok(GeneratedField::Name),
                            "namespace" => Ok(GeneratedField::Namespace),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "standard" => Ok(GeneratedField::Standard),
                            "custom" => Ok(GeneratedField::Custom),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Action;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.Action")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Action, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut namespace__ = None;
                let mut metadata__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Namespace => {
                            if namespace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespace"));
                            }
                            namespace__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::Standard => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("standard"));
                            }
                            value__ = map_.next_value::<::std::option::Option<action::StandardAction>>()?.map(|x| action::Value::Standard(x as i32));
                        }
                        GeneratedField::Custom => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("custom"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(action::Value::Custom);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Action {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    namespace: namespace__,
                    metadata: metadata__,
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("policy.Action", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for action::StandardAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "STANDARD_ACTION_UNSPECIFIED",
            Self::Decrypt => "STANDARD_ACTION_DECRYPT",
            Self::Transmit => "STANDARD_ACTION_TRANSMIT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for action::StandardAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STANDARD_ACTION_UNSPECIFIED",
            "STANDARD_ACTION_DECRYPT",
            "STANDARD_ACTION_TRANSMIT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = action::StandardAction;

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
                    "STANDARD_ACTION_UNSPECIFIED" => Ok(action::StandardAction::Unspecified),
                    "STANDARD_ACTION_DECRYPT" => Ok(action::StandardAction::Decrypt),
                    "STANDARD_ACTION_TRANSMIT" => Ok(action::StandardAction::Transmit),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Algorithm {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ALGORITHM_UNSPECIFIED",
            Self::Rsa2048 => "ALGORITHM_RSA_2048",
            Self::Rsa4096 => "ALGORITHM_RSA_4096",
            Self::EcP256 => "ALGORITHM_EC_P256",
            Self::EcP384 => "ALGORITHM_EC_P384",
            Self::EcP521 => "ALGORITHM_EC_P521",
            Self::HpqtXwing => "ALGORITHM_HPQT_XWING",
            Self::HpqtSecp256r1Mlkem768 => "ALGORITHM_HPQT_SECP256R1_MLKEM768",
            Self::HpqtSecp384r1Mlkem1024 => "ALGORITHM_HPQT_SECP384R1_MLKEM1024",
            Self::Mlkem768 => "ALGORITHM_MLKEM_768",
            Self::Mlkem1024 => "ALGORITHM_MLKEM_1024",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Algorithm {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ALGORITHM_UNSPECIFIED",
            "ALGORITHM_RSA_2048",
            "ALGORITHM_RSA_4096",
            "ALGORITHM_EC_P256",
            "ALGORITHM_EC_P384",
            "ALGORITHM_EC_P521",
            "ALGORITHM_HPQT_XWING",
            "ALGORITHM_HPQT_SECP256R1_MLKEM768",
            "ALGORITHM_HPQT_SECP384R1_MLKEM1024",
            "ALGORITHM_MLKEM_768",
            "ALGORITHM_MLKEM_1024",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Algorithm;

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
                    "ALGORITHM_UNSPECIFIED" => Ok(Algorithm::Unspecified),
                    "ALGORITHM_RSA_2048" => Ok(Algorithm::Rsa2048),
                    "ALGORITHM_RSA_4096" => Ok(Algorithm::Rsa4096),
                    "ALGORITHM_EC_P256" => Ok(Algorithm::EcP256),
                    "ALGORITHM_EC_P384" => Ok(Algorithm::EcP384),
                    "ALGORITHM_EC_P521" => Ok(Algorithm::EcP521),
                    "ALGORITHM_HPQT_XWING" => Ok(Algorithm::HpqtXwing),
                    "ALGORITHM_HPQT_SECP256R1_MLKEM768" => Ok(Algorithm::HpqtSecp256r1Mlkem768),
                    "ALGORITHM_HPQT_SECP384R1_MLKEM1024" => Ok(Algorithm::HpqtSecp384r1Mlkem1024),
                    "ALGORITHM_MLKEM_768" => Ok(Algorithm::Mlkem768),
                    "ALGORITHM_MLKEM_1024" => Ok(Algorithm::Mlkem1024),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AsymmetricKey {
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
        if !self.key_id.is_empty() {
            len += 1;
        }
        if self.key_algorithm != 0 {
            len += 1;
        }
        if self.key_status != 0 {
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
        if self.provider_config.is_some() {
            len += 1;
        }
        if self.legacy {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.AsymmetricKey", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.key_id.is_empty() {
            struct_ser.serialize_field("keyId", &self.key_id)?;
        }
        if self.key_algorithm != 0 {
            let v = Algorithm::try_from(self.key_algorithm)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.key_algorithm)))?;
            struct_ser.serialize_field("keyAlgorithm", &v)?;
        }
        if self.key_status != 0 {
            let v = KeyStatus::try_from(self.key_status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.key_status)))?;
            struct_ser.serialize_field("keyStatus", &v)?;
        }
        if self.key_mode != 0 {
            let v = KeyMode::try_from(self.key_mode)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.key_mode)))?;
            struct_ser.serialize_field("keyMode", &v)?;
        }
        if let Some(v) = self.public_key_ctx.as_ref() {
            struct_ser.serialize_field("publicKeyCtx", v)?;
        }
        if let Some(v) = self.private_key_ctx.as_ref() {
            struct_ser.serialize_field("privateKeyCtx", v)?;
        }
        if let Some(v) = self.provider_config.as_ref() {
            struct_ser.serialize_field("providerConfig", v)?;
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
impl<'de> serde::Deserialize<'de> for AsymmetricKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "key_id",
            "keyId",
            "key_algorithm",
            "keyAlgorithm",
            "key_status",
            "keyStatus",
            "key_mode",
            "keyMode",
            "public_key_ctx",
            "publicKeyCtx",
            "private_key_ctx",
            "privateKeyCtx",
            "provider_config",
            "providerConfig",
            "legacy",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            KeyId,
            KeyAlgorithm,
            KeyStatus,
            KeyMode,
            PublicKeyCtx,
            PrivateKeyCtx,
            ProviderConfig,
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
                            "id" => Ok(GeneratedField::Id),
                            "keyId" | "key_id" => Ok(GeneratedField::KeyId),
                            "keyAlgorithm" | "key_algorithm" => Ok(GeneratedField::KeyAlgorithm),
                            "keyStatus" | "key_status" => Ok(GeneratedField::KeyStatus),
                            "keyMode" | "key_mode" => Ok(GeneratedField::KeyMode),
                            "publicKeyCtx" | "public_key_ctx" => Ok(GeneratedField::PublicKeyCtx),
                            "privateKeyCtx" | "private_key_ctx" => Ok(GeneratedField::PrivateKeyCtx),
                            "providerConfig" | "provider_config" => Ok(GeneratedField::ProviderConfig),
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
            type Value = AsymmetricKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.AsymmetricKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AsymmetricKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut key_id__ = None;
                let mut key_algorithm__ = None;
                let mut key_status__ = None;
                let mut key_mode__ = None;
                let mut public_key_ctx__ = None;
                let mut private_key_ctx__ = None;
                let mut provider_config__ = None;
                let mut legacy__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
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
                            key_algorithm__ = Some(map_.next_value::<Algorithm>()? as i32);
                        }
                        GeneratedField::KeyStatus => {
                            if key_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyStatus"));
                            }
                            key_status__ = Some(map_.next_value::<KeyStatus>()? as i32);
                        }
                        GeneratedField::KeyMode => {
                            if key_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyMode"));
                            }
                            key_mode__ = Some(map_.next_value::<KeyMode>()? as i32);
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
                        GeneratedField::ProviderConfig => {
                            if provider_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerConfig"));
                            }
                            provider_config__ = map_.next_value()?;
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
                Ok(AsymmetricKey {
                    id: id__.unwrap_or_default(),
                    key_id: key_id__.unwrap_or_default(),
                    key_algorithm: key_algorithm__.unwrap_or_default(),
                    key_status: key_status__.unwrap_or_default(),
                    key_mode: key_mode__.unwrap_or_default(),
                    public_key_ctx: public_key_ctx__,
                    private_key_ctx: private_key_ctx__,
                    provider_config: provider_config__,
                    legacy: legacy__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.AsymmetricKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Attribute {
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
        if self.namespace.is_some() {
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
        if !self.grants.is_empty() {
            len += 1;
        }
        if !self.fqn.is_empty() {
            len += 1;
        }
        if self.active.is_some() {
            len += 1;
        }
        if !self.kas_keys.is_empty() {
            len += 1;
        }
        if self.allow_traversal.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.Attribute", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.namespace.as_ref() {
            struct_ser.serialize_field("namespace", v)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.rule != 0 {
            let v = AttributeRuleTypeEnum::try_from(self.rule)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.rule)))?;
            struct_ser.serialize_field("rule", &v)?;
        }
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        if !self.grants.is_empty() {
            struct_ser.serialize_field("grants", &self.grants)?;
        }
        if !self.fqn.is_empty() {
            struct_ser.serialize_field("fqn", &self.fqn)?;
        }
        if let Some(v) = self.active.as_ref() {
            struct_ser.serialize_field("active", v)?;
        }
        if !self.kas_keys.is_empty() {
            struct_ser.serialize_field("kasKeys", &self.kas_keys)?;
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
impl<'de> serde::Deserialize<'de> for Attribute {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "namespace",
            "name",
            "rule",
            "values",
            "grants",
            "fqn",
            "active",
            "kas_keys",
            "kasKeys",
            "allow_traversal",
            "allowTraversal",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Namespace,
            Name,
            Rule,
            Values,
            Grants,
            Fqn,
            Active,
            KasKeys,
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
                            "id" => Ok(GeneratedField::Id),
                            "namespace" => Ok(GeneratedField::Namespace),
                            "name" => Ok(GeneratedField::Name),
                            "rule" => Ok(GeneratedField::Rule),
                            "values" => Ok(GeneratedField::Values),
                            "grants" => Ok(GeneratedField::Grants),
                            "fqn" => Ok(GeneratedField::Fqn),
                            "active" => Ok(GeneratedField::Active),
                            "kasKeys" | "kas_keys" => Ok(GeneratedField::KasKeys),
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
            type Value = Attribute;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.Attribute")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Attribute, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut namespace__ = None;
                let mut name__ = None;
                let mut rule__ = None;
                let mut values__ = None;
                let mut grants__ = None;
                let mut fqn__ = None;
                let mut active__ = None;
                let mut kas_keys__ = None;
                let mut allow_traversal__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Namespace => {
                            if namespace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespace"));
                            }
                            namespace__ = map_.next_value()?;
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
                            rule__ = Some(map_.next_value::<AttributeRuleTypeEnum>()? as i32);
                        }
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Grants => {
                            if grants__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grants"));
                            }
                            grants__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fqn => {
                            if fqn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fqn"));
                            }
                            fqn__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Active => {
                            if active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("active"));
                            }
                            active__ = map_.next_value()?;
                        }
                        GeneratedField::KasKeys => {
                            if kas_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasKeys"));
                            }
                            kas_keys__ = Some(map_.next_value()?);
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
                Ok(Attribute {
                    id: id__.unwrap_or_default(),
                    namespace: namespace__,
                    name: name__.unwrap_or_default(),
                    rule: rule__.unwrap_or_default(),
                    values: values__.unwrap_or_default(),
                    grants: grants__.unwrap_or_default(),
                    fqn: fqn__.unwrap_or_default(),
                    active: active__,
                    kas_keys: kas_keys__.unwrap_or_default(),
                    allow_traversal: allow_traversal__,
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.Attribute", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AttributeDefinitionSelector {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.with_key_access_grants {
            len += 1;
        }
        if self.with_namespace.is_some() {
            len += 1;
        }
        if self.with_values.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.AttributeDefinitionSelector", len)?;
        if self.with_key_access_grants {
            struct_ser.serialize_field("withKeyAccessGrants", &self.with_key_access_grants)?;
        }
        if let Some(v) = self.with_namespace.as_ref() {
            struct_ser.serialize_field("withNamespace", v)?;
        }
        if let Some(v) = self.with_values.as_ref() {
            struct_ser.serialize_field("withValues", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AttributeDefinitionSelector {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "with_key_access_grants",
            "withKeyAccessGrants",
            "with_namespace",
            "withNamespace",
            "with_values",
            "withValues",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WithKeyAccessGrants,
            WithNamespace,
            WithValues,
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
                            "withKeyAccessGrants" | "with_key_access_grants" => Ok(GeneratedField::WithKeyAccessGrants),
                            "withNamespace" | "with_namespace" => Ok(GeneratedField::WithNamespace),
                            "withValues" | "with_values" => Ok(GeneratedField::WithValues),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AttributeDefinitionSelector;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.AttributeDefinitionSelector")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AttributeDefinitionSelector, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut with_key_access_grants__ = None;
                let mut with_namespace__ = None;
                let mut with_values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WithKeyAccessGrants => {
                            if with_key_access_grants__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withKeyAccessGrants"));
                            }
                            with_key_access_grants__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WithNamespace => {
                            if with_namespace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withNamespace"));
                            }
                            with_namespace__ = map_.next_value()?;
                        }
                        GeneratedField::WithValues => {
                            if with_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withValues"));
                            }
                            with_values__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AttributeDefinitionSelector {
                    with_key_access_grants: with_key_access_grants__.unwrap_or_default(),
                    with_namespace: with_namespace__,
                    with_values: with_values__,
                })
            }
        }
        deserializer.deserialize_struct("policy.AttributeDefinitionSelector", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for attribute_definition_selector::NamespaceSelector {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("policy.AttributeDefinitionSelector.NamespaceSelector", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for attribute_definition_selector::NamespaceSelector {
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
            type Value = attribute_definition_selector::NamespaceSelector;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.AttributeDefinitionSelector.NamespaceSelector")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<attribute_definition_selector::NamespaceSelector, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(attribute_definition_selector::NamespaceSelector {
                })
            }
        }
        deserializer.deserialize_struct("policy.AttributeDefinitionSelector.NamespaceSelector", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for attribute_definition_selector::ValueSelector {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.with_key_access_grants {
            len += 1;
        }
        if self.with_subject_maps {
            len += 1;
        }
        if self.with_resource_maps {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.AttributeDefinitionSelector.ValueSelector", len)?;
        if self.with_key_access_grants {
            struct_ser.serialize_field("withKeyAccessGrants", &self.with_key_access_grants)?;
        }
        if self.with_subject_maps {
            struct_ser.serialize_field("withSubjectMaps", &self.with_subject_maps)?;
        }
        if self.with_resource_maps {
            struct_ser.serialize_field("withResourceMaps", &self.with_resource_maps)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for attribute_definition_selector::ValueSelector {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "with_key_access_grants",
            "withKeyAccessGrants",
            "with_subject_maps",
            "withSubjectMaps",
            "with_resource_maps",
            "withResourceMaps",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WithKeyAccessGrants,
            WithSubjectMaps,
            WithResourceMaps,
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
                            "withKeyAccessGrants" | "with_key_access_grants" => Ok(GeneratedField::WithKeyAccessGrants),
                            "withSubjectMaps" | "with_subject_maps" => Ok(GeneratedField::WithSubjectMaps),
                            "withResourceMaps" | "with_resource_maps" => Ok(GeneratedField::WithResourceMaps),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = attribute_definition_selector::ValueSelector;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.AttributeDefinitionSelector.ValueSelector")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<attribute_definition_selector::ValueSelector, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut with_key_access_grants__ = None;
                let mut with_subject_maps__ = None;
                let mut with_resource_maps__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WithKeyAccessGrants => {
                            if with_key_access_grants__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withKeyAccessGrants"));
                            }
                            with_key_access_grants__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WithSubjectMaps => {
                            if with_subject_maps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withSubjectMaps"));
                            }
                            with_subject_maps__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WithResourceMaps => {
                            if with_resource_maps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withResourceMaps"));
                            }
                            with_resource_maps__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(attribute_definition_selector::ValueSelector {
                    with_key_access_grants: with_key_access_grants__.unwrap_or_default(),
                    with_subject_maps: with_subject_maps__.unwrap_or_default(),
                    with_resource_maps: with_resource_maps__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.AttributeDefinitionSelector.ValueSelector", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AttributeNamespaceSelector {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.with_attributes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.AttributeNamespaceSelector", len)?;
        if let Some(v) = self.with_attributes.as_ref() {
            struct_ser.serialize_field("withAttributes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AttributeNamespaceSelector {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "with_attributes",
            "withAttributes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WithAttributes,
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
                            "withAttributes" | "with_attributes" => Ok(GeneratedField::WithAttributes),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AttributeNamespaceSelector;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.AttributeNamespaceSelector")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AttributeNamespaceSelector, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut with_attributes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WithAttributes => {
                            if with_attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withAttributes"));
                            }
                            with_attributes__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AttributeNamespaceSelector {
                    with_attributes: with_attributes__,
                })
            }
        }
        deserializer.deserialize_struct("policy.AttributeNamespaceSelector", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for attribute_namespace_selector::AttributeSelector {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.with_key_access_grants {
            len += 1;
        }
        if self.with_values.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.AttributeNamespaceSelector.AttributeSelector", len)?;
        if self.with_key_access_grants {
            struct_ser.serialize_field("withKeyAccessGrants", &self.with_key_access_grants)?;
        }
        if let Some(v) = self.with_values.as_ref() {
            struct_ser.serialize_field("withValues", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for attribute_namespace_selector::AttributeSelector {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "with_key_access_grants",
            "withKeyAccessGrants",
            "with_values",
            "withValues",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WithKeyAccessGrants,
            WithValues,
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
                            "withKeyAccessGrants" | "with_key_access_grants" => Ok(GeneratedField::WithKeyAccessGrants),
                            "withValues" | "with_values" => Ok(GeneratedField::WithValues),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = attribute_namespace_selector::AttributeSelector;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.AttributeNamespaceSelector.AttributeSelector")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<attribute_namespace_selector::AttributeSelector, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut with_key_access_grants__ = None;
                let mut with_values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WithKeyAccessGrants => {
                            if with_key_access_grants__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withKeyAccessGrants"));
                            }
                            with_key_access_grants__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WithValues => {
                            if with_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withValues"));
                            }
                            with_values__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(attribute_namespace_selector::AttributeSelector {
                    with_key_access_grants: with_key_access_grants__.unwrap_or_default(),
                    with_values: with_values__,
                })
            }
        }
        deserializer.deserialize_struct("policy.AttributeNamespaceSelector.AttributeSelector", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for attribute_namespace_selector::attribute_selector::ValueSelector {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.with_key_access_grants {
            len += 1;
        }
        if self.with_subject_maps {
            len += 1;
        }
        if self.with_resource_maps {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.AttributeNamespaceSelector.AttributeSelector.ValueSelector", len)?;
        if self.with_key_access_grants {
            struct_ser.serialize_field("withKeyAccessGrants", &self.with_key_access_grants)?;
        }
        if self.with_subject_maps {
            struct_ser.serialize_field("withSubjectMaps", &self.with_subject_maps)?;
        }
        if self.with_resource_maps {
            struct_ser.serialize_field("withResourceMaps", &self.with_resource_maps)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for attribute_namespace_selector::attribute_selector::ValueSelector {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "with_key_access_grants",
            "withKeyAccessGrants",
            "with_subject_maps",
            "withSubjectMaps",
            "with_resource_maps",
            "withResourceMaps",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WithKeyAccessGrants,
            WithSubjectMaps,
            WithResourceMaps,
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
                            "withKeyAccessGrants" | "with_key_access_grants" => Ok(GeneratedField::WithKeyAccessGrants),
                            "withSubjectMaps" | "with_subject_maps" => Ok(GeneratedField::WithSubjectMaps),
                            "withResourceMaps" | "with_resource_maps" => Ok(GeneratedField::WithResourceMaps),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = attribute_namespace_selector::attribute_selector::ValueSelector;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.AttributeNamespaceSelector.AttributeSelector.ValueSelector")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<attribute_namespace_selector::attribute_selector::ValueSelector, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut with_key_access_grants__ = None;
                let mut with_subject_maps__ = None;
                let mut with_resource_maps__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WithKeyAccessGrants => {
                            if with_key_access_grants__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withKeyAccessGrants"));
                            }
                            with_key_access_grants__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WithSubjectMaps => {
                            if with_subject_maps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withSubjectMaps"));
                            }
                            with_subject_maps__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WithResourceMaps => {
                            if with_resource_maps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withResourceMaps"));
                            }
                            with_resource_maps__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(attribute_namespace_selector::attribute_selector::ValueSelector {
                    with_key_access_grants: with_key_access_grants__.unwrap_or_default(),
                    with_subject_maps: with_subject_maps__.unwrap_or_default(),
                    with_resource_maps: with_resource_maps__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.AttributeNamespaceSelector.AttributeSelector.ValueSelector", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AttributeRuleTypeEnum {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ATTRIBUTE_RULE_TYPE_ENUM_UNSPECIFIED",
            Self::AllOf => "ATTRIBUTE_RULE_TYPE_ENUM_ALL_OF",
            Self::AnyOf => "ATTRIBUTE_RULE_TYPE_ENUM_ANY_OF",
            Self::Hierarchy => "ATTRIBUTE_RULE_TYPE_ENUM_HIERARCHY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AttributeRuleTypeEnum {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ATTRIBUTE_RULE_TYPE_ENUM_UNSPECIFIED",
            "ATTRIBUTE_RULE_TYPE_ENUM_ALL_OF",
            "ATTRIBUTE_RULE_TYPE_ENUM_ANY_OF",
            "ATTRIBUTE_RULE_TYPE_ENUM_HIERARCHY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AttributeRuleTypeEnum;

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
                    "ATTRIBUTE_RULE_TYPE_ENUM_UNSPECIFIED" => Ok(AttributeRuleTypeEnum::Unspecified),
                    "ATTRIBUTE_RULE_TYPE_ENUM_ALL_OF" => Ok(AttributeRuleTypeEnum::AllOf),
                    "ATTRIBUTE_RULE_TYPE_ENUM_ANY_OF" => Ok(AttributeRuleTypeEnum::AnyOf),
                    "ATTRIBUTE_RULE_TYPE_ENUM_HIERARCHY" => Ok(AttributeRuleTypeEnum::Hierarchy),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AttributeValueSelector {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.with_key_access_grants {
            len += 1;
        }
        if self.with_subject_maps {
            len += 1;
        }
        if self.with_resource_maps {
            len += 1;
        }
        if self.with_attribute.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.AttributeValueSelector", len)?;
        if self.with_key_access_grants {
            struct_ser.serialize_field("withKeyAccessGrants", &self.with_key_access_grants)?;
        }
        if self.with_subject_maps {
            struct_ser.serialize_field("withSubjectMaps", &self.with_subject_maps)?;
        }
        if self.with_resource_maps {
            struct_ser.serialize_field("withResourceMaps", &self.with_resource_maps)?;
        }
        if let Some(v) = self.with_attribute.as_ref() {
            struct_ser.serialize_field("withAttribute", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AttributeValueSelector {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "with_key_access_grants",
            "withKeyAccessGrants",
            "with_subject_maps",
            "withSubjectMaps",
            "with_resource_maps",
            "withResourceMaps",
            "with_attribute",
            "withAttribute",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WithKeyAccessGrants,
            WithSubjectMaps,
            WithResourceMaps,
            WithAttribute,
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
                            "withKeyAccessGrants" | "with_key_access_grants" => Ok(GeneratedField::WithKeyAccessGrants),
                            "withSubjectMaps" | "with_subject_maps" => Ok(GeneratedField::WithSubjectMaps),
                            "withResourceMaps" | "with_resource_maps" => Ok(GeneratedField::WithResourceMaps),
                            "withAttribute" | "with_attribute" => Ok(GeneratedField::WithAttribute),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AttributeValueSelector;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.AttributeValueSelector")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AttributeValueSelector, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut with_key_access_grants__ = None;
                let mut with_subject_maps__ = None;
                let mut with_resource_maps__ = None;
                let mut with_attribute__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WithKeyAccessGrants => {
                            if with_key_access_grants__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withKeyAccessGrants"));
                            }
                            with_key_access_grants__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WithSubjectMaps => {
                            if with_subject_maps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withSubjectMaps"));
                            }
                            with_subject_maps__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WithResourceMaps => {
                            if with_resource_maps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withResourceMaps"));
                            }
                            with_resource_maps__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WithAttribute => {
                            if with_attribute__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withAttribute"));
                            }
                            with_attribute__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AttributeValueSelector {
                    with_key_access_grants: with_key_access_grants__.unwrap_or_default(),
                    with_subject_maps: with_subject_maps__.unwrap_or_default(),
                    with_resource_maps: with_resource_maps__.unwrap_or_default(),
                    with_attribute: with_attribute__,
                })
            }
        }
        deserializer.deserialize_struct("policy.AttributeValueSelector", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for attribute_value_selector::AttributeSelector {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.with_key_access_grants {
            len += 1;
        }
        if self.with_namespace.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.AttributeValueSelector.AttributeSelector", len)?;
        if self.with_key_access_grants {
            struct_ser.serialize_field("withKeyAccessGrants", &self.with_key_access_grants)?;
        }
        if let Some(v) = self.with_namespace.as_ref() {
            struct_ser.serialize_field("withNamespace", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for attribute_value_selector::AttributeSelector {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "with_key_access_grants",
            "withKeyAccessGrants",
            "with_namespace",
            "withNamespace",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WithKeyAccessGrants,
            WithNamespace,
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
                            "withKeyAccessGrants" | "with_key_access_grants" => Ok(GeneratedField::WithKeyAccessGrants),
                            "withNamespace" | "with_namespace" => Ok(GeneratedField::WithNamespace),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = attribute_value_selector::AttributeSelector;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.AttributeValueSelector.AttributeSelector")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<attribute_value_selector::AttributeSelector, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut with_key_access_grants__ = None;
                let mut with_namespace__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WithKeyAccessGrants => {
                            if with_key_access_grants__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withKeyAccessGrants"));
                            }
                            with_key_access_grants__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WithNamespace => {
                            if with_namespace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withNamespace"));
                            }
                            with_namespace__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(attribute_value_selector::AttributeSelector {
                    with_key_access_grants: with_key_access_grants__.unwrap_or_default(),
                    with_namespace: with_namespace__,
                })
            }
        }
        deserializer.deserialize_struct("policy.AttributeValueSelector.AttributeSelector", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for attribute_value_selector::attribute_selector::NamespaceSelector {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("policy.AttributeValueSelector.AttributeSelector.NamespaceSelector", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for attribute_value_selector::attribute_selector::NamespaceSelector {
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
            type Value = attribute_value_selector::attribute_selector::NamespaceSelector;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.AttributeValueSelector.AttributeSelector.NamespaceSelector")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<attribute_value_selector::attribute_selector::NamespaceSelector, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(attribute_value_selector::attribute_selector::NamespaceSelector {
                })
            }
        }
        deserializer.deserialize_struct("policy.AttributeValueSelector.AttributeSelector.NamespaceSelector", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Condition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.subject_external_selector_value.is_empty() {
            len += 1;
        }
        if self.operator != 0 {
            len += 1;
        }
        if !self.subject_external_values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.Condition", len)?;
        if !self.subject_external_selector_value.is_empty() {
            struct_ser.serialize_field("subjectExternalSelectorValue", &self.subject_external_selector_value)?;
        }
        if self.operator != 0 {
            let v = SubjectMappingOperatorEnum::try_from(self.operator)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.operator)))?;
            struct_ser.serialize_field("operator", &v)?;
        }
        if !self.subject_external_values.is_empty() {
            struct_ser.serialize_field("subjectExternalValues", &self.subject_external_values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Condition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subject_external_selector_value",
            "subjectExternalSelectorValue",
            "operator",
            "subject_external_values",
            "subjectExternalValues",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubjectExternalSelectorValue,
            Operator,
            SubjectExternalValues,
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
                            "subjectExternalSelectorValue" | "subject_external_selector_value" => Ok(GeneratedField::SubjectExternalSelectorValue),
                            "operator" => Ok(GeneratedField::Operator),
                            "subjectExternalValues" | "subject_external_values" => Ok(GeneratedField::SubjectExternalValues),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Condition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.Condition")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Condition, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut subject_external_selector_value__ = None;
                let mut operator__ = None;
                let mut subject_external_values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SubjectExternalSelectorValue => {
                            if subject_external_selector_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectExternalSelectorValue"));
                            }
                            subject_external_selector_value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Operator => {
                            if operator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operator"));
                            }
                            operator__ = Some(map_.next_value::<SubjectMappingOperatorEnum>()? as i32);
                        }
                        GeneratedField::SubjectExternalValues => {
                            if subject_external_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectExternalValues"));
                            }
                            subject_external_values__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Condition {
                    subject_external_selector_value: subject_external_selector_value__.unwrap_or_default(),
                    operator: operator__.unwrap_or_default(),
                    subject_external_values: subject_external_values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.Condition", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConditionBooleanTypeEnum {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "CONDITION_BOOLEAN_TYPE_ENUM_UNSPECIFIED",
            Self::And => "CONDITION_BOOLEAN_TYPE_ENUM_AND",
            Self::Or => "CONDITION_BOOLEAN_TYPE_ENUM_OR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ConditionBooleanTypeEnum {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CONDITION_BOOLEAN_TYPE_ENUM_UNSPECIFIED",
            "CONDITION_BOOLEAN_TYPE_ENUM_AND",
            "CONDITION_BOOLEAN_TYPE_ENUM_OR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConditionBooleanTypeEnum;

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
                    "CONDITION_BOOLEAN_TYPE_ENUM_UNSPECIFIED" => Ok(ConditionBooleanTypeEnum::Unspecified),
                    "CONDITION_BOOLEAN_TYPE_ENUM_AND" => Ok(ConditionBooleanTypeEnum::And),
                    "CONDITION_BOOLEAN_TYPE_ENUM_OR" => Ok(ConditionBooleanTypeEnum::Or),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ConditionGroup {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.conditions.is_empty() {
            len += 1;
        }
        if self.boolean_operator != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.ConditionGroup", len)?;
        if !self.conditions.is_empty() {
            struct_ser.serialize_field("conditions", &self.conditions)?;
        }
        if self.boolean_operator != 0 {
            let v = ConditionBooleanTypeEnum::try_from(self.boolean_operator)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.boolean_operator)))?;
            struct_ser.serialize_field("booleanOperator", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConditionGroup {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "conditions",
            "boolean_operator",
            "booleanOperator",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Conditions,
            BooleanOperator,
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
                            "conditions" => Ok(GeneratedField::Conditions),
                            "booleanOperator" | "boolean_operator" => Ok(GeneratedField::BooleanOperator),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConditionGroup;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.ConditionGroup")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ConditionGroup, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut conditions__ = None;
                let mut boolean_operator__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Conditions => {
                            if conditions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conditions"));
                            }
                            conditions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BooleanOperator => {
                            if boolean_operator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("booleanOperator"));
                            }
                            boolean_operator__ = Some(map_.next_value::<ConditionBooleanTypeEnum>()? as i32);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ConditionGroup {
                    conditions: conditions__.unwrap_or_default(),
                    boolean_operator: boolean_operator__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.ConditionGroup", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DynamicValueMapping {
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
        if self.attribute_definition.is_some() {
            len += 1;
        }
        if self.value_resolver.is_some() {
            len += 1;
        }
        if self.subject_condition_set.is_some() {
            len += 1;
        }
        if !self.actions.is_empty() {
            len += 1;
        }
        if self.namespace.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.DynamicValueMapping", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.attribute_definition.as_ref() {
            struct_ser.serialize_field("attributeDefinition", v)?;
        }
        if let Some(v) = self.value_resolver.as_ref() {
            struct_ser.serialize_field("valueResolver", v)?;
        }
        if let Some(v) = self.subject_condition_set.as_ref() {
            struct_ser.serialize_field("subjectConditionSet", v)?;
        }
        if !self.actions.is_empty() {
            struct_ser.serialize_field("actions", &self.actions)?;
        }
        if let Some(v) = self.namespace.as_ref() {
            struct_ser.serialize_field("namespace", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DynamicValueMapping {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "attribute_definition",
            "attributeDefinition",
            "value_resolver",
            "valueResolver",
            "subject_condition_set",
            "subjectConditionSet",
            "actions",
            "namespace",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            AttributeDefinition,
            ValueResolver,
            SubjectConditionSet,
            Actions,
            Namespace,
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
                            "id" => Ok(GeneratedField::Id),
                            "attributeDefinition" | "attribute_definition" => Ok(GeneratedField::AttributeDefinition),
                            "valueResolver" | "value_resolver" => Ok(GeneratedField::ValueResolver),
                            "subjectConditionSet" | "subject_condition_set" => Ok(GeneratedField::SubjectConditionSet),
                            "actions" => Ok(GeneratedField::Actions),
                            "namespace" => Ok(GeneratedField::Namespace),
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
            type Value = DynamicValueMapping;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.DynamicValueMapping")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DynamicValueMapping, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut attribute_definition__ = None;
                let mut value_resolver__ = None;
                let mut subject_condition_set__ = None;
                let mut actions__ = None;
                let mut namespace__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AttributeDefinition => {
                            if attribute_definition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeDefinition"));
                            }
                            attribute_definition__ = map_.next_value()?;
                        }
                        GeneratedField::ValueResolver => {
                            if value_resolver__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueResolver"));
                            }
                            value_resolver__ = map_.next_value()?;
                        }
                        GeneratedField::SubjectConditionSet => {
                            if subject_condition_set__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectConditionSet"));
                            }
                            subject_condition_set__ = map_.next_value()?;
                        }
                        GeneratedField::Actions => {
                            if actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            actions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Namespace => {
                            if namespace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespace"));
                            }
                            namespace__ = map_.next_value()?;
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
                Ok(DynamicValueMapping {
                    id: id__.unwrap_or_default(),
                    attribute_definition: attribute_definition__,
                    value_resolver: value_resolver__,
                    subject_condition_set: subject_condition_set__,
                    actions: actions__.unwrap_or_default(),
                    namespace: namespace__,
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.DynamicValueMapping", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DynamicValueResolver {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.subject_external_selector_value.is_empty() {
            len += 1;
        }
        if self.operator != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.DynamicValueResolver", len)?;
        if !self.subject_external_selector_value.is_empty() {
            struct_ser.serialize_field("subjectExternalSelectorValue", &self.subject_external_selector_value)?;
        }
        if self.operator != 0 {
            let v = SubjectMappingOperatorEnum::try_from(self.operator)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.operator)))?;
            struct_ser.serialize_field("operator", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DynamicValueResolver {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subject_external_selector_value",
            "subjectExternalSelectorValue",
            "operator",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubjectExternalSelectorValue,
            Operator,
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
                            "subjectExternalSelectorValue" | "subject_external_selector_value" => Ok(GeneratedField::SubjectExternalSelectorValue),
                            "operator" => Ok(GeneratedField::Operator),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DynamicValueResolver;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.DynamicValueResolver")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DynamicValueResolver, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut subject_external_selector_value__ = None;
                let mut operator__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SubjectExternalSelectorValue => {
                            if subject_external_selector_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectExternalSelectorValue"));
                            }
                            subject_external_selector_value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Operator => {
                            if operator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operator"));
                            }
                            operator__ = Some(map_.next_value::<SubjectMappingOperatorEnum>()? as i32);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DynamicValueResolver {
                    subject_external_selector_value: subject_external_selector_value__.unwrap_or_default(),
                    operator: operator__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.DynamicValueResolver", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KasKey {
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
        if !self.kas_uri.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.KasKey", len)?;
        if !self.kas_id.is_empty() {
            struct_ser.serialize_field("kasId", &self.kas_id)?;
        }
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        if !self.kas_uri.is_empty() {
            struct_ser.serialize_field("kasUri", &self.kas_uri)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for KasKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kas_id",
            "kasId",
            "key",
            "kas_uri",
            "kasUri",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KasId,
            Key,
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
                            "kasId" | "kas_id" => Ok(GeneratedField::KasId),
                            "key" => Ok(GeneratedField::Key),
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
            type Value = KasKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.KasKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<KasKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kas_id__ = None;
                let mut key__ = None;
                let mut kas_uri__ = None;
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
                        GeneratedField::KasUri => {
                            if kas_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasUri"));
                            }
                            kas_uri__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(KasKey {
                    kas_id: kas_id__.unwrap_or_default(),
                    key: key__,
                    kas_uri: kas_uri__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.KasKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KasPublicKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.pem.is_empty() {
            len += 1;
        }
        if !self.kid.is_empty() {
            len += 1;
        }
        if self.alg != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.KasPublicKey", len)?;
        if !self.pem.is_empty() {
            struct_ser.serialize_field("pem", &self.pem)?;
        }
        if !self.kid.is_empty() {
            struct_ser.serialize_field("kid", &self.kid)?;
        }
        if self.alg != 0 {
            let v = KasPublicKeyAlgEnum::try_from(self.alg)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.alg)))?;
            struct_ser.serialize_field("alg", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for KasPublicKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pem",
            "kid",
            "alg",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pem,
            Kid,
            Alg,
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
                            "pem" => Ok(GeneratedField::Pem),
                            "kid" => Ok(GeneratedField::Kid),
                            "alg" => Ok(GeneratedField::Alg),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = KasPublicKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.KasPublicKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<KasPublicKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pem__ = None;
                let mut kid__ = None;
                let mut alg__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pem => {
                            if pem__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pem"));
                            }
                            pem__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Kid => {
                            if kid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kid"));
                            }
                            kid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Alg => {
                            if alg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("alg"));
                            }
                            alg__ = Some(map_.next_value::<KasPublicKeyAlgEnum>()? as i32);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(KasPublicKey {
                    pem: pem__.unwrap_or_default(),
                    kid: kid__.unwrap_or_default(),
                    alg: alg__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.KasPublicKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KasPublicKeyAlgEnum {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "KAS_PUBLIC_KEY_ALG_ENUM_UNSPECIFIED",
            Self::Rsa2048 => "KAS_PUBLIC_KEY_ALG_ENUM_RSA_2048",
            Self::Rsa4096 => "KAS_PUBLIC_KEY_ALG_ENUM_RSA_4096",
            Self::EcSecp256r1 => "KAS_PUBLIC_KEY_ALG_ENUM_EC_SECP256R1",
            Self::EcSecp384r1 => "KAS_PUBLIC_KEY_ALG_ENUM_EC_SECP384R1",
            Self::EcSecp521r1 => "KAS_PUBLIC_KEY_ALG_ENUM_EC_SECP521R1",
            Self::HpqtXwing => "KAS_PUBLIC_KEY_ALG_ENUM_HPQT_XWING",
            Self::HpqtSecp256r1Mlkem768 => "KAS_PUBLIC_KEY_ALG_ENUM_HPQT_SECP256R1_MLKEM768",
            Self::HpqtSecp384r1Mlkem1024 => "KAS_PUBLIC_KEY_ALG_ENUM_HPQT_SECP384R1_MLKEM1024",
            Self::Mlkem768 => "KAS_PUBLIC_KEY_ALG_ENUM_MLKEM_768",
            Self::Mlkem1024 => "KAS_PUBLIC_KEY_ALG_ENUM_MLKEM_1024",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for KasPublicKeyAlgEnum {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "KAS_PUBLIC_KEY_ALG_ENUM_UNSPECIFIED",
            "KAS_PUBLIC_KEY_ALG_ENUM_RSA_2048",
            "KAS_PUBLIC_KEY_ALG_ENUM_RSA_4096",
            "KAS_PUBLIC_KEY_ALG_ENUM_EC_SECP256R1",
            "KAS_PUBLIC_KEY_ALG_ENUM_EC_SECP384R1",
            "KAS_PUBLIC_KEY_ALG_ENUM_EC_SECP521R1",
            "KAS_PUBLIC_KEY_ALG_ENUM_HPQT_XWING",
            "KAS_PUBLIC_KEY_ALG_ENUM_HPQT_SECP256R1_MLKEM768",
            "KAS_PUBLIC_KEY_ALG_ENUM_HPQT_SECP384R1_MLKEM1024",
            "KAS_PUBLIC_KEY_ALG_ENUM_MLKEM_768",
            "KAS_PUBLIC_KEY_ALG_ENUM_MLKEM_1024",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = KasPublicKeyAlgEnum;

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
                    "KAS_PUBLIC_KEY_ALG_ENUM_UNSPECIFIED" => Ok(KasPublicKeyAlgEnum::Unspecified),
                    "KAS_PUBLIC_KEY_ALG_ENUM_RSA_2048" => Ok(KasPublicKeyAlgEnum::Rsa2048),
                    "KAS_PUBLIC_KEY_ALG_ENUM_RSA_4096" => Ok(KasPublicKeyAlgEnum::Rsa4096),
                    "KAS_PUBLIC_KEY_ALG_ENUM_EC_SECP256R1" => Ok(KasPublicKeyAlgEnum::EcSecp256r1),
                    "KAS_PUBLIC_KEY_ALG_ENUM_EC_SECP384R1" => Ok(KasPublicKeyAlgEnum::EcSecp384r1),
                    "KAS_PUBLIC_KEY_ALG_ENUM_EC_SECP521R1" => Ok(KasPublicKeyAlgEnum::EcSecp521r1),
                    "KAS_PUBLIC_KEY_ALG_ENUM_HPQT_XWING" => Ok(KasPublicKeyAlgEnum::HpqtXwing),
                    "KAS_PUBLIC_KEY_ALG_ENUM_HPQT_SECP256R1_MLKEM768" => Ok(KasPublicKeyAlgEnum::HpqtSecp256r1Mlkem768),
                    "KAS_PUBLIC_KEY_ALG_ENUM_HPQT_SECP384R1_MLKEM1024" => Ok(KasPublicKeyAlgEnum::HpqtSecp384r1Mlkem1024),
                    "KAS_PUBLIC_KEY_ALG_ENUM_MLKEM_768" => Ok(KasPublicKeyAlgEnum::Mlkem768),
                    "KAS_PUBLIC_KEY_ALG_ENUM_MLKEM_1024" => Ok(KasPublicKeyAlgEnum::Mlkem1024),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for KasPublicKeySet {
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
        let mut struct_ser = serializer.serialize_struct("policy.KasPublicKeySet", len)?;
        if !self.keys.is_empty() {
            struct_ser.serialize_field("keys", &self.keys)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for KasPublicKeySet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "keys",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = KasPublicKeySet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.KasPublicKeySet")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<KasPublicKeySet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut keys__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                Ok(KasPublicKeySet {
                    keys: keys__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.KasPublicKeySet", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Key {
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
        if self.is_active.is_some() {
            len += 1;
        }
        if self.was_mapped.is_some() {
            len += 1;
        }
        if self.public_key.is_some() {
            len += 1;
        }
        if self.kas.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.Key", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.is_active.as_ref() {
            struct_ser.serialize_field("isActive", v)?;
        }
        if let Some(v) = self.was_mapped.as_ref() {
            struct_ser.serialize_field("wasMapped", v)?;
        }
        if let Some(v) = self.public_key.as_ref() {
            struct_ser.serialize_field("publicKey", v)?;
        }
        if let Some(v) = self.kas.as_ref() {
            struct_ser.serialize_field("kas", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Key {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "is_active",
            "isActive",
            "was_mapped",
            "wasMapped",
            "public_key",
            "publicKey",
            "kas",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            IsActive,
            WasMapped,
            PublicKey,
            Kas,
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
                            "id" => Ok(GeneratedField::Id),
                            "isActive" | "is_active" => Ok(GeneratedField::IsActive),
                            "wasMapped" | "was_mapped" => Ok(GeneratedField::WasMapped),
                            "publicKey" | "public_key" => Ok(GeneratedField::PublicKey),
                            "kas" => Ok(GeneratedField::Kas),
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
            type Value = Key;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.Key")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Key, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut is_active__ = None;
                let mut was_mapped__ = None;
                let mut public_key__ = None;
                let mut kas__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsActive => {
                            if is_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isActive"));
                            }
                            is_active__ = map_.next_value()?;
                        }
                        GeneratedField::WasMapped => {
                            if was_mapped__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wasMapped"));
                            }
                            was_mapped__ = map_.next_value()?;
                        }
                        GeneratedField::PublicKey => {
                            if public_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicKey"));
                            }
                            public_key__ = map_.next_value()?;
                        }
                        GeneratedField::Kas => {
                            if kas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kas"));
                            }
                            kas__ = map_.next_value()?;
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
                Ok(Key {
                    id: id__.unwrap_or_default(),
                    is_active: is_active__,
                    was_mapped: was_mapped__,
                    public_key: public_key__,
                    kas: kas__,
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.Key", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KeyAccessServer {
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
        if !self.kas_keys.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.KeyAccessServer", len)?;
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
            let v = SourceType::try_from(self.source_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.source_type)))?;
            struct_ser.serialize_field("sourceType", &v)?;
        }
        if !self.kas_keys.is_empty() {
            struct_ser.serialize_field("kasKeys", &self.kas_keys)?;
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
impl<'de> serde::Deserialize<'de> for KeyAccessServer {
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
            "kas_keys",
            "kasKeys",
            "name",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Uri,
            PublicKey,
            SourceType,
            KasKeys,
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
                            "id" => Ok(GeneratedField::Id),
                            "uri" => Ok(GeneratedField::Uri),
                            "publicKey" | "public_key" => Ok(GeneratedField::PublicKey),
                            "sourceType" | "source_type" => Ok(GeneratedField::SourceType),
                            "kasKeys" | "kas_keys" => Ok(GeneratedField::KasKeys),
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
            type Value = KeyAccessServer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.KeyAccessServer")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<KeyAccessServer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut uri__ = None;
                let mut public_key__ = None;
                let mut source_type__ = None;
                let mut kas_keys__ = None;
                let mut name__ = None;
                let mut metadata__ = None;
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
                            source_type__ = Some(map_.next_value::<SourceType>()? as i32);
                        }
                        GeneratedField::KasKeys => {
                            if kas_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasKeys"));
                            }
                            kas_keys__ = Some(map_.next_value()?);
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
                Ok(KeyAccessServer {
                    id: id__.unwrap_or_default(),
                    uri: uri__.unwrap_or_default(),
                    public_key: public_key__,
                    source_type: source_type__.unwrap_or_default(),
                    kas_keys: kas_keys__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.KeyAccessServer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KeyMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "KEY_MODE_UNSPECIFIED",
            Self::ConfigRootKey => "KEY_MODE_CONFIG_ROOT_KEY",
            Self::ProviderRootKey => "KEY_MODE_PROVIDER_ROOT_KEY",
            Self::Remote => "KEY_MODE_REMOTE",
            Self::PublicKeyOnly => "KEY_MODE_PUBLIC_KEY_ONLY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for KeyMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "KEY_MODE_UNSPECIFIED",
            "KEY_MODE_CONFIG_ROOT_KEY",
            "KEY_MODE_PROVIDER_ROOT_KEY",
            "KEY_MODE_REMOTE",
            "KEY_MODE_PUBLIC_KEY_ONLY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = KeyMode;

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
                    "KEY_MODE_UNSPECIFIED" => Ok(KeyMode::Unspecified),
                    "KEY_MODE_CONFIG_ROOT_KEY" => Ok(KeyMode::ConfigRootKey),
                    "KEY_MODE_PROVIDER_ROOT_KEY" => Ok(KeyMode::ProviderRootKey),
                    "KEY_MODE_REMOTE" => Ok(KeyMode::Remote),
                    "KEY_MODE_PUBLIC_KEY_ONLY" => Ok(KeyMode::PublicKeyOnly),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for KeyProviderConfig {
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
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.config_json.is_empty() {
            len += 1;
        }
        if !self.manager.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.KeyProviderConfig", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.config_json.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("configJson", pbjson::private::base64::encode(&self.config_json).as_str())?;
        }
        if !self.manager.is_empty() {
            struct_ser.serialize_field("manager", &self.manager)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for KeyProviderConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "config_json",
            "configJson",
            "manager",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            ConfigJson,
            Manager,
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
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "configJson" | "config_json" => Ok(GeneratedField::ConfigJson),
                            "manager" => Ok(GeneratedField::Manager),
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
            type Value = KeyProviderConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.KeyProviderConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<KeyProviderConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut config_json__ = None;
                let mut manager__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConfigJson => {
                            if config_json__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configJson"));
                            }
                            config_json__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Manager => {
                            if manager__.is_some() {
                                return Err(serde::de::Error::duplicate_field("manager"));
                            }
                            manager__ = Some(map_.next_value()?);
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
                Ok(KeyProviderConfig {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    config_json: config_json__.unwrap_or_default(),
                    manager: manager__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.KeyProviderConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KeyStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "KEY_STATUS_UNSPECIFIED",
            Self::Active => "KEY_STATUS_ACTIVE",
            Self::Rotated => "KEY_STATUS_ROTATED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for KeyStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "KEY_STATUS_UNSPECIFIED",
            "KEY_STATUS_ACTIVE",
            "KEY_STATUS_ROTATED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = KeyStatus;

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
                    "KEY_STATUS_UNSPECIFIED" => Ok(KeyStatus::Unspecified),
                    "KEY_STATUS_ACTIVE" => Ok(KeyStatus::Active),
                    "KEY_STATUS_ROTATED" => Ok(KeyStatus::Rotated),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Namespace {
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
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.fqn.is_empty() {
            len += 1;
        }
        if self.active.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        if !self.grants.is_empty() {
            len += 1;
        }
        if !self.kas_keys.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.Namespace", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.fqn.is_empty() {
            struct_ser.serialize_field("fqn", &self.fqn)?;
        }
        if let Some(v) = self.active.as_ref() {
            struct_ser.serialize_field("active", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.grants.is_empty() {
            struct_ser.serialize_field("grants", &self.grants)?;
        }
        if !self.kas_keys.is_empty() {
            struct_ser.serialize_field("kasKeys", &self.kas_keys)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Namespace {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "fqn",
            "active",
            "metadata",
            "grants",
            "kas_keys",
            "kasKeys",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Fqn,
            Active,
            Metadata,
            Grants,
            KasKeys,
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
                            "name" => Ok(GeneratedField::Name),
                            "fqn" => Ok(GeneratedField::Fqn),
                            "active" => Ok(GeneratedField::Active),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "grants" => Ok(GeneratedField::Grants),
                            "kasKeys" | "kas_keys" => Ok(GeneratedField::KasKeys),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Namespace;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.Namespace")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Namespace, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut fqn__ = None;
                let mut active__ = None;
                let mut metadata__ = None;
                let mut grants__ = None;
                let mut kas_keys__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fqn => {
                            if fqn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fqn"));
                            }
                            fqn__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Active => {
                            if active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("active"));
                            }
                            active__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::Grants => {
                            if grants__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grants"));
                            }
                            grants__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KasKeys => {
                            if kas_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasKeys"));
                            }
                            kas_keys__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Namespace {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    fqn: fqn__.unwrap_or_default(),
                    active: active__,
                    metadata: metadata__,
                    grants: grants__.unwrap_or_default(),
                    kas_keys: kas_keys__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.Namespace", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Obligation {
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
        if self.namespace.is_some() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.values.is_empty() {
            len += 1;
        }
        if !self.fqn.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.Obligation", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.namespace.as_ref() {
            struct_ser.serialize_field("namespace", v)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        if !self.fqn.is_empty() {
            struct_ser.serialize_field("fqn", &self.fqn)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Obligation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "namespace",
            "name",
            "values",
            "fqn",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Namespace,
            Name,
            Values,
            Fqn,
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
                            "id" => Ok(GeneratedField::Id),
                            "namespace" => Ok(GeneratedField::Namespace),
                            "name" => Ok(GeneratedField::Name),
                            "values" => Ok(GeneratedField::Values),
                            "fqn" => Ok(GeneratedField::Fqn),
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
            type Value = Obligation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.Obligation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Obligation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut namespace__ = None;
                let mut name__ = None;
                let mut values__ = None;
                let mut fqn__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Namespace => {
                            if namespace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespace"));
                            }
                            namespace__ = map_.next_value()?;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fqn => {
                            if fqn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fqn"));
                            }
                            fqn__ = Some(map_.next_value()?);
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
                Ok(Obligation {
                    id: id__.unwrap_or_default(),
                    namespace: namespace__,
                    name: name__.unwrap_or_default(),
                    values: values__.unwrap_or_default(),
                    fqn: fqn__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.Obligation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ObligationTrigger {
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
        if self.obligation_value.is_some() {
            len += 1;
        }
        if self.action.is_some() {
            len += 1;
        }
        if self.attribute_value.is_some() {
            len += 1;
        }
        if !self.context.is_empty() {
            len += 1;
        }
        if self.namespace.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.ObligationTrigger", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.obligation_value.as_ref() {
            struct_ser.serialize_field("obligationValue", v)?;
        }
        if let Some(v) = self.action.as_ref() {
            struct_ser.serialize_field("action", v)?;
        }
        if let Some(v) = self.attribute_value.as_ref() {
            struct_ser.serialize_field("attributeValue", v)?;
        }
        if !self.context.is_empty() {
            struct_ser.serialize_field("context", &self.context)?;
        }
        if let Some(v) = self.namespace.as_ref() {
            struct_ser.serialize_field("namespace", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ObligationTrigger {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "obligation_value",
            "obligationValue",
            "action",
            "attribute_value",
            "attributeValue",
            "context",
            "namespace",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ObligationValue,
            Action,
            AttributeValue,
            Context,
            Namespace,
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
                            "id" => Ok(GeneratedField::Id),
                            "obligationValue" | "obligation_value" => Ok(GeneratedField::ObligationValue),
                            "action" => Ok(GeneratedField::Action),
                            "attributeValue" | "attribute_value" => Ok(GeneratedField::AttributeValue),
                            "context" => Ok(GeneratedField::Context),
                            "namespace" => Ok(GeneratedField::Namespace),
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
            type Value = ObligationTrigger;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.ObligationTrigger")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ObligationTrigger, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut obligation_value__ = None;
                let mut action__ = None;
                let mut attribute_value__ = None;
                let mut context__ = None;
                let mut namespace__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
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
                        GeneratedField::AttributeValue => {
                            if attribute_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeValue"));
                            }
                            attribute_value__ = map_.next_value()?;
                        }
                        GeneratedField::Context => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            context__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Namespace => {
                            if namespace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespace"));
                            }
                            namespace__ = map_.next_value()?;
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
                Ok(ObligationTrigger {
                    id: id__.unwrap_or_default(),
                    obligation_value: obligation_value__,
                    action: action__,
                    attribute_value: attribute_value__,
                    context: context__.unwrap_or_default(),
                    namespace: namespace__,
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.ObligationTrigger", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ObligationValue {
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
        if self.obligation.is_some() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        if !self.triggers.is_empty() {
            len += 1;
        }
        if !self.fqn.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.ObligationValue", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.obligation.as_ref() {
            struct_ser.serialize_field("obligation", v)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        if !self.triggers.is_empty() {
            struct_ser.serialize_field("triggers", &self.triggers)?;
        }
        if !self.fqn.is_empty() {
            struct_ser.serialize_field("fqn", &self.fqn)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ObligationValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "obligation",
            "value",
            "triggers",
            "fqn",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Obligation,
            Value,
            Triggers,
            Fqn,
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
                            "id" => Ok(GeneratedField::Id),
                            "obligation" => Ok(GeneratedField::Obligation),
                            "value" => Ok(GeneratedField::Value),
                            "triggers" => Ok(GeneratedField::Triggers),
                            "fqn" => Ok(GeneratedField::Fqn),
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
            type Value = ObligationValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.ObligationValue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ObligationValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut obligation__ = None;
                let mut value__ = None;
                let mut triggers__ = None;
                let mut fqn__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Obligation => {
                            if obligation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("obligation"));
                            }
                            obligation__ = map_.next_value()?;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Triggers => {
                            if triggers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("triggers"));
                            }
                            triggers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fqn => {
                            if fqn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fqn"));
                            }
                            fqn__ = Some(map_.next_value()?);
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
                Ok(ObligationValue {
                    id: id__.unwrap_or_default(),
                    obligation: obligation__,
                    value: value__.unwrap_or_default(),
                    triggers: triggers__.unwrap_or_default(),
                    fqn: fqn__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.ObligationValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PageRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.limit != 0 {
            len += 1;
        }
        if self.offset != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.PageRequest", len)?;
        if self.limit != 0 {
            struct_ser.serialize_field("limit", &self.limit)?;
        }
        if self.offset != 0 {
            struct_ser.serialize_field("offset", &self.offset)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PageRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "limit",
            "offset",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Limit,
            Offset,
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
                            "limit" => Ok(GeneratedField::Limit),
                            "offset" => Ok(GeneratedField::Offset),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PageRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.PageRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PageRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut limit__ = None;
                let mut offset__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Offset => {
                            if offset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offset"));
                            }
                            offset__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(PageRequest {
                    limit: limit__.unwrap_or_default(),
                    offset: offset__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.PageRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.current_offset != 0 {
            len += 1;
        }
        if self.next_offset != 0 {
            len += 1;
        }
        if self.total != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.PageResponse", len)?;
        if self.current_offset != 0 {
            struct_ser.serialize_field("currentOffset", &self.current_offset)?;
        }
        if self.next_offset != 0 {
            struct_ser.serialize_field("nextOffset", &self.next_offset)?;
        }
        if self.total != 0 {
            struct_ser.serialize_field("total", &self.total)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PageResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "current_offset",
            "currentOffset",
            "next_offset",
            "nextOffset",
            "total",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CurrentOffset,
            NextOffset,
            Total,
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
                            "currentOffset" | "current_offset" => Ok(GeneratedField::CurrentOffset),
                            "nextOffset" | "next_offset" => Ok(GeneratedField::NextOffset),
                            "total" => Ok(GeneratedField::Total),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.PageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut current_offset__ = None;
                let mut next_offset__ = None;
                let mut total__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CurrentOffset => {
                            if current_offset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentOffset"));
                            }
                            current_offset__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NextOffset => {
                            if next_offset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextOffset"));
                            }
                            next_offset__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Total => {
                            if total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            total__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(PageResponse {
                    current_offset: current_offset__.unwrap_or_default(),
                    next_offset: next_offset__.unwrap_or_default(),
                    total: total__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.PageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PolicyEnforcementPoint {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.client_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.PolicyEnforcementPoint", len)?;
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PolicyEnforcementPoint {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "client_id",
            "clientId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientId,
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
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PolicyEnforcementPoint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.PolicyEnforcementPoint")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PolicyEnforcementPoint, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(PolicyEnforcementPoint {
                    client_id: client_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.PolicyEnforcementPoint", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PrivateKeyCtx {
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
        if !self.wrapped_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.PrivateKeyCtx", len)?;
        if !self.key_id.is_empty() {
            struct_ser.serialize_field("keyId", &self.key_id)?;
        }
        if !self.wrapped_key.is_empty() {
            struct_ser.serialize_field("wrappedKey", &self.wrapped_key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PrivateKeyCtx {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key_id",
            "keyId",
            "wrapped_key",
            "wrappedKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KeyId,
            WrappedKey,
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
                            "wrappedKey" | "wrapped_key" => Ok(GeneratedField::WrappedKey),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PrivateKeyCtx;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.PrivateKeyCtx")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PrivateKeyCtx, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key_id__ = None;
                let mut wrapped_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KeyId => {
                            if key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyId"));
                            }
                            key_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WrappedKey => {
                            if wrapped_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wrappedKey"));
                            }
                            wrapped_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(PrivateKeyCtx {
                    key_id: key_id__.unwrap_or_default(),
                    wrapped_key: wrapped_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.PrivateKeyCtx", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PublicKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.public_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.PublicKey", len)?;
        if let Some(v) = self.public_key.as_ref() {
            match v {
                public_key::PublicKey::Remote(v) => {
                    struct_ser.serialize_field("remote", v)?;
                }
                public_key::PublicKey::Cached(v) => {
                    struct_ser.serialize_field("cached", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PublicKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "remote",
            "cached",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Remote,
            Cached,
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
                            "remote" => Ok(GeneratedField::Remote),
                            "cached" => Ok(GeneratedField::Cached),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PublicKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.PublicKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PublicKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut public_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Remote => {
                            if public_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remote"));
                            }
                            public_key__ = map_.next_value::<::std::option::Option<_>>()?.map(public_key::PublicKey::Remote);
                        }
                        GeneratedField::Cached => {
                            if public_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cached"));
                            }
                            public_key__ = map_.next_value::<::std::option::Option<_>>()?.map(public_key::PublicKey::Cached)
;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(PublicKey {
                    public_key: public_key__,
                })
            }
        }
        deserializer.deserialize_struct("policy.PublicKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PublicKeyCtx {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.pem.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.PublicKeyCtx", len)?;
        if !self.pem.is_empty() {
            struct_ser.serialize_field("pem", &self.pem)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PublicKeyCtx {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pem",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pem,
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
                            "pem" => Ok(GeneratedField::Pem),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PublicKeyCtx;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.PublicKeyCtx")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PublicKeyCtx, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pem__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pem => {
                            if pem__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pem"));
                            }
                            pem__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(PublicKeyCtx {
                    pem: pem__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.PublicKeyCtx", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RegisteredResource {
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
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.values.is_empty() {
            len += 1;
        }
        if self.namespace.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.RegisteredResource", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        if let Some(v) = self.namespace.as_ref() {
            struct_ser.serialize_field("namespace", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegisteredResource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "values",
            "namespace",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Values,
            Namespace,
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
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "values" => Ok(GeneratedField::Values),
                            "namespace" => Ok(GeneratedField::Namespace),
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
            type Value = RegisteredResource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.RegisteredResource")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RegisteredResource, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut values__ = None;
                let mut namespace__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
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
                Ok(RegisteredResource {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    values: values__.unwrap_or_default(),
                    namespace: namespace__,
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.RegisteredResource", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RegisteredResourceValue {
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
        if !self.value.is_empty() {
            len += 1;
        }
        if self.resource.is_some() {
            len += 1;
        }
        if !self.action_attribute_values.is_empty() {
            len += 1;
        }
        if !self.fqn.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.RegisteredResourceValue", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
        }
        if !self.action_attribute_values.is_empty() {
            struct_ser.serialize_field("actionAttributeValues", &self.action_attribute_values)?;
        }
        if !self.fqn.is_empty() {
            struct_ser.serialize_field("fqn", &self.fqn)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegisteredResourceValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "value",
            "resource",
            "action_attribute_values",
            "actionAttributeValues",
            "fqn",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Value,
            Resource,
            ActionAttributeValues,
            Fqn,
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
                            "id" => Ok(GeneratedField::Id),
                            "value" => Ok(GeneratedField::Value),
                            "resource" => Ok(GeneratedField::Resource),
                            "actionAttributeValues" | "action_attribute_values" => Ok(GeneratedField::ActionAttributeValues),
                            "fqn" => Ok(GeneratedField::Fqn),
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
            type Value = RegisteredResourceValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.RegisteredResourceValue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RegisteredResourceValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut value__ = None;
                let mut resource__ = None;
                let mut action_attribute_values__ = None;
                let mut fqn__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Resource => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            resource__ = map_.next_value()?;
                        }
                        GeneratedField::ActionAttributeValues => {
                            if action_attribute_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionAttributeValues"));
                            }
                            action_attribute_values__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fqn => {
                            if fqn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fqn"));
                            }
                            fqn__ = Some(map_.next_value()?);
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
                Ok(RegisteredResourceValue {
                    id: id__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                    resource: resource__,
                    action_attribute_values: action_attribute_values__.unwrap_or_default(),
                    fqn: fqn__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.RegisteredResourceValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for registered_resource_value::ActionAttributeValue {
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
        if self.action.is_some() {
            len += 1;
        }
        if self.attribute_value.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.RegisteredResourceValue.ActionAttributeValue", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.action.as_ref() {
            struct_ser.serialize_field("action", v)?;
        }
        if let Some(v) = self.attribute_value.as_ref() {
            struct_ser.serialize_field("attributeValue", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for registered_resource_value::ActionAttributeValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "action",
            "attribute_value",
            "attributeValue",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Action,
            AttributeValue,
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
                            "id" => Ok(GeneratedField::Id),
                            "action" => Ok(GeneratedField::Action),
                            "attributeValue" | "attribute_value" => Ok(GeneratedField::AttributeValue),
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
            type Value = registered_resource_value::ActionAttributeValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.RegisteredResourceValue.ActionAttributeValue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<registered_resource_value::ActionAttributeValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut action__ = None;
                let mut attribute_value__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Action => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            action__ = map_.next_value()?;
                        }
                        GeneratedField::AttributeValue => {
                            if attribute_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeValue"));
                            }
                            attribute_value__ = map_.next_value()?;
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
                Ok(registered_resource_value::ActionAttributeValue {
                    id: id__.unwrap_or_default(),
                    action: action__,
                    attribute_value: attribute_value__,
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.RegisteredResourceValue.ActionAttributeValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RequestContext {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pep.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.RequestContext", len)?;
        if let Some(v) = self.pep.as_ref() {
            struct_ser.serialize_field("pep", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RequestContext {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pep",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pep,
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
                            "pep" => Ok(GeneratedField::Pep),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RequestContext;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.RequestContext")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RequestContext, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pep__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pep => {
                            if pep__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pep"));
                            }
                            pep__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(RequestContext {
                    pep: pep__,
                })
            }
        }
        deserializer.deserialize_struct("policy.RequestContext", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceMapping {
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
        if self.attribute_value.is_some() {
            len += 1;
        }
        if !self.terms.is_empty() {
            len += 1;
        }
        if self.group.is_some() {
            len += 1;
        }
        if self.namespace.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.ResourceMapping", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.attribute_value.as_ref() {
            struct_ser.serialize_field("attributeValue", v)?;
        }
        if !self.terms.is_empty() {
            struct_ser.serialize_field("terms", &self.terms)?;
        }
        if let Some(v) = self.group.as_ref() {
            struct_ser.serialize_field("group", v)?;
        }
        if let Some(v) = self.namespace.as_ref() {
            struct_ser.serialize_field("namespace", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceMapping {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "metadata",
            "attribute_value",
            "attributeValue",
            "terms",
            "group",
            "namespace",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Metadata,
            AttributeValue,
            Terms,
            Group,
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
                            "id" => Ok(GeneratedField::Id),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "attributeValue" | "attribute_value" => Ok(GeneratedField::AttributeValue),
                            "terms" => Ok(GeneratedField::Terms),
                            "group" => Ok(GeneratedField::Group),
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
            type Value = ResourceMapping;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.ResourceMapping")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceMapping, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut metadata__ = None;
                let mut attribute_value__ = None;
                let mut terms__ = None;
                let mut group__ = None;
                let mut namespace__ = None;
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
                        GeneratedField::AttributeValue => {
                            if attribute_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeValue"));
                            }
                            attribute_value__ = map_.next_value()?;
                        }
                        GeneratedField::Terms => {
                            if terms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("terms"));
                            }
                            terms__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Group => {
                            if group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            group__ = map_.next_value()?;
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
                Ok(ResourceMapping {
                    id: id__.unwrap_or_default(),
                    metadata: metadata__,
                    attribute_value: attribute_value__,
                    terms: terms__.unwrap_or_default(),
                    group: group__,
                    namespace: namespace__,
                })
            }
        }
        deserializer.deserialize_struct("policy.ResourceMapping", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceMappingGroup {
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
        if !self.namespace_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.fqn.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.ResourceMappingGroup", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.namespace_id.is_empty() {
            struct_ser.serialize_field("namespaceId", &self.namespace_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.fqn.is_empty() {
            struct_ser.serialize_field("fqn", &self.fqn)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceMappingGroup {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "namespace_id",
            "namespaceId",
            "name",
            "fqn",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            NamespaceId,
            Name,
            Fqn,
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
                            "id" => Ok(GeneratedField::Id),
                            "namespaceId" | "namespace_id" => Ok(GeneratedField::NamespaceId),
                            "name" => Ok(GeneratedField::Name),
                            "fqn" => Ok(GeneratedField::Fqn),
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
            type Value = ResourceMappingGroup;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.ResourceMappingGroup")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceMappingGroup, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut namespace_id__ = None;
                let mut name__ = None;
                let mut fqn__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
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
                        GeneratedField::Fqn => {
                            if fqn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fqn"));
                            }
                            fqn__ = Some(map_.next_value()?);
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
                Ok(ResourceMappingGroup {
                    id: id__.unwrap_or_default(),
                    namespace_id: namespace_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    fqn: fqn__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.ResourceMappingGroup", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Search {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.term.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.Search", len)?;
        if !self.term.is_empty() {
            struct_ser.serialize_field("term", &self.term)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Search {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "term",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Term,
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
                            "term" => Ok(GeneratedField::Term),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Search;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.Search")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Search, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut term__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Term => {
                            if term__.is_some() {
                                return Err(serde::de::Error::duplicate_field("term"));
                            }
                            term__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Search {
                    term: term__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.Search", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SimpleKasKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.kas_uri.is_empty() {
            len += 1;
        }
        if self.public_key.is_some() {
            len += 1;
        }
        if !self.kas_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.SimpleKasKey", len)?;
        if !self.kas_uri.is_empty() {
            struct_ser.serialize_field("kasUri", &self.kas_uri)?;
        }
        if let Some(v) = self.public_key.as_ref() {
            struct_ser.serialize_field("publicKey", v)?;
        }
        if !self.kas_id.is_empty() {
            struct_ser.serialize_field("kasId", &self.kas_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SimpleKasKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kas_uri",
            "kasUri",
            "public_key",
            "publicKey",
            "kas_id",
            "kasId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KasUri,
            PublicKey,
            KasId,
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
                            "kasUri" | "kas_uri" => Ok(GeneratedField::KasUri),
                            "publicKey" | "public_key" => Ok(GeneratedField::PublicKey),
                            "kasId" | "kas_id" => Ok(GeneratedField::KasId),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SimpleKasKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.SimpleKasKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SimpleKasKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kas_uri__ = None;
                let mut public_key__ = None;
                let mut kas_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KasUri => {
                            if kas_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasUri"));
                            }
                            kas_uri__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PublicKey => {
                            if public_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicKey"));
                            }
                            public_key__ = map_.next_value()?;
                        }
                        GeneratedField::KasId => {
                            if kas_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasId"));
                            }
                            kas_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(SimpleKasKey {
                    kas_uri: kas_uri__.unwrap_or_default(),
                    public_key: public_key__,
                    kas_id: kas_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.SimpleKasKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SimpleKasPublicKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.algorithm != 0 {
            len += 1;
        }
        if !self.kid.is_empty() {
            len += 1;
        }
        if !self.pem.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.SimpleKasPublicKey", len)?;
        if self.algorithm != 0 {
            let v = Algorithm::try_from(self.algorithm)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.algorithm)))?;
            struct_ser.serialize_field("algorithm", &v)?;
        }
        if !self.kid.is_empty() {
            struct_ser.serialize_field("kid", &self.kid)?;
        }
        if !self.pem.is_empty() {
            struct_ser.serialize_field("pem", &self.pem)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SimpleKasPublicKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "algorithm",
            "kid",
            "pem",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Algorithm,
            Kid,
            Pem,
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
                            "algorithm" => Ok(GeneratedField::Algorithm),
                            "kid" => Ok(GeneratedField::Kid),
                            "pem" => Ok(GeneratedField::Pem),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SimpleKasPublicKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.SimpleKasPublicKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SimpleKasPublicKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut algorithm__ = None;
                let mut kid__ = None;
                let mut pem__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Algorithm => {
                            if algorithm__.is_some() {
                                return Err(serde::de::Error::duplicate_field("algorithm"));
                            }
                            algorithm__ = Some(map_.next_value::<Algorithm>()? as i32);
                        }
                        GeneratedField::Kid => {
                            if kid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kid"));
                            }
                            kid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pem => {
                            if pem__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pem"));
                            }
                            pem__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(SimpleKasPublicKey {
                    algorithm: algorithm__.unwrap_or_default(),
                    kid: kid__.unwrap_or_default(),
                    pem: pem__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.SimpleKasPublicKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SortDirection {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SORT_DIRECTION_UNSPECIFIED",
            Self::Asc => "SORT_DIRECTION_ASC",
            Self::Desc => "SORT_DIRECTION_DESC",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SortDirection {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SORT_DIRECTION_UNSPECIFIED",
            "SORT_DIRECTION_ASC",
            "SORT_DIRECTION_DESC",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SortDirection;

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
                    "SORT_DIRECTION_UNSPECIFIED" => Ok(SortDirection::Unspecified),
                    "SORT_DIRECTION_ASC" => Ok(SortDirection::Asc),
                    "SORT_DIRECTION_DESC" => Ok(SortDirection::Desc),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SourceType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SOURCE_TYPE_UNSPECIFIED",
            Self::Internal => "SOURCE_TYPE_INTERNAL",
            Self::External => "SOURCE_TYPE_EXTERNAL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SourceType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SOURCE_TYPE_UNSPECIFIED",
            "SOURCE_TYPE_INTERNAL",
            "SOURCE_TYPE_EXTERNAL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SourceType;

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
                    "SOURCE_TYPE_UNSPECIFIED" => Ok(SourceType::Unspecified),
                    "SOURCE_TYPE_INTERNAL" => Ok(SourceType::Internal),
                    "SOURCE_TYPE_EXTERNAL" => Ok(SourceType::External),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SubjectConditionSet {
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
        if self.namespace.is_some() {
            len += 1;
        }
        if !self.subject_sets.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.SubjectConditionSet", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.namespace.as_ref() {
            struct_ser.serialize_field("namespace", v)?;
        }
        if !self.subject_sets.is_empty() {
            struct_ser.serialize_field("subjectSets", &self.subject_sets)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubjectConditionSet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "namespace",
            "subject_sets",
            "subjectSets",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Namespace,
            SubjectSets,
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
                            "id" => Ok(GeneratedField::Id),
                            "namespace" => Ok(GeneratedField::Namespace),
                            "subjectSets" | "subject_sets" => Ok(GeneratedField::SubjectSets),
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
            type Value = SubjectConditionSet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.SubjectConditionSet")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SubjectConditionSet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut namespace__ = None;
                let mut subject_sets__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Namespace => {
                            if namespace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespace"));
                            }
                            namespace__ = map_.next_value()?;
                        }
                        GeneratedField::SubjectSets => {
                            if subject_sets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectSets"));
                            }
                            subject_sets__ = Some(map_.next_value()?);
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
                Ok(SubjectConditionSet {
                    id: id__.unwrap_or_default(),
                    namespace: namespace__,
                    subject_sets: subject_sets__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.SubjectConditionSet", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SubjectMapping {
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
        if self.attribute_value.is_some() {
            len += 1;
        }
        if self.subject_condition_set.is_some() {
            len += 1;
        }
        if !self.actions.is_empty() {
            len += 1;
        }
        if self.namespace.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.SubjectMapping", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.attribute_value.as_ref() {
            struct_ser.serialize_field("attributeValue", v)?;
        }
        if let Some(v) = self.subject_condition_set.as_ref() {
            struct_ser.serialize_field("subjectConditionSet", v)?;
        }
        if !self.actions.is_empty() {
            struct_ser.serialize_field("actions", &self.actions)?;
        }
        if let Some(v) = self.namespace.as_ref() {
            struct_ser.serialize_field("namespace", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubjectMapping {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "attribute_value",
            "attributeValue",
            "subject_condition_set",
            "subjectConditionSet",
            "actions",
            "namespace",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            AttributeValue,
            SubjectConditionSet,
            Actions,
            Namespace,
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
                            "id" => Ok(GeneratedField::Id),
                            "attributeValue" | "attribute_value" => Ok(GeneratedField::AttributeValue),
                            "subjectConditionSet" | "subject_condition_set" => Ok(GeneratedField::SubjectConditionSet),
                            "actions" => Ok(GeneratedField::Actions),
                            "namespace" => Ok(GeneratedField::Namespace),
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
            type Value = SubjectMapping;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.SubjectMapping")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SubjectMapping, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut attribute_value__ = None;
                let mut subject_condition_set__ = None;
                let mut actions__ = None;
                let mut namespace__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AttributeValue => {
                            if attribute_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeValue"));
                            }
                            attribute_value__ = map_.next_value()?;
                        }
                        GeneratedField::SubjectConditionSet => {
                            if subject_condition_set__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectConditionSet"));
                            }
                            subject_condition_set__ = map_.next_value()?;
                        }
                        GeneratedField::Actions => {
                            if actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            actions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Namespace => {
                            if namespace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespace"));
                            }
                            namespace__ = map_.next_value()?;
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
                Ok(SubjectMapping {
                    id: id__.unwrap_or_default(),
                    attribute_value: attribute_value__,
                    subject_condition_set: subject_condition_set__,
                    actions: actions__.unwrap_or_default(),
                    namespace: namespace__,
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.SubjectMapping", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SubjectMappingOperatorEnum {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SUBJECT_MAPPING_OPERATOR_ENUM_UNSPECIFIED",
            Self::In => "SUBJECT_MAPPING_OPERATOR_ENUM_IN",
            Self::NotIn => "SUBJECT_MAPPING_OPERATOR_ENUM_NOT_IN",
            Self::InContains => "SUBJECT_MAPPING_OPERATOR_ENUM_IN_CONTAINS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SubjectMappingOperatorEnum {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SUBJECT_MAPPING_OPERATOR_ENUM_UNSPECIFIED",
            "SUBJECT_MAPPING_OPERATOR_ENUM_IN",
            "SUBJECT_MAPPING_OPERATOR_ENUM_NOT_IN",
            "SUBJECT_MAPPING_OPERATOR_ENUM_IN_CONTAINS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubjectMappingOperatorEnum;

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
                    "SUBJECT_MAPPING_OPERATOR_ENUM_UNSPECIFIED" => Ok(SubjectMappingOperatorEnum::Unspecified),
                    "SUBJECT_MAPPING_OPERATOR_ENUM_IN" => Ok(SubjectMappingOperatorEnum::In),
                    "SUBJECT_MAPPING_OPERATOR_ENUM_NOT_IN" => Ok(SubjectMappingOperatorEnum::NotIn),
                    "SUBJECT_MAPPING_OPERATOR_ENUM_IN_CONTAINS" => Ok(SubjectMappingOperatorEnum::InContains),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SubjectProperty {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.external_selector_value.is_empty() {
            len += 1;
        }
        if !self.external_value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.SubjectProperty", len)?;
        if !self.external_selector_value.is_empty() {
            struct_ser.serialize_field("externalSelectorValue", &self.external_selector_value)?;
        }
        if !self.external_value.is_empty() {
            struct_ser.serialize_field("externalValue", &self.external_value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubjectProperty {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "external_selector_value",
            "externalSelectorValue",
            "external_value",
            "externalValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExternalSelectorValue,
            ExternalValue,
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
                            "externalSelectorValue" | "external_selector_value" => Ok(GeneratedField::ExternalSelectorValue),
                            "externalValue" | "external_value" => Ok(GeneratedField::ExternalValue),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubjectProperty;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.SubjectProperty")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SubjectProperty, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut external_selector_value__ = None;
                let mut external_value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ExternalSelectorValue => {
                            if external_selector_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("externalSelectorValue"));
                            }
                            external_selector_value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExternalValue => {
                            if external_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("externalValue"));
                            }
                            external_value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(SubjectProperty {
                    external_selector_value: external_selector_value__.unwrap_or_default(),
                    external_value: external_value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.SubjectProperty", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SubjectSet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.condition_groups.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.SubjectSet", len)?;
        if !self.condition_groups.is_empty() {
            struct_ser.serialize_field("conditionGroups", &self.condition_groups)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubjectSet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "condition_groups",
            "conditionGroups",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConditionGroups,
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
                            "conditionGroups" | "condition_groups" => Ok(GeneratedField::ConditionGroups),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubjectSet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.SubjectSet")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SubjectSet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut condition_groups__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ConditionGroups => {
                            if condition_groups__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conditionGroups"));
                            }
                            condition_groups__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(SubjectSet {
                    condition_groups: condition_groups__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.SubjectSet", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SymmetricKey {
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
        if !self.key_id.is_empty() {
            len += 1;
        }
        if self.key_status != 0 {
            len += 1;
        }
        if self.key_mode != 0 {
            len += 1;
        }
        if !self.key_ctx.is_empty() {
            len += 1;
        }
        if self.provider_config.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.SymmetricKey", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.key_id.is_empty() {
            struct_ser.serialize_field("keyId", &self.key_id)?;
        }
        if self.key_status != 0 {
            let v = KeyStatus::try_from(self.key_status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.key_status)))?;
            struct_ser.serialize_field("keyStatus", &v)?;
        }
        if self.key_mode != 0 {
            let v = KeyMode::try_from(self.key_mode)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.key_mode)))?;
            struct_ser.serialize_field("keyMode", &v)?;
        }
        if !self.key_ctx.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("keyCtx", pbjson::private::base64::encode(&self.key_ctx).as_str())?;
        }
        if let Some(v) = self.provider_config.as_ref() {
            struct_ser.serialize_field("providerConfig", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SymmetricKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "key_id",
            "keyId",
            "key_status",
            "keyStatus",
            "key_mode",
            "keyMode",
            "key_ctx",
            "keyCtx",
            "provider_config",
            "providerConfig",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            KeyId,
            KeyStatus,
            KeyMode,
            KeyCtx,
            ProviderConfig,
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
                            "id" => Ok(GeneratedField::Id),
                            "keyId" | "key_id" => Ok(GeneratedField::KeyId),
                            "keyStatus" | "key_status" => Ok(GeneratedField::KeyStatus),
                            "keyMode" | "key_mode" => Ok(GeneratedField::KeyMode),
                            "keyCtx" | "key_ctx" => Ok(GeneratedField::KeyCtx),
                            "providerConfig" | "provider_config" => Ok(GeneratedField::ProviderConfig),
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
            type Value = SymmetricKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.SymmetricKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SymmetricKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut key_id__ = None;
                let mut key_status__ = None;
                let mut key_mode__ = None;
                let mut key_ctx__ = None;
                let mut provider_config__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KeyId => {
                            if key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyId"));
                            }
                            key_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KeyStatus => {
                            if key_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyStatus"));
                            }
                            key_status__ = Some(map_.next_value::<KeyStatus>()? as i32);
                        }
                        GeneratedField::KeyMode => {
                            if key_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyMode"));
                            }
                            key_mode__ = Some(map_.next_value::<KeyMode>()? as i32);
                        }
                        GeneratedField::KeyCtx => {
                            if key_ctx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyCtx"));
                            }
                            key_ctx__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProviderConfig => {
                            if provider_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerConfig"));
                            }
                            provider_config__ = map_.next_value()?;
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
                Ok(SymmetricKey {
                    id: id__.unwrap_or_default(),
                    key_id: key_id__.unwrap_or_default(),
                    key_status: key_status__.unwrap_or_default(),
                    key_mode: key_mode__.unwrap_or_default(),
                    key_ctx: key_ctx__.unwrap_or_default(),
                    provider_config: provider_config__,
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.SymmetricKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Value {
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
        if self.attribute.is_some() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        if !self.grants.is_empty() {
            len += 1;
        }
        if !self.fqn.is_empty() {
            len += 1;
        }
        if self.active.is_some() {
            len += 1;
        }
        if !self.subject_mappings.is_empty() {
            len += 1;
        }
        if !self.kas_keys.is_empty() {
            len += 1;
        }
        if !self.resource_mappings.is_empty() {
            len += 1;
        }
        if !self.obligations.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.Value", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.attribute.as_ref() {
            struct_ser.serialize_field("attribute", v)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        if !self.grants.is_empty() {
            struct_ser.serialize_field("grants", &self.grants)?;
        }
        if !self.fqn.is_empty() {
            struct_ser.serialize_field("fqn", &self.fqn)?;
        }
        if let Some(v) = self.active.as_ref() {
            struct_ser.serialize_field("active", v)?;
        }
        if !self.subject_mappings.is_empty() {
            struct_ser.serialize_field("subjectMappings", &self.subject_mappings)?;
        }
        if !self.kas_keys.is_empty() {
            struct_ser.serialize_field("kasKeys", &self.kas_keys)?;
        }
        if !self.resource_mappings.is_empty() {
            struct_ser.serialize_field("resourceMappings", &self.resource_mappings)?;
        }
        if !self.obligations.is_empty() {
            struct_ser.serialize_field("obligations", &self.obligations)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Value {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "attribute",
            "value",
            "grants",
            "fqn",
            "active",
            "subject_mappings",
            "subjectMappings",
            "kas_keys",
            "kasKeys",
            "resource_mappings",
            "resourceMappings",
            "obligations",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Attribute,
            Value,
            Grants,
            Fqn,
            Active,
            SubjectMappings,
            KasKeys,
            ResourceMappings,
            Obligations,
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
                            "id" => Ok(GeneratedField::Id),
                            "attribute" => Ok(GeneratedField::Attribute),
                            "value" => Ok(GeneratedField::Value),
                            "grants" => Ok(GeneratedField::Grants),
                            "fqn" => Ok(GeneratedField::Fqn),
                            "active" => Ok(GeneratedField::Active),
                            "subjectMappings" | "subject_mappings" => Ok(GeneratedField::SubjectMappings),
                            "kasKeys" | "kas_keys" => Ok(GeneratedField::KasKeys),
                            "resourceMappings" | "resource_mappings" => Ok(GeneratedField::ResourceMappings),
                            "obligations" => Ok(GeneratedField::Obligations),
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
            type Value = Value;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.Value")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Value, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut attribute__ = None;
                let mut value__ = None;
                let mut grants__ = None;
                let mut fqn__ = None;
                let mut active__ = None;
                let mut subject_mappings__ = None;
                let mut kas_keys__ = None;
                let mut resource_mappings__ = None;
                let mut obligations__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
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
                            value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Grants => {
                            if grants__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grants"));
                            }
                            grants__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fqn => {
                            if fqn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fqn"));
                            }
                            fqn__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Active => {
                            if active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("active"));
                            }
                            active__ = map_.next_value()?;
                        }
                        GeneratedField::SubjectMappings => {
                            if subject_mappings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectMappings"));
                            }
                            subject_mappings__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KasKeys => {
                            if kas_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kasKeys"));
                            }
                            kas_keys__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceMappings => {
                            if resource_mappings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceMappings"));
                            }
                            resource_mappings__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Obligations => {
                            if obligations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("obligations"));
                            }
                            obligations__ = Some(map_.next_value()?);
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
                Ok(Value {
                    id: id__.unwrap_or_default(),
                    attribute: attribute__,
                    value: value__.unwrap_or_default(),
                    grants: grants__.unwrap_or_default(),
                    fqn: fqn__.unwrap_or_default(),
                    active: active__,
                    subject_mappings: subject_mappings__.unwrap_or_default(),
                    kas_keys: kas_keys__.unwrap_or_default(),
                    resource_mappings: resource_mappings__.unwrap_or_default(),
                    obligations: obligations__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.Value", FIELDS, GeneratedVisitor)
    }
}
