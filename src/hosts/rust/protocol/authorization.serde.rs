// @generated
impl serde::Serialize for DecisionRequest {
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
        if !self.entity_chains.is_empty() {
            len += 1;
        }
        if !self.resource_attributes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authorization.DecisionRequest", len)?;
        if !self.actions.is_empty() {
            struct_ser.serialize_field("actions", &self.actions)?;
        }
        if !self.entity_chains.is_empty() {
            struct_ser.serialize_field("entityChains", &self.entity_chains)?;
        }
        if !self.resource_attributes.is_empty() {
            struct_ser.serialize_field("resourceAttributes", &self.resource_attributes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DecisionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "actions",
            "entity_chains",
            "entityChains",
            "resource_attributes",
            "resourceAttributes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Actions,
            EntityChains,
            ResourceAttributes,
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
                            "entityChains" | "entity_chains" => Ok(GeneratedField::EntityChains),
                            "resourceAttributes" | "resource_attributes" => Ok(GeneratedField::ResourceAttributes),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DecisionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authorization.DecisionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DecisionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut actions__ = None;
                let mut entity_chains__ = None;
                let mut resource_attributes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Actions => {
                            if actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            actions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EntityChains => {
                            if entity_chains__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityChains"));
                            }
                            entity_chains__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceAttributes => {
                            if resource_attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributes"));
                            }
                            resource_attributes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DecisionRequest {
                    actions: actions__.unwrap_or_default(),
                    entity_chains: entity_chains__.unwrap_or_default(),
                    resource_attributes: resource_attributes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authorization.DecisionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DecisionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.entity_chain_id.is_empty() {
            len += 1;
        }
        if !self.resource_attributes_id.is_empty() {
            len += 1;
        }
        if self.action.is_some() {
            len += 1;
        }
        if self.decision != 0 {
            len += 1;
        }
        if !self.obligations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authorization.DecisionResponse", len)?;
        if !self.entity_chain_id.is_empty() {
            struct_ser.serialize_field("entityChainId", &self.entity_chain_id)?;
        }
        if !self.resource_attributes_id.is_empty() {
            struct_ser.serialize_field("resourceAttributesId", &self.resource_attributes_id)?;
        }
        if let Some(v) = self.action.as_ref() {
            struct_ser.serialize_field("action", v)?;
        }
        if self.decision != 0 {
            let v = decision_response::Decision::try_from(self.decision)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.decision)))?;
            struct_ser.serialize_field("decision", &v)?;
        }
        if !self.obligations.is_empty() {
            struct_ser.serialize_field("obligations", &self.obligations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DecisionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity_chain_id",
            "entityChainId",
            "resource_attributes_id",
            "resourceAttributesId",
            "action",
            "decision",
            "obligations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EntityChainId,
            ResourceAttributesId,
            Action,
            Decision,
            Obligations,
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
                            "entityChainId" | "entity_chain_id" => Ok(GeneratedField::EntityChainId),
                            "resourceAttributesId" | "resource_attributes_id" => Ok(GeneratedField::ResourceAttributesId),
                            "action" => Ok(GeneratedField::Action),
                            "decision" => Ok(GeneratedField::Decision),
                            "obligations" => Ok(GeneratedField::Obligations),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DecisionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authorization.DecisionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DecisionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity_chain_id__ = None;
                let mut resource_attributes_id__ = None;
                let mut action__ = None;
                let mut decision__ = None;
                let mut obligations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EntityChainId => {
                            if entity_chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityChainId"));
                            }
                            entity_chain_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceAttributesId => {
                            if resource_attributes_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributesId"));
                            }
                            resource_attributes_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Action => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            action__ = map_.next_value()?;
                        }
                        GeneratedField::Decision => {
                            if decision__.is_some() {
                                return Err(serde::de::Error::duplicate_field("decision"));
                            }
                            decision__ = Some(map_.next_value::<decision_response::Decision>()? as i32);
                        }
                        GeneratedField::Obligations => {
                            if obligations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("obligations"));
                            }
                            obligations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DecisionResponse {
                    entity_chain_id: entity_chain_id__.unwrap_or_default(),
                    resource_attributes_id: resource_attributes_id__.unwrap_or_default(),
                    action: action__,
                    decision: decision__.unwrap_or_default(),
                    obligations: obligations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authorization.DecisionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for decision_response::Decision {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "DECISION_UNSPECIFIED",
            Self::Deny => "DECISION_DENY",
            Self::Permit => "DECISION_PERMIT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for decision_response::Decision {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DECISION_UNSPECIFIED",
            "DECISION_DENY",
            "DECISION_PERMIT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = decision_response::Decision;

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
                    "DECISION_UNSPECIFIED" => Ok(decision_response::Decision::Unspecified),
                    "DECISION_DENY" => Ok(decision_response::Decision::Deny),
                    "DECISION_PERMIT" => Ok(decision_response::Decision::Permit),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Entity {
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
        if self.category != 0 {
            len += 1;
        }
        if self.entity_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authorization.Entity", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if self.category != 0 {
            let v = entity::Category::try_from(self.category)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.category)))?;
            struct_ser.serialize_field("category", &v)?;
        }
        if let Some(v) = self.entity_type.as_ref() {
            match v {
                entity::EntityType::EmailAddress(v) => {
                    struct_ser.serialize_field("emailAddress", v)?;
                }
                entity::EntityType::UserName(v) => {
                    struct_ser.serialize_field("userName", v)?;
                }
                entity::EntityType::RemoteClaimsUrl(v) => {
                    struct_ser.serialize_field("remoteClaimsUrl", v)?;
                }
                entity::EntityType::Uuid(v) => {
                    struct_ser.serialize_field("uuid", v)?;
                }
                entity::EntityType::Claims(v) => {
                    struct_ser.serialize_field("claims", v)?;
                }
                entity::EntityType::Custom(v) => {
                    struct_ser.serialize_field("custom", v)?;
                }
                entity::EntityType::ClientId(v) => {
                    struct_ser.serialize_field("clientId", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Entity {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "category",
            "email_address",
            "emailAddress",
            "user_name",
            "userName",
            "remote_claims_url",
            "remoteClaimsUrl",
            "uuid",
            "claims",
            "custom",
            "client_id",
            "clientId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Category,
            EmailAddress,
            UserName,
            RemoteClaimsUrl,
            Uuid,
            Claims,
            Custom,
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
                            "id" => Ok(GeneratedField::Id),
                            "category" => Ok(GeneratedField::Category),
                            "emailAddress" | "email_address" => Ok(GeneratedField::EmailAddress),
                            "userName" | "user_name" => Ok(GeneratedField::UserName),
                            "remoteClaimsUrl" | "remote_claims_url" => Ok(GeneratedField::RemoteClaimsUrl),
                            "uuid" => Ok(GeneratedField::Uuid),
                            "claims" => Ok(GeneratedField::Claims),
                            "custom" => Ok(GeneratedField::Custom),
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
            type Value = Entity;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authorization.Entity")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Entity, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut category__ = None;
                let mut entity_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Category => {
                            if category__.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            category__ = Some(map_.next_value::<entity::Category>()? as i32);
                        }
                        GeneratedField::EmailAddress => {
                            if entity_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emailAddress"));
                            }
                            entity_type__ = map_.next_value::<::std::option::Option<_>>()?.map(entity::EntityType::EmailAddress);
                        }
                        GeneratedField::UserName => {
                            if entity_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userName"));
                            }
                            entity_type__ = map_.next_value::<::std::option::Option<_>>()?.map(entity::EntityType::UserName);
                        }
                        GeneratedField::RemoteClaimsUrl => {
                            if entity_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteClaimsUrl"));
                            }
                            entity_type__ = map_.next_value::<::std::option::Option<_>>()?.map(entity::EntityType::RemoteClaimsUrl);
                        }
                        GeneratedField::Uuid => {
                            if entity_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            entity_type__ = map_.next_value::<::std::option::Option<_>>()?.map(entity::EntityType::Uuid);
                        }
                        GeneratedField::Claims => {
                            if entity_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("claims"));
                            }
                            entity_type__ = map_.next_value::<::std::option::Option<_>>()?.map(entity::EntityType::Claims)
;
                        }
                        GeneratedField::Custom => {
                            if entity_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("custom"));
                            }
                            entity_type__ = map_.next_value::<::std::option::Option<_>>()?.map(entity::EntityType::Custom)
;
                        }
                        GeneratedField::ClientId => {
                            if entity_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            entity_type__ = map_.next_value::<::std::option::Option<_>>()?.map(entity::EntityType::ClientId);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Entity {
                    id: id__.unwrap_or_default(),
                    category: category__.unwrap_or_default(),
                    entity_type: entity_type__,
                })
            }
        }
        deserializer.deserialize_struct("authorization.Entity", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for entity::Category {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "CATEGORY_UNSPECIFIED",
            Self::Subject => "CATEGORY_SUBJECT",
            Self::Environment => "CATEGORY_ENVIRONMENT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for entity::Category {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CATEGORY_UNSPECIFIED",
            "CATEGORY_SUBJECT",
            "CATEGORY_ENVIRONMENT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = entity::Category;

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
                    "CATEGORY_UNSPECIFIED" => Ok(entity::Category::Unspecified),
                    "CATEGORY_SUBJECT" => Ok(entity::Category::Subject),
                    "CATEGORY_ENVIRONMENT" => Ok(entity::Category::Environment),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for EntityChain {
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
        if !self.entities.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authorization.EntityChain", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.entities.is_empty() {
            struct_ser.serialize_field("entities", &self.entities)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EntityChain {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "entities",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
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
                            "id" => Ok(GeneratedField::Id),
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
            type Value = EntityChain;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authorization.EntityChain")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EntityChain, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut entities__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
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
                Ok(EntityChain {
                    id: id__.unwrap_or_default(),
                    entities: entities__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authorization.EntityChain", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EntityCustom {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.extension.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authorization.EntityCustom", len)?;
        if let Some(v) = self.extension.as_ref() {
            struct_ser.serialize_field("extension", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EntityCustom {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "extension",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Extension,
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
                            "extension" => Ok(GeneratedField::Extension),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EntityCustom;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authorization.EntityCustom")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EntityCustom, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut extension__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Extension => {
                            if extension__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            extension__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(EntityCustom {
                    extension: extension__,
                })
            }
        }
        deserializer.deserialize_struct("authorization.EntityCustom", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EntityEntitlements {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.entity_id.is_empty() {
            len += 1;
        }
        if !self.attribute_value_fqns.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authorization.EntityEntitlements", len)?;
        if !self.entity_id.is_empty() {
            struct_ser.serialize_field("entityId", &self.entity_id)?;
        }
        if !self.attribute_value_fqns.is_empty() {
            struct_ser.serialize_field("attributeValueFqns", &self.attribute_value_fqns)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EntityEntitlements {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity_id",
            "entityId",
            "attribute_value_fqns",
            "attributeValueFqns",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EntityId,
            AttributeValueFqns,
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
                            "entityId" | "entity_id" => Ok(GeneratedField::EntityId),
                            "attributeValueFqns" | "attribute_value_fqns" => Ok(GeneratedField::AttributeValueFqns),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EntityEntitlements;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authorization.EntityEntitlements")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EntityEntitlements, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity_id__ = None;
                let mut attribute_value_fqns__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EntityId => {
                            if entity_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityId"));
                            }
                            entity_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AttributeValueFqns => {
                            if attribute_value_fqns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeValueFqns"));
                            }
                            attribute_value_fqns__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(EntityEntitlements {
                    entity_id: entity_id__.unwrap_or_default(),
                    attribute_value_fqns: attribute_value_fqns__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authorization.EntityEntitlements", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDecisionsByTokenRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.decision_requests.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authorization.GetDecisionsByTokenRequest", len)?;
        if !self.decision_requests.is_empty() {
            struct_ser.serialize_field("decisionRequests", &self.decision_requests)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDecisionsByTokenRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "decision_requests",
            "decisionRequests",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DecisionRequests,
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
                            "decisionRequests" | "decision_requests" => Ok(GeneratedField::DecisionRequests),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDecisionsByTokenRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authorization.GetDecisionsByTokenRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetDecisionsByTokenRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut decision_requests__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DecisionRequests => {
                            if decision_requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("decisionRequests"));
                            }
                            decision_requests__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetDecisionsByTokenRequest {
                    decision_requests: decision_requests__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authorization.GetDecisionsByTokenRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDecisionsByTokenResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.decision_responses.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authorization.GetDecisionsByTokenResponse", len)?;
        if !self.decision_responses.is_empty() {
            struct_ser.serialize_field("decisionResponses", &self.decision_responses)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDecisionsByTokenResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "decision_responses",
            "decisionResponses",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DecisionResponses,
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
                            "decisionResponses" | "decision_responses" => Ok(GeneratedField::DecisionResponses),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDecisionsByTokenResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authorization.GetDecisionsByTokenResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetDecisionsByTokenResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut decision_responses__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DecisionResponses => {
                            if decision_responses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("decisionResponses"));
                            }
                            decision_responses__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetDecisionsByTokenResponse {
                    decision_responses: decision_responses__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authorization.GetDecisionsByTokenResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDecisionsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.decision_requests.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authorization.GetDecisionsRequest", len)?;
        if !self.decision_requests.is_empty() {
            struct_ser.serialize_field("decisionRequests", &self.decision_requests)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDecisionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "decision_requests",
            "decisionRequests",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DecisionRequests,
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
                            "decisionRequests" | "decision_requests" => Ok(GeneratedField::DecisionRequests),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDecisionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authorization.GetDecisionsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetDecisionsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut decision_requests__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DecisionRequests => {
                            if decision_requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("decisionRequests"));
                            }
                            decision_requests__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetDecisionsRequest {
                    decision_requests: decision_requests__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authorization.GetDecisionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDecisionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.decision_responses.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authorization.GetDecisionsResponse", len)?;
        if !self.decision_responses.is_empty() {
            struct_ser.serialize_field("decisionResponses", &self.decision_responses)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDecisionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "decision_responses",
            "decisionResponses",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DecisionResponses,
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
                            "decisionResponses" | "decision_responses" => Ok(GeneratedField::DecisionResponses),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDecisionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authorization.GetDecisionsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetDecisionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut decision_responses__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DecisionResponses => {
                            if decision_responses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("decisionResponses"));
                            }
                            decision_responses__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetDecisionsResponse {
                    decision_responses: decision_responses__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authorization.GetDecisionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetEntitlementsRequest {
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
        if self.scope.is_some() {
            len += 1;
        }
        if self.with_comprehensive_hierarchy.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authorization.GetEntitlementsRequest", len)?;
        if !self.entities.is_empty() {
            struct_ser.serialize_field("entities", &self.entities)?;
        }
        if let Some(v) = self.scope.as_ref() {
            struct_ser.serialize_field("scope", v)?;
        }
        if let Some(v) = self.with_comprehensive_hierarchy.as_ref() {
            struct_ser.serialize_field("withComprehensiveHierarchy", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetEntitlementsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entities",
            "scope",
            "with_comprehensive_hierarchy",
            "withComprehensiveHierarchy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Entities,
            Scope,
            WithComprehensiveHierarchy,
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
                            "scope" => Ok(GeneratedField::Scope),
                            "withComprehensiveHierarchy" | "with_comprehensive_hierarchy" => Ok(GeneratedField::WithComprehensiveHierarchy),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetEntitlementsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authorization.GetEntitlementsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetEntitlementsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entities__ = None;
                let mut scope__ = None;
                let mut with_comprehensive_hierarchy__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Entities => {
                            if entities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entities"));
                            }
                            entities__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Scope => {
                            if scope__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scope"));
                            }
                            scope__ = map_.next_value()?;
                        }
                        GeneratedField::WithComprehensiveHierarchy => {
                            if with_comprehensive_hierarchy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withComprehensiveHierarchy"));
                            }
                            with_comprehensive_hierarchy__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetEntitlementsRequest {
                    entities: entities__.unwrap_or_default(),
                    scope: scope__,
                    with_comprehensive_hierarchy: with_comprehensive_hierarchy__,
                })
            }
        }
        deserializer.deserialize_struct("authorization.GetEntitlementsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetEntitlementsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.entitlements.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authorization.GetEntitlementsResponse", len)?;
        if !self.entitlements.is_empty() {
            struct_ser.serialize_field("entitlements", &self.entitlements)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetEntitlementsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entitlements",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Entitlements,
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
                            "entitlements" => Ok(GeneratedField::Entitlements),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetEntitlementsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authorization.GetEntitlementsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetEntitlementsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entitlements__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Entitlements => {
                            if entitlements__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entitlements"));
                            }
                            entitlements__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetEntitlementsResponse {
                    entitlements: entitlements__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authorization.GetEntitlementsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceAttribute {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_attributes_id.is_empty() {
            len += 1;
        }
        if !self.attribute_value_fqns.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authorization.ResourceAttribute", len)?;
        if !self.resource_attributes_id.is_empty() {
            struct_ser.serialize_field("resourceAttributesId", &self.resource_attributes_id)?;
        }
        if !self.attribute_value_fqns.is_empty() {
            struct_ser.serialize_field("attributeValueFqns", &self.attribute_value_fqns)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceAttribute {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attributes_id",
            "resourceAttributesId",
            "attribute_value_fqns",
            "attributeValueFqns",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributesId,
            AttributeValueFqns,
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
                            "resourceAttributesId" | "resource_attributes_id" => Ok(GeneratedField::ResourceAttributesId),
                            "attributeValueFqns" | "attribute_value_fqns" => Ok(GeneratedField::AttributeValueFqns),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceAttribute;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authorization.ResourceAttribute")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceAttribute, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attributes_id__ = None;
                let mut attribute_value_fqns__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributesId => {
                            if resource_attributes_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributesId"));
                            }
                            resource_attributes_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AttributeValueFqns => {
                            if attribute_value_fqns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeValueFqns"));
                            }
                            attribute_value_fqns__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ResourceAttribute {
                    resource_attributes_id: resource_attributes_id__.unwrap_or_default(),
                    attribute_value_fqns: attribute_value_fqns__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authorization.ResourceAttribute", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Token {
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
        if !self.jwt.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authorization.Token", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.jwt.is_empty() {
            struct_ser.serialize_field("jwt", &self.jwt)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Token {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "jwt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Jwt,
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
                            "jwt" => Ok(GeneratedField::Jwt),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Token;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authorization.Token")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Token, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut jwt__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Jwt => {
                            if jwt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jwt"));
                            }
                            jwt__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Token {
                    id: id__.unwrap_or_default(),
                    jwt: jwt__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authorization.Token", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TokenDecisionRequest {
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
        if !self.tokens.is_empty() {
            len += 1;
        }
        if !self.resource_attributes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authorization.TokenDecisionRequest", len)?;
        if !self.actions.is_empty() {
            struct_ser.serialize_field("actions", &self.actions)?;
        }
        if !self.tokens.is_empty() {
            struct_ser.serialize_field("tokens", &self.tokens)?;
        }
        if !self.resource_attributes.is_empty() {
            struct_ser.serialize_field("resourceAttributes", &self.resource_attributes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TokenDecisionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "actions",
            "tokens",
            "resource_attributes",
            "resourceAttributes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Actions,
            Tokens,
            ResourceAttributes,
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
                            "tokens" => Ok(GeneratedField::Tokens),
                            "resourceAttributes" | "resource_attributes" => Ok(GeneratedField::ResourceAttributes),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TokenDecisionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authorization.TokenDecisionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TokenDecisionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut actions__ = None;
                let mut tokens__ = None;
                let mut resource_attributes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Actions => {
                            if actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            actions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Tokens => {
                            if tokens__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokens"));
                            }
                            tokens__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceAttributes => {
                            if resource_attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributes"));
                            }
                            resource_attributes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(TokenDecisionRequest {
                    actions: actions__.unwrap_or_default(),
                    tokens: tokens__.unwrap_or_default(),
                    resource_attributes: resource_attributes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authorization.TokenDecisionRequest", FIELDS, GeneratedVisitor)
    }
}
