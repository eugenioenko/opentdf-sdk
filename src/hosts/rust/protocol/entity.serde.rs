// @generated
impl serde::Serialize for Entity {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.ephemeral_id.is_empty() {
            len += 1;
        }
        if self.category != 0 {
            len += 1;
        }
        if self.entity_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("entity.Entity", len)?;
        if !self.ephemeral_id.is_empty() {
            struct_ser.serialize_field("ephemeralId", &self.ephemeral_id)?;
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
                entity::EntityType::Claims(v) => {
                    struct_ser.serialize_field("claims", v)?;
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
            "ephemeral_id",
            "ephemeralId",
            "category",
            "email_address",
            "emailAddress",
            "user_name",
            "userName",
            "claims",
            "client_id",
            "clientId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EphemeralId,
            Category,
            EmailAddress,
            UserName,
            Claims,
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
                            "ephemeralId" | "ephemeral_id" => Ok(GeneratedField::EphemeralId),
                            "category" => Ok(GeneratedField::Category),
                            "emailAddress" | "email_address" => Ok(GeneratedField::EmailAddress),
                            "userName" | "user_name" => Ok(GeneratedField::UserName),
                            "claims" => Ok(GeneratedField::Claims),
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
                formatter.write_str("struct entity.Entity")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Entity, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ephemeral_id__ = None;
                let mut category__ = None;
                let mut entity_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EphemeralId => {
                            if ephemeral_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ephemeralId"));
                            }
                            ephemeral_id__ = Some(map_.next_value()?);
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
                        GeneratedField::Claims => {
                            if entity_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("claims"));
                            }
                            entity_type__ = map_.next_value::<::std::option::Option<_>>()?.map(entity::EntityType::Claims)
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
                    ephemeral_id: ephemeral_id__.unwrap_or_default(),
                    category: category__.unwrap_or_default(),
                    entity_type: entity_type__,
                })
            }
        }
        deserializer.deserialize_struct("entity.Entity", FIELDS, GeneratedVisitor)
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
        if !self.ephemeral_id.is_empty() {
            len += 1;
        }
        if !self.entities.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("entity.EntityChain", len)?;
        if !self.ephemeral_id.is_empty() {
            struct_ser.serialize_field("ephemeralId", &self.ephemeral_id)?;
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
            "ephemeral_id",
            "ephemeralId",
            "entities",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EphemeralId,
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
                            "ephemeralId" | "ephemeral_id" => Ok(GeneratedField::EphemeralId),
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
                formatter.write_str("struct entity.EntityChain")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EntityChain, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ephemeral_id__ = None;
                let mut entities__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EphemeralId => {
                            if ephemeral_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ephemeralId"));
                            }
                            ephemeral_id__ = Some(map_.next_value()?);
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
                    ephemeral_id: ephemeral_id__.unwrap_or_default(),
                    entities: entities__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("entity.EntityChain", FIELDS, GeneratedVisitor)
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
        if !self.ephemeral_id.is_empty() {
            len += 1;
        }
        if !self.jwt.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("entity.Token", len)?;
        if !self.ephemeral_id.is_empty() {
            struct_ser.serialize_field("ephemeralId", &self.ephemeral_id)?;
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
            "ephemeral_id",
            "ephemeralId",
            "jwt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EphemeralId,
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
                            "ephemeralId" | "ephemeral_id" => Ok(GeneratedField::EphemeralId),
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
                formatter.write_str("struct entity.Token")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Token, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ephemeral_id__ = None;
                let mut jwt__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EphemeralId => {
                            if ephemeral_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ephemeralId"));
                            }
                            ephemeral_id__ = Some(map_.next_value()?);
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
                    ephemeral_id: ephemeral_id__.unwrap_or_default(),
                    jwt: jwt__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("entity.Token", FIELDS, GeneratedVisitor)
    }
}
