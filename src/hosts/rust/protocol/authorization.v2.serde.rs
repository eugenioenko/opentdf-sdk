// @generated
impl serde::Serialize for Decision {
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
impl<'de> serde::Deserialize<'de> for Decision {
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
            type Value = Decision;

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
                    "DECISION_UNSPECIFIED" => Ok(Decision::Unspecified),
                    "DECISION_DENY" => Ok(Decision::Deny),
                    "DECISION_PERMIT" => Ok(Decision::Permit),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
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
        if !self.ephemeral_id.is_empty() {
            len += 1;
        }
        if !self.actions_per_attribute_value_fqn.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authorization.v2.EntityEntitlements", len)?;
        if !self.ephemeral_id.is_empty() {
            struct_ser.serialize_field("ephemeralId", &self.ephemeral_id)?;
        }
        if !self.actions_per_attribute_value_fqn.is_empty() {
            struct_ser.serialize_field("actionsPerAttributeValueFqn", &self.actions_per_attribute_value_fqn)?;
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
            "ephemeral_id",
            "ephemeralId",
            "actions_per_attribute_value_fqn",
            "actionsPerAttributeValueFqn",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EphemeralId,
            ActionsPerAttributeValueFqn,
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
                            "actionsPerAttributeValueFqn" | "actions_per_attribute_value_fqn" => Ok(GeneratedField::ActionsPerAttributeValueFqn),
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
                formatter.write_str("struct authorization.v2.EntityEntitlements")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EntityEntitlements, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ephemeral_id__ = None;
                let mut actions_per_attribute_value_fqn__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EphemeralId => {
                            if ephemeral_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ephemeralId"));
                            }
                            ephemeral_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ActionsPerAttributeValueFqn => {
                            if actions_per_attribute_value_fqn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionsPerAttributeValueFqn"));
                            }
                            actions_per_attribute_value_fqn__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(EntityEntitlements {
                    ephemeral_id: ephemeral_id__.unwrap_or_default(),
                    actions_per_attribute_value_fqn: actions_per_attribute_value_fqn__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authorization.v2.EntityEntitlements", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for entity_entitlements::ActionsList {
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
        let mut struct_ser = serializer.serialize_struct("authorization.v2.EntityEntitlements.ActionsList", len)?;
        if !self.actions.is_empty() {
            struct_ser.serialize_field("actions", &self.actions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for entity_entitlements::ActionsList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "actions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = entity_entitlements::ActionsList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authorization.v2.EntityEntitlements.ActionsList")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<entity_entitlements::ActionsList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut actions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                Ok(entity_entitlements::ActionsList {
                    actions: actions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authorization.v2.EntityEntitlements.ActionsList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EntityIdentifier {
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
        let mut struct_ser = serializer.serialize_struct("authorization.v2.EntityIdentifier", len)?;
        if let Some(v) = self.identifier.as_ref() {
            match v {
                entity_identifier::Identifier::EntityChain(v) => {
                    struct_ser.serialize_field("entityChain", v)?;
                }
                entity_identifier::Identifier::RegisteredResourceValueFqn(v) => {
                    struct_ser.serialize_field("registeredResourceValueFqn", v)?;
                }
                entity_identifier::Identifier::Token(v) => {
                    struct_ser.serialize_field("token", v)?;
                }
                entity_identifier::Identifier::WithRequestToken(v) => {
                    struct_ser.serialize_field("withRequestToken", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EntityIdentifier {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity_chain",
            "entityChain",
            "registered_resource_value_fqn",
            "registeredResourceValueFqn",
            "token",
            "with_request_token",
            "withRequestToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EntityChain,
            RegisteredResourceValueFqn,
            Token,
            WithRequestToken,
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
                            "entityChain" | "entity_chain" => Ok(GeneratedField::EntityChain),
                            "registeredResourceValueFqn" | "registered_resource_value_fqn" => Ok(GeneratedField::RegisteredResourceValueFqn),
                            "token" => Ok(GeneratedField::Token),
                            "withRequestToken" | "with_request_token" => Ok(GeneratedField::WithRequestToken),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EntityIdentifier;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authorization.v2.EntityIdentifier")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EntityIdentifier, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut identifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EntityChain => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityChain"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_identifier::Identifier::EntityChain)
;
                        }
                        GeneratedField::RegisteredResourceValueFqn => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("registeredResourceValueFqn"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_identifier::Identifier::RegisteredResourceValueFqn);
                        }
                        GeneratedField::Token => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("token"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_identifier::Identifier::Token)
;
                        }
                        GeneratedField::WithRequestToken => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withRequestToken"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(entity_identifier::Identifier::WithRequestToken)
;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(EntityIdentifier {
                    identifier: identifier__,
                })
            }
        }
        deserializer.deserialize_struct("authorization.v2.EntityIdentifier", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDecisionBulkRequest {
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
        let mut struct_ser = serializer.serialize_struct("authorization.v2.GetDecisionBulkRequest", len)?;
        if !self.decision_requests.is_empty() {
            struct_ser.serialize_field("decisionRequests", &self.decision_requests)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDecisionBulkRequest {
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
            type Value = GetDecisionBulkRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authorization.v2.GetDecisionBulkRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetDecisionBulkRequest, V::Error>
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
                Ok(GetDecisionBulkRequest {
                    decision_requests: decision_requests__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authorization.v2.GetDecisionBulkRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDecisionBulkResponse {
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
        let mut struct_ser = serializer.serialize_struct("authorization.v2.GetDecisionBulkResponse", len)?;
        if !self.decision_responses.is_empty() {
            struct_ser.serialize_field("decisionResponses", &self.decision_responses)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDecisionBulkResponse {
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
            type Value = GetDecisionBulkResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authorization.v2.GetDecisionBulkResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetDecisionBulkResponse, V::Error>
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
                Ok(GetDecisionBulkResponse {
                    decision_responses: decision_responses__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authorization.v2.GetDecisionBulkResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDecisionMultiResourceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.entity_identifier.is_some() {
            len += 1;
        }
        if self.action.is_some() {
            len += 1;
        }
        if !self.resources.is_empty() {
            len += 1;
        }
        if !self.fulfillable_obligation_fqns.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authorization.v2.GetDecisionMultiResourceRequest", len)?;
        if let Some(v) = self.entity_identifier.as_ref() {
            struct_ser.serialize_field("entityIdentifier", v)?;
        }
        if let Some(v) = self.action.as_ref() {
            struct_ser.serialize_field("action", v)?;
        }
        if !self.resources.is_empty() {
            struct_ser.serialize_field("resources", &self.resources)?;
        }
        if !self.fulfillable_obligation_fqns.is_empty() {
            struct_ser.serialize_field("fulfillableObligationFqns", &self.fulfillable_obligation_fqns)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDecisionMultiResourceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity_identifier",
            "entityIdentifier",
            "action",
            "resources",
            "fulfillable_obligation_fqns",
            "fulfillableObligationFqns",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EntityIdentifier,
            Action,
            Resources,
            FulfillableObligationFqns,
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
                            "entityIdentifier" | "entity_identifier" => Ok(GeneratedField::EntityIdentifier),
                            "action" => Ok(GeneratedField::Action),
                            "resources" => Ok(GeneratedField::Resources),
                            "fulfillableObligationFqns" | "fulfillable_obligation_fqns" => Ok(GeneratedField::FulfillableObligationFqns),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDecisionMultiResourceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authorization.v2.GetDecisionMultiResourceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetDecisionMultiResourceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity_identifier__ = None;
                let mut action__ = None;
                let mut resources__ = None;
                let mut fulfillable_obligation_fqns__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EntityIdentifier => {
                            if entity_identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityIdentifier"));
                            }
                            entity_identifier__ = map_.next_value()?;
                        }
                        GeneratedField::Action => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            action__ = map_.next_value()?;
                        }
                        GeneratedField::Resources => {
                            if resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resources"));
                            }
                            resources__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FulfillableObligationFqns => {
                            if fulfillable_obligation_fqns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fulfillableObligationFqns"));
                            }
                            fulfillable_obligation_fqns__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetDecisionMultiResourceRequest {
                    entity_identifier: entity_identifier__,
                    action: action__,
                    resources: resources__.unwrap_or_default(),
                    fulfillable_obligation_fqns: fulfillable_obligation_fqns__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authorization.v2.GetDecisionMultiResourceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDecisionMultiResourceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.all_permitted.is_some() {
            len += 1;
        }
        if !self.resource_decisions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authorization.v2.GetDecisionMultiResourceResponse", len)?;
        if let Some(v) = self.all_permitted.as_ref() {
            struct_ser.serialize_field("allPermitted", v)?;
        }
        if !self.resource_decisions.is_empty() {
            struct_ser.serialize_field("resourceDecisions", &self.resource_decisions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDecisionMultiResourceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "all_permitted",
            "allPermitted",
            "resource_decisions",
            "resourceDecisions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllPermitted,
            ResourceDecisions,
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
                            "allPermitted" | "all_permitted" => Ok(GeneratedField::AllPermitted),
                            "resourceDecisions" | "resource_decisions" => Ok(GeneratedField::ResourceDecisions),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDecisionMultiResourceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authorization.v2.GetDecisionMultiResourceResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetDecisionMultiResourceResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut all_permitted__ = None;
                let mut resource_decisions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AllPermitted => {
                            if all_permitted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allPermitted"));
                            }
                            all_permitted__ = map_.next_value()?;
                        }
                        GeneratedField::ResourceDecisions => {
                            if resource_decisions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceDecisions"));
                            }
                            resource_decisions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetDecisionMultiResourceResponse {
                    all_permitted: all_permitted__,
                    resource_decisions: resource_decisions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authorization.v2.GetDecisionMultiResourceResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDecisionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.entity_identifier.is_some() {
            len += 1;
        }
        if self.action.is_some() {
            len += 1;
        }
        if self.resource.is_some() {
            len += 1;
        }
        if !self.fulfillable_obligation_fqns.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authorization.v2.GetDecisionRequest", len)?;
        if let Some(v) = self.entity_identifier.as_ref() {
            struct_ser.serialize_field("entityIdentifier", v)?;
        }
        if let Some(v) = self.action.as_ref() {
            struct_ser.serialize_field("action", v)?;
        }
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
        }
        if !self.fulfillable_obligation_fqns.is_empty() {
            struct_ser.serialize_field("fulfillableObligationFqns", &self.fulfillable_obligation_fqns)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDecisionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity_identifier",
            "entityIdentifier",
            "action",
            "resource",
            "fulfillable_obligation_fqns",
            "fulfillableObligationFqns",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EntityIdentifier,
            Action,
            Resource,
            FulfillableObligationFqns,
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
                            "entityIdentifier" | "entity_identifier" => Ok(GeneratedField::EntityIdentifier),
                            "action" => Ok(GeneratedField::Action),
                            "resource" => Ok(GeneratedField::Resource),
                            "fulfillableObligationFqns" | "fulfillable_obligation_fqns" => Ok(GeneratedField::FulfillableObligationFqns),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDecisionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authorization.v2.GetDecisionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetDecisionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity_identifier__ = None;
                let mut action__ = None;
                let mut resource__ = None;
                let mut fulfillable_obligation_fqns__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EntityIdentifier => {
                            if entity_identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityIdentifier"));
                            }
                            entity_identifier__ = map_.next_value()?;
                        }
                        GeneratedField::Action => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            action__ = map_.next_value()?;
                        }
                        GeneratedField::Resource => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            resource__ = map_.next_value()?;
                        }
                        GeneratedField::FulfillableObligationFqns => {
                            if fulfillable_obligation_fqns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fulfillableObligationFqns"));
                            }
                            fulfillable_obligation_fqns__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetDecisionRequest {
                    entity_identifier: entity_identifier__,
                    action: action__,
                    resource: resource__,
                    fulfillable_obligation_fqns: fulfillable_obligation_fqns__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authorization.v2.GetDecisionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDecisionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.decision.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authorization.v2.GetDecisionResponse", len)?;
        if let Some(v) = self.decision.as_ref() {
            struct_ser.serialize_field("decision", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDecisionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "decision",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Decision,
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
                            "decision" => Ok(GeneratedField::Decision),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDecisionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authorization.v2.GetDecisionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetDecisionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut decision__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Decision => {
                            if decision__.is_some() {
                                return Err(serde::de::Error::duplicate_field("decision"));
                            }
                            decision__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetDecisionResponse {
                    decision: decision__,
                })
            }
        }
        deserializer.deserialize_struct("authorization.v2.GetDecisionResponse", FIELDS, GeneratedVisitor)
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
        if self.entity_identifier.is_some() {
            len += 1;
        }
        if self.with_comprehensive_hierarchy.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authorization.v2.GetEntitlementsRequest", len)?;
        if let Some(v) = self.entity_identifier.as_ref() {
            struct_ser.serialize_field("entityIdentifier", v)?;
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
            "entity_identifier",
            "entityIdentifier",
            "with_comprehensive_hierarchy",
            "withComprehensiveHierarchy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EntityIdentifier,
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
                            "entityIdentifier" | "entity_identifier" => Ok(GeneratedField::EntityIdentifier),
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
                formatter.write_str("struct authorization.v2.GetEntitlementsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetEntitlementsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity_identifier__ = None;
                let mut with_comprehensive_hierarchy__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EntityIdentifier => {
                            if entity_identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityIdentifier"));
                            }
                            entity_identifier__ = map_.next_value()?;
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
                    entity_identifier: entity_identifier__,
                    with_comprehensive_hierarchy: with_comprehensive_hierarchy__,
                })
            }
        }
        deserializer.deserialize_struct("authorization.v2.GetEntitlementsRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("authorization.v2.GetEntitlementsResponse", len)?;
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
                formatter.write_str("struct authorization.v2.GetEntitlementsResponse")
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
        deserializer.deserialize_struct("authorization.v2.GetEntitlementsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Resource {
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
        if self.resource.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authorization.v2.Resource", len)?;
        if !self.ephemeral_id.is_empty() {
            struct_ser.serialize_field("ephemeralId", &self.ephemeral_id)?;
        }
        if let Some(v) = self.resource.as_ref() {
            match v {
                resource::Resource::AttributeValues(v) => {
                    struct_ser.serialize_field("attributeValues", v)?;
                }
                resource::Resource::RegisteredResourceValueFqn(v) => {
                    struct_ser.serialize_field("registeredResourceValueFqn", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Resource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ephemeral_id",
            "ephemeralId",
            "attribute_values",
            "attributeValues",
            "registered_resource_value_fqn",
            "registeredResourceValueFqn",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EphemeralId,
            AttributeValues,
            RegisteredResourceValueFqn,
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
                            "attributeValues" | "attribute_values" => Ok(GeneratedField::AttributeValues),
                            "registeredResourceValueFqn" | "registered_resource_value_fqn" => Ok(GeneratedField::RegisteredResourceValueFqn),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Resource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authorization.v2.Resource")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Resource, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ephemeral_id__ = None;
                let mut resource__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EphemeralId => {
                            if ephemeral_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ephemeralId"));
                            }
                            ephemeral_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AttributeValues => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeValues"));
                            }
                            resource__ = map_.next_value::<::std::option::Option<_>>()?.map(resource::Resource::AttributeValues)
;
                        }
                        GeneratedField::RegisteredResourceValueFqn => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("registeredResourceValueFqn"));
                            }
                            resource__ = map_.next_value::<::std::option::Option<_>>()?.map(resource::Resource::RegisteredResourceValueFqn);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Resource {
                    ephemeral_id: ephemeral_id__.unwrap_or_default(),
                    resource: resource__,
                })
            }
        }
        deserializer.deserialize_struct("authorization.v2.Resource", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for resource::AttributeValues {
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
        let mut struct_ser = serializer.serialize_struct("authorization.v2.Resource.AttributeValues", len)?;
        if !self.fqns.is_empty() {
            struct_ser.serialize_field("fqns", &self.fqns)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for resource::AttributeValues {
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
            type Value = resource::AttributeValues;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authorization.v2.Resource.AttributeValues")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<resource::AttributeValues, V::Error>
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
                Ok(resource::AttributeValues {
                    fqns: fqns__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authorization.v2.Resource.AttributeValues", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceDecision {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.ephemeral_resource_id.is_empty() {
            len += 1;
        }
        if self.decision != 0 {
            len += 1;
        }
        if !self.required_obligations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("authorization.v2.ResourceDecision", len)?;
        if !self.ephemeral_resource_id.is_empty() {
            struct_ser.serialize_field("ephemeralResourceId", &self.ephemeral_resource_id)?;
        }
        if self.decision != 0 {
            let v = Decision::try_from(self.decision)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.decision)))?;
            struct_ser.serialize_field("decision", &v)?;
        }
        if !self.required_obligations.is_empty() {
            struct_ser.serialize_field("requiredObligations", &self.required_obligations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceDecision {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ephemeral_resource_id",
            "ephemeralResourceId",
            "decision",
            "required_obligations",
            "requiredObligations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EphemeralResourceId,
            Decision,
            RequiredObligations,
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
                            "ephemeralResourceId" | "ephemeral_resource_id" => Ok(GeneratedField::EphemeralResourceId),
                            "decision" => Ok(GeneratedField::Decision),
                            "requiredObligations" | "required_obligations" => Ok(GeneratedField::RequiredObligations),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceDecision;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct authorization.v2.ResourceDecision")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceDecision, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ephemeral_resource_id__ = None;
                let mut decision__ = None;
                let mut required_obligations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EphemeralResourceId => {
                            if ephemeral_resource_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ephemeralResourceId"));
                            }
                            ephemeral_resource_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Decision => {
                            if decision__.is_some() {
                                return Err(serde::de::Error::duplicate_field("decision"));
                            }
                            decision__ = Some(map_.next_value::<Decision>()? as i32);
                        }
                        GeneratedField::RequiredObligations => {
                            if required_obligations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requiredObligations"));
                            }
                            required_obligations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ResourceDecision {
                    ephemeral_resource_id: ephemeral_resource_id__.unwrap_or_default(),
                    decision: decision__.unwrap_or_default(),
                    required_obligations: required_obligations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("authorization.v2.ResourceDecision", FIELDS, GeneratedVisitor)
    }
}
