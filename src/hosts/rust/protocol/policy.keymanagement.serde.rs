// @generated
impl serde::Serialize for CreateProviderConfigRequest {
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
        if !self.config_json.is_empty() {
            len += 1;
        }
        if !self.manager.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.keymanagement.CreateProviderConfigRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for CreateProviderConfigRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "config_json",
            "configJson",
            "manager",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = CreateProviderConfigRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.keymanagement.CreateProviderConfigRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateProviderConfigRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut config_json__ = None;
                let mut manager__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                Ok(CreateProviderConfigRequest {
                    name: name__.unwrap_or_default(),
                    config_json: config_json__.unwrap_or_default(),
                    manager: manager__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.keymanagement.CreateProviderConfigRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateProviderConfigResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.provider_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.keymanagement.CreateProviderConfigResponse", len)?;
        if let Some(v) = self.provider_config.as_ref() {
            struct_ser.serialize_field("providerConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateProviderConfigResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "provider_config",
            "providerConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProviderConfig,
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
                            "providerConfig" | "provider_config" => Ok(GeneratedField::ProviderConfig),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateProviderConfigResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.keymanagement.CreateProviderConfigResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateProviderConfigResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut provider_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProviderConfig => {
                            if provider_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerConfig"));
                            }
                            provider_config__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(CreateProviderConfigResponse {
                    provider_config: provider_config__,
                })
            }
        }
        deserializer.deserialize_struct("policy.keymanagement.CreateProviderConfigResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteProviderConfigRequest {
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
        let mut struct_ser = serializer.serialize_struct("policy.keymanagement.DeleteProviderConfigRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteProviderConfigRequest {
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
            type Value = DeleteProviderConfigRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.keymanagement.DeleteProviderConfigRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteProviderConfigRequest, V::Error>
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
                Ok(DeleteProviderConfigRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.keymanagement.DeleteProviderConfigRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteProviderConfigResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.provider_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.keymanagement.DeleteProviderConfigResponse", len)?;
        if let Some(v) = self.provider_config.as_ref() {
            struct_ser.serialize_field("providerConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteProviderConfigResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "provider_config",
            "providerConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProviderConfig,
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
                            "providerConfig" | "provider_config" => Ok(GeneratedField::ProviderConfig),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteProviderConfigResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.keymanagement.DeleteProviderConfigResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteProviderConfigResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut provider_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProviderConfig => {
                            if provider_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerConfig"));
                            }
                            provider_config__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DeleteProviderConfigResponse {
                    provider_config: provider_config__,
                })
            }
        }
        deserializer.deserialize_struct("policy.keymanagement.DeleteProviderConfigResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetProviderConfigRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.manager.is_empty() {
            len += 1;
        }
        if self.identifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.keymanagement.GetProviderConfigRequest", len)?;
        if !self.manager.is_empty() {
            struct_ser.serialize_field("manager", &self.manager)?;
        }
        if let Some(v) = self.identifier.as_ref() {
            match v {
                get_provider_config_request::Identifier::Id(v) => {
                    struct_ser.serialize_field("id", v)?;
                }
                get_provider_config_request::Identifier::Name(v) => {
                    struct_ser.serialize_field("name", v)?;
                }
                get_provider_config_request::Identifier::NameManager(v) => {
                    struct_ser.serialize_field("nameManager", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetProviderConfigRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "manager",
            "id",
            "name",
            "name_manager",
            "nameManager",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Manager,
            Id,
            Name,
            NameManager,
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
                            "manager" => Ok(GeneratedField::Manager),
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "nameManager" | "name_manager" => Ok(GeneratedField::NameManager),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetProviderConfigRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.keymanagement.GetProviderConfigRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetProviderConfigRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut manager__ = None;
                let mut identifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Manager => {
                            if manager__.is_some() {
                                return Err(serde::de::Error::duplicate_field("manager"));
                            }
                            manager__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Id => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(get_provider_config_request::Identifier::Id);
                        }
                        GeneratedField::Name => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(get_provider_config_request::Identifier::Name);
                        }
                        GeneratedField::NameManager => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nameManager"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(get_provider_config_request::Identifier::NameManager)
;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetProviderConfigRequest {
                    manager: manager__.unwrap_or_default(),
                    identifier: identifier__,
                })
            }
        }
        deserializer.deserialize_struct("policy.keymanagement.GetProviderConfigRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for get_provider_config_request::NameManager {
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
        if !self.manager.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.keymanagement.GetProviderConfigRequest.NameManager", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.manager.is_empty() {
            struct_ser.serialize_field("manager", &self.manager)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for get_provider_config_request::NameManager {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "manager",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Manager,
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
                            "manager" => Ok(GeneratedField::Manager),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = get_provider_config_request::NameManager;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.keymanagement.GetProviderConfigRequest.NameManager")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<get_provider_config_request::NameManager, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut manager__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Manager => {
                            if manager__.is_some() {
                                return Err(serde::de::Error::duplicate_field("manager"));
                            }
                            manager__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(get_provider_config_request::NameManager {
                    name: name__.unwrap_or_default(),
                    manager: manager__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.keymanagement.GetProviderConfigRequest.NameManager", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetProviderConfigResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.provider_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.keymanagement.GetProviderConfigResponse", len)?;
        if let Some(v) = self.provider_config.as_ref() {
            struct_ser.serialize_field("providerConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetProviderConfigResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "provider_config",
            "providerConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProviderConfig,
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
                            "providerConfig" | "provider_config" => Ok(GeneratedField::ProviderConfig),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetProviderConfigResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.keymanagement.GetProviderConfigResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetProviderConfigResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut provider_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProviderConfig => {
                            if provider_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerConfig"));
                            }
                            provider_config__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetProviderConfigResponse {
                    provider_config: provider_config__,
                })
            }
        }
        deserializer.deserialize_struct("policy.keymanagement.GetProviderConfigResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListProviderConfigsRequest {
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
        let mut struct_ser = serializer.serialize_struct("policy.keymanagement.ListProviderConfigsRequest", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListProviderConfigsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = ListProviderConfigsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.keymanagement.ListProviderConfigsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListProviderConfigsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                Ok(ListProviderConfigsRequest {
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("policy.keymanagement.ListProviderConfigsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListProviderConfigsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.provider_configs.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.keymanagement.ListProviderConfigsResponse", len)?;
        if !self.provider_configs.is_empty() {
            struct_ser.serialize_field("providerConfigs", &self.provider_configs)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListProviderConfigsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "provider_configs",
            "providerConfigs",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProviderConfigs,
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
                            "providerConfigs" | "provider_configs" => Ok(GeneratedField::ProviderConfigs),
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
            type Value = ListProviderConfigsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.keymanagement.ListProviderConfigsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListProviderConfigsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut provider_configs__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProviderConfigs => {
                            if provider_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerConfigs"));
                            }
                            provider_configs__ = Some(map_.next_value()?);
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
                Ok(ListProviderConfigsResponse {
                    provider_configs: provider_configs__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("policy.keymanagement.ListProviderConfigsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateProviderConfigRequest {
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
        if self.metadata_update_behavior != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.keymanagement.UpdateProviderConfigRequest", len)?;
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
        if self.metadata_update_behavior != 0 {
            let v = super::super::common::MetadataUpdateEnum::try_from(self.metadata_update_behavior)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.metadata_update_behavior)))?;
            struct_ser.serialize_field("metadataUpdateBehavior", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateProviderConfigRequest {
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
            "metadata_update_behavior",
            "metadataUpdateBehavior",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            ConfigJson,
            Manager,
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
                            "name" => Ok(GeneratedField::Name),
                            "configJson" | "config_json" => Ok(GeneratedField::ConfigJson),
                            "manager" => Ok(GeneratedField::Manager),
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
            type Value = UpdateProviderConfigRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.keymanagement.UpdateProviderConfigRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateProviderConfigRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut config_json__ = None;
                let mut manager__ = None;
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
                Ok(UpdateProviderConfigRequest {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    config_json: config_json__.unwrap_or_default(),
                    manager: manager__.unwrap_or_default(),
                    metadata: metadata__,
                    metadata_update_behavior: metadata_update_behavior__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.keymanagement.UpdateProviderConfigRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateProviderConfigResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.provider_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.keymanagement.UpdateProviderConfigResponse", len)?;
        if let Some(v) = self.provider_config.as_ref() {
            struct_ser.serialize_field("providerConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateProviderConfigResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "provider_config",
            "providerConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProviderConfig,
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
                            "providerConfig" | "provider_config" => Ok(GeneratedField::ProviderConfig),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateProviderConfigResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.keymanagement.UpdateProviderConfigResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateProviderConfigResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut provider_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProviderConfig => {
                            if provider_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerConfig"));
                            }
                            provider_config__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(UpdateProviderConfigResponse {
                    provider_config: provider_config__,
                })
            }
        }
        deserializer.deserialize_struct("policy.keymanagement.UpdateProviderConfigResponse", FIELDS, GeneratedVisitor)
    }
}
