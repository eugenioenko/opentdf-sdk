// @generated
impl serde::Serialize for CreateEntityChainsFromTokensRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tokens.is_empty() {
            len += 1;
        }
        if !self.resources.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("entityresolution.v2.CreateEntityChainsFromTokensRequest", len)?;
        if !self.tokens.is_empty() {
            struct_ser.serialize_field("tokens", &self.tokens)?;
        }
        if !self.resources.is_empty() {
            struct_ser.serialize_field("resources", &self.resources)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateEntityChainsFromTokensRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tokens",
            "resources",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tokens,
            Resources,
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
                            "tokens" => Ok(GeneratedField::Tokens),
                            "resources" => Ok(GeneratedField::Resources),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateEntityChainsFromTokensRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entityresolution.v2.CreateEntityChainsFromTokensRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateEntityChainsFromTokensRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tokens__ = None;
                let mut resources__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tokens => {
                            if tokens__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokens"));
                            }
                            tokens__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Resources => {
                            if resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resources"));
                            }
                            resources__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(CreateEntityChainsFromTokensRequest {
                    tokens: tokens__.unwrap_or_default(),
                    resources: resources__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("entityresolution.v2.CreateEntityChainsFromTokensRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateEntityChainsFromTokensResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.entity_chains.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("entityresolution.v2.CreateEntityChainsFromTokensResponse", len)?;
        if !self.entity_chains.is_empty() {
            struct_ser.serialize_field("entityChains", &self.entity_chains)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateEntityChainsFromTokensResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity_chains",
            "entityChains",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EntityChains,
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
                            "entityChains" | "entity_chains" => Ok(GeneratedField::EntityChains),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateEntityChainsFromTokensResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entityresolution.v2.CreateEntityChainsFromTokensResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateEntityChainsFromTokensResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity_chains__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EntityChains => {
                            if entity_chains__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityChains"));
                            }
                            entity_chains__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(CreateEntityChainsFromTokensResponse {
                    entity_chains: entity_chains__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("entityresolution.v2.CreateEntityChainsFromTokensResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DirectEntitlement {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.attribute_value_fqn.is_empty() {
            len += 1;
        }
        if !self.actions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("entityresolution.v2.DirectEntitlement", len)?;
        if !self.attribute_value_fqn.is_empty() {
            struct_ser.serialize_field("attributeValueFqn", &self.attribute_value_fqn)?;
        }
        if !self.actions.is_empty() {
            struct_ser.serialize_field("actions", &self.actions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DirectEntitlement {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attribute_value_fqn",
            "attributeValueFqn",
            "actions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AttributeValueFqn,
            Actions,
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
                            "attributeValueFqn" | "attribute_value_fqn" => Ok(GeneratedField::AttributeValueFqn),
                            "actions" => Ok(GeneratedField::Actions),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DirectEntitlement;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entityresolution.v2.DirectEntitlement")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DirectEntitlement, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attribute_value_fqn__ = None;
                let mut actions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AttributeValueFqn => {
                            if attribute_value_fqn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeValueFqn"));
                            }
                            attribute_value_fqn__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Actions => {
                            if actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            actions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DirectEntitlement {
                    attribute_value_fqn: attribute_value_fqn__.unwrap_or_default(),
                    actions: actions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("entityresolution.v2.DirectEntitlement", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EntityNotFoundError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code != 0 {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        if !self.details.is_empty() {
            len += 1;
        }
        if !self.entity.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("entityresolution.v2.EntityNotFoundError", len)?;
        if self.code != 0 {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        if !self.details.is_empty() {
            struct_ser.serialize_field("details", &self.details)?;
        }
        if !self.entity.is_empty() {
            struct_ser.serialize_field("entity", &self.entity)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EntityNotFoundError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "message",
            "details",
            "entity",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Message,
            Details,
            Entity,
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
                            "code" => Ok(GeneratedField::Code),
                            "message" => Ok(GeneratedField::Message),
                            "details" => Ok(GeneratedField::Details),
                            "entity" => Ok(GeneratedField::Entity),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EntityNotFoundError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entityresolution.v2.EntityNotFoundError")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EntityNotFoundError, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut message__ = None;
                let mut details__ = None;
                let mut entity__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Entity => {
                            if entity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entity"));
                            }
                            entity__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(EntityNotFoundError {
                    code: code__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                    details: details__.unwrap_or_default(),
                    entity: entity__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("entityresolution.v2.EntityNotFoundError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EntityRepresentation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.original_id.is_empty() {
            len += 1;
        }
        if !self.additional_props.is_empty() {
            len += 1;
        }
        if !self.direct_entitlements.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("entityresolution.v2.EntityRepresentation", len)?;
        if !self.original_id.is_empty() {
            struct_ser.serialize_field("originalId", &self.original_id)?;
        }
        if !self.additional_props.is_empty() {
            struct_ser.serialize_field("additionalProps", &self.additional_props)?;
        }
        if !self.direct_entitlements.is_empty() {
            struct_ser.serialize_field("directEntitlements", &self.direct_entitlements)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EntityRepresentation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "original_id",
            "originalId",
            "additional_props",
            "additionalProps",
            "direct_entitlements",
            "directEntitlements",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OriginalId,
            AdditionalProps,
            DirectEntitlements,
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
                            "originalId" | "original_id" => Ok(GeneratedField::OriginalId),
                            "additionalProps" | "additional_props" => Ok(GeneratedField::AdditionalProps),
                            "directEntitlements" | "direct_entitlements" => Ok(GeneratedField::DirectEntitlements),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EntityRepresentation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entityresolution.v2.EntityRepresentation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EntityRepresentation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut original_id__ = None;
                let mut additional_props__ = None;
                let mut direct_entitlements__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OriginalId => {
                            if original_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originalId"));
                            }
                            original_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AdditionalProps => {
                            if additional_props__.is_some() {
                                return Err(serde::de::Error::duplicate_field("additionalProps"));
                            }
                            additional_props__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DirectEntitlements => {
                            if direct_entitlements__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directEntitlements"));
                            }
                            direct_entitlements__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(EntityRepresentation {
                    original_id: original_id__.unwrap_or_default(),
                    additional_props: additional_props__.unwrap_or_default(),
                    direct_entitlements: direct_entitlements__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("entityresolution.v2.EntityRepresentation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResolveEntitiesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.entities.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("entityresolution.v2.ResolveEntitiesRequest", len)?;
        if !self.entities.is_empty() {
            struct_ser.serialize_field("entities", &self.entities)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResolveEntitiesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entities",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Entities,
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
                            "entities" => Ok(GeneratedField::Entities),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResolveEntitiesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entityresolution.v2.ResolveEntitiesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResolveEntitiesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entities__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Entities => {
                            if entities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entities"));
                            }
                            entities__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ResolveEntitiesRequest {
                    entities: entities__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("entityresolution.v2.ResolveEntitiesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResolveEntitiesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.entity_representations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("entityresolution.v2.ResolveEntitiesResponse", len)?;
        if !self.entity_representations.is_empty() {
            struct_ser.serialize_field("entityRepresentations", &self.entity_representations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResolveEntitiesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity_representations",
            "entityRepresentations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EntityRepresentations,
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
                            "entityRepresentations" | "entity_representations" => Ok(GeneratedField::EntityRepresentations),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResolveEntitiesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct entityresolution.v2.ResolveEntitiesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResolveEntitiesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity_representations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EntityRepresentations => {
                            if entity_representations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityRepresentations"));
                            }
                            entity_representations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ResolveEntitiesResponse {
                    entity_representations: entity_representations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("entityresolution.v2.ResolveEntitiesResponse", FIELDS, GeneratedVisitor)
    }
}
