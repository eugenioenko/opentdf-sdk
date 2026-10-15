// @generated
impl serde::Serialize for GetWellKnownConfigurationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("wellknownconfiguration.GetWellKnownConfigurationRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetWellKnownConfigurationRequest {
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
            type Value = GetWellKnownConfigurationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct wellknownconfiguration.GetWellKnownConfigurationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetWellKnownConfigurationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GetWellKnownConfigurationRequest {
                })
            }
        }
        deserializer.deserialize_struct("wellknownconfiguration.GetWellKnownConfigurationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetWellKnownConfigurationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.configuration.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("wellknownconfiguration.GetWellKnownConfigurationResponse", len)?;
        if let Some(v) = self.configuration.as_ref() {
            struct_ser.serialize_field("configuration", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetWellKnownConfigurationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "configuration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Configuration,
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
                            "configuration" => Ok(GeneratedField::Configuration),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetWellKnownConfigurationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct wellknownconfiguration.GetWellKnownConfigurationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetWellKnownConfigurationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut configuration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Configuration => {
                            if configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configuration"));
                            }
                            configuration__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetWellKnownConfigurationResponse {
                    configuration: configuration__,
                })
            }
        }
        deserializer.deserialize_struct("wellknownconfiguration.GetWellKnownConfigurationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WellKnownConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.configuration.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("wellknownconfiguration.WellKnownConfig", len)?;
        if !self.configuration.is_empty() {
            struct_ser.serialize_field("configuration", &self.configuration)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WellKnownConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "configuration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Configuration,
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
                            "configuration" => Ok(GeneratedField::Configuration),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WellKnownConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct wellknownconfiguration.WellKnownConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WellKnownConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut configuration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Configuration => {
                            if configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configuration"));
                            }
                            configuration__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(WellKnownConfig {
                    configuration: configuration__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("wellknownconfiguration.WellKnownConfig", FIELDS, GeneratedVisitor)
    }
}
