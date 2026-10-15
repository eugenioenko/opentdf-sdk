// @generated
impl serde::Serialize for CreateDynamicValueMappingRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.attribute_definition_id.is_empty() {
            len += 1;
        }
        if !self.attribute_definition_fqn.is_empty() {
            len += 1;
        }
        if self.value_resolver.is_some() {
            len += 1;
        }
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
        let mut struct_ser = serializer.serialize_struct("policy.dynamicvaluemapping.CreateDynamicValueMappingRequest", len)?;
        if !self.attribute_definition_id.is_empty() {
            struct_ser.serialize_field("attributeDefinitionId", &self.attribute_definition_id)?;
        }
        if !self.attribute_definition_fqn.is_empty() {
            struct_ser.serialize_field("attributeDefinitionFqn", &self.attribute_definition_fqn)?;
        }
        if let Some(v) = self.value_resolver.as_ref() {
            struct_ser.serialize_field("valueResolver", v)?;
        }
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
impl<'de> serde::Deserialize<'de> for CreateDynamicValueMappingRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attribute_definition_id",
            "attributeDefinitionId",
            "attribute_definition_fqn",
            "attributeDefinitionFqn",
            "value_resolver",
            "valueResolver",
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
            AttributeDefinitionId,
            AttributeDefinitionFqn,
            ValueResolver,
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
                            "attributeDefinitionId" | "attribute_definition_id" => Ok(GeneratedField::AttributeDefinitionId),
                            "attributeDefinitionFqn" | "attribute_definition_fqn" => Ok(GeneratedField::AttributeDefinitionFqn),
                            "valueResolver" | "value_resolver" => Ok(GeneratedField::ValueResolver),
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
            type Value = CreateDynamicValueMappingRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.dynamicvaluemapping.CreateDynamicValueMappingRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateDynamicValueMappingRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attribute_definition_id__ = None;
                let mut attribute_definition_fqn__ = None;
                let mut value_resolver__ = None;
                let mut actions__ = None;
                let mut existing_subject_condition_set_id__ = None;
                let mut new_subject_condition_set__ = None;
                let mut namespace_id__ = None;
                let mut namespace_fqn__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AttributeDefinitionId => {
                            if attribute_definition_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeDefinitionId"));
                            }
                            attribute_definition_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AttributeDefinitionFqn => {
                            if attribute_definition_fqn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeDefinitionFqn"));
                            }
                            attribute_definition_fqn__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValueResolver => {
                            if value_resolver__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueResolver"));
                            }
                            value_resolver__ = map_.next_value()?;
                        }
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
                Ok(CreateDynamicValueMappingRequest {
                    attribute_definition_id: attribute_definition_id__.unwrap_or_default(),
                    attribute_definition_fqn: attribute_definition_fqn__.unwrap_or_default(),
                    value_resolver: value_resolver__,
                    actions: actions__.unwrap_or_default(),
                    existing_subject_condition_set_id: existing_subject_condition_set_id__.unwrap_or_default(),
                    new_subject_condition_set: new_subject_condition_set__,
                    namespace_id: namespace_id__.unwrap_or_default(),
                    namespace_fqn: namespace_fqn__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("policy.dynamicvaluemapping.CreateDynamicValueMappingRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateDynamicValueMappingResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.dynamic_value_mapping.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.dynamicvaluemapping.CreateDynamicValueMappingResponse", len)?;
        if let Some(v) = self.dynamic_value_mapping.as_ref() {
            struct_ser.serialize_field("dynamicValueMapping", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateDynamicValueMappingResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dynamic_value_mapping",
            "dynamicValueMapping",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DynamicValueMapping,
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
                            "dynamicValueMapping" | "dynamic_value_mapping" => Ok(GeneratedField::DynamicValueMapping),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateDynamicValueMappingResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.dynamicvaluemapping.CreateDynamicValueMappingResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateDynamicValueMappingResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut dynamic_value_mapping__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DynamicValueMapping => {
                            if dynamic_value_mapping__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicValueMapping"));
                            }
                            dynamic_value_mapping__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(CreateDynamicValueMappingResponse {
                    dynamic_value_mapping: dynamic_value_mapping__,
                })
            }
        }
        deserializer.deserialize_struct("policy.dynamicvaluemapping.CreateDynamicValueMappingResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteDynamicValueMappingRequest {
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
        let mut struct_ser = serializer.serialize_struct("policy.dynamicvaluemapping.DeleteDynamicValueMappingRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteDynamicValueMappingRequest {
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
            type Value = DeleteDynamicValueMappingRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.dynamicvaluemapping.DeleteDynamicValueMappingRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteDynamicValueMappingRequest, V::Error>
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
                Ok(DeleteDynamicValueMappingRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.dynamicvaluemapping.DeleteDynamicValueMappingRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteDynamicValueMappingResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.dynamic_value_mapping.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.dynamicvaluemapping.DeleteDynamicValueMappingResponse", len)?;
        if let Some(v) = self.dynamic_value_mapping.as_ref() {
            struct_ser.serialize_field("dynamicValueMapping", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteDynamicValueMappingResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dynamic_value_mapping",
            "dynamicValueMapping",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DynamicValueMapping,
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
                            "dynamicValueMapping" | "dynamic_value_mapping" => Ok(GeneratedField::DynamicValueMapping),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteDynamicValueMappingResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.dynamicvaluemapping.DeleteDynamicValueMappingResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteDynamicValueMappingResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut dynamic_value_mapping__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DynamicValueMapping => {
                            if dynamic_value_mapping__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicValueMapping"));
                            }
                            dynamic_value_mapping__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DeleteDynamicValueMappingResponse {
                    dynamic_value_mapping: dynamic_value_mapping__,
                })
            }
        }
        deserializer.deserialize_struct("policy.dynamicvaluemapping.DeleteDynamicValueMappingResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DynamicValueMappingsSort {
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
        let mut struct_ser = serializer.serialize_struct("policy.dynamicvaluemapping.DynamicValueMappingsSort", len)?;
        if self.field != 0 {
            let v = SortDynamicValueMappingsType::try_from(self.field)
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
impl<'de> serde::Deserialize<'de> for DynamicValueMappingsSort {
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
            type Value = DynamicValueMappingsSort;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.dynamicvaluemapping.DynamicValueMappingsSort")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DynamicValueMappingsSort, V::Error>
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
                            field__ = Some(map_.next_value::<SortDynamicValueMappingsType>()? as i32);
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
                Ok(DynamicValueMappingsSort {
                    field: field__.unwrap_or_default(),
                    direction: direction__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.dynamicvaluemapping.DynamicValueMappingsSort", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDynamicValueMappingRequest {
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
        let mut struct_ser = serializer.serialize_struct("policy.dynamicvaluemapping.GetDynamicValueMappingRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDynamicValueMappingRequest {
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
            type Value = GetDynamicValueMappingRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.dynamicvaluemapping.GetDynamicValueMappingRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetDynamicValueMappingRequest, V::Error>
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
                Ok(GetDynamicValueMappingRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.dynamicvaluemapping.GetDynamicValueMappingRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDynamicValueMappingResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.dynamic_value_mapping.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.dynamicvaluemapping.GetDynamicValueMappingResponse", len)?;
        if let Some(v) = self.dynamic_value_mapping.as_ref() {
            struct_ser.serialize_field("dynamicValueMapping", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDynamicValueMappingResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dynamic_value_mapping",
            "dynamicValueMapping",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DynamicValueMapping,
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
                            "dynamicValueMapping" | "dynamic_value_mapping" => Ok(GeneratedField::DynamicValueMapping),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDynamicValueMappingResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.dynamicvaluemapping.GetDynamicValueMappingResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetDynamicValueMappingResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut dynamic_value_mapping__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DynamicValueMapping => {
                            if dynamic_value_mapping__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicValueMapping"));
                            }
                            dynamic_value_mapping__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GetDynamicValueMappingResponse {
                    dynamic_value_mapping: dynamic_value_mapping__,
                })
            }
        }
        deserializer.deserialize_struct("policy.dynamicvaluemapping.GetDynamicValueMappingResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListDynamicValueMappingsRequest {
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
        if !self.namespace_fqn.is_empty() {
            len += 1;
        }
        if !self.attribute_definition_id.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        if !self.sort.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.dynamicvaluemapping.ListDynamicValueMappingsRequest", len)?;
        if !self.namespace_id.is_empty() {
            struct_ser.serialize_field("namespaceId", &self.namespace_id)?;
        }
        if !self.namespace_fqn.is_empty() {
            struct_ser.serialize_field("namespaceFqn", &self.namespace_fqn)?;
        }
        if !self.attribute_definition_id.is_empty() {
            struct_ser.serialize_field("attributeDefinitionId", &self.attribute_definition_id)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        if !self.sort.is_empty() {
            struct_ser.serialize_field("sort", &self.sort)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListDynamicValueMappingsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "namespace_id",
            "namespaceId",
            "namespace_fqn",
            "namespaceFqn",
            "attribute_definition_id",
            "attributeDefinitionId",
            "pagination",
            "sort",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NamespaceId,
            NamespaceFqn,
            AttributeDefinitionId,
            Pagination,
            Sort,
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
                            "namespaceFqn" | "namespace_fqn" => Ok(GeneratedField::NamespaceFqn),
                            "attributeDefinitionId" | "attribute_definition_id" => Ok(GeneratedField::AttributeDefinitionId),
                            "pagination" => Ok(GeneratedField::Pagination),
                            "sort" => Ok(GeneratedField::Sort),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListDynamicValueMappingsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.dynamicvaluemapping.ListDynamicValueMappingsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListDynamicValueMappingsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut namespace_id__ = None;
                let mut namespace_fqn__ = None;
                let mut attribute_definition_id__ = None;
                let mut pagination__ = None;
                let mut sort__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                        GeneratedField::AttributeDefinitionId => {
                            if attribute_definition_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributeDefinitionId"));
                            }
                            attribute_definition_id__ = Some(map_.next_value()?);
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
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ListDynamicValueMappingsRequest {
                    namespace_id: namespace_id__.unwrap_or_default(),
                    namespace_fqn: namespace_fqn__.unwrap_or_default(),
                    attribute_definition_id: attribute_definition_id__.unwrap_or_default(),
                    pagination: pagination__,
                    sort: sort__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.dynamicvaluemapping.ListDynamicValueMappingsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListDynamicValueMappingsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.dynamic_value_mappings.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.dynamicvaluemapping.ListDynamicValueMappingsResponse", len)?;
        if !self.dynamic_value_mappings.is_empty() {
            struct_ser.serialize_field("dynamicValueMappings", &self.dynamic_value_mappings)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListDynamicValueMappingsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dynamic_value_mappings",
            "dynamicValueMappings",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DynamicValueMappings,
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
                            "dynamicValueMappings" | "dynamic_value_mappings" => Ok(GeneratedField::DynamicValueMappings),
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
            type Value = ListDynamicValueMappingsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.dynamicvaluemapping.ListDynamicValueMappingsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListDynamicValueMappingsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut dynamic_value_mappings__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DynamicValueMappings => {
                            if dynamic_value_mappings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicValueMappings"));
                            }
                            dynamic_value_mappings__ = Some(map_.next_value()?);
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
                Ok(ListDynamicValueMappingsResponse {
                    dynamic_value_mappings: dynamic_value_mappings__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("policy.dynamicvaluemapping.ListDynamicValueMappingsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SortDynamicValueMappingsType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SORT_DYNAMIC_VALUE_MAPPINGS_TYPE_UNSPECIFIED",
            Self::CreatedAt => "SORT_DYNAMIC_VALUE_MAPPINGS_TYPE_CREATED_AT",
            Self::UpdatedAt => "SORT_DYNAMIC_VALUE_MAPPINGS_TYPE_UPDATED_AT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SortDynamicValueMappingsType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SORT_DYNAMIC_VALUE_MAPPINGS_TYPE_UNSPECIFIED",
            "SORT_DYNAMIC_VALUE_MAPPINGS_TYPE_CREATED_AT",
            "SORT_DYNAMIC_VALUE_MAPPINGS_TYPE_UPDATED_AT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SortDynamicValueMappingsType;

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
                    "SORT_DYNAMIC_VALUE_MAPPINGS_TYPE_UNSPECIFIED" => Ok(SortDynamicValueMappingsType::Unspecified),
                    "SORT_DYNAMIC_VALUE_MAPPINGS_TYPE_CREATED_AT" => Ok(SortDynamicValueMappingsType::CreatedAt),
                    "SORT_DYNAMIC_VALUE_MAPPINGS_TYPE_UPDATED_AT" => Ok(SortDynamicValueMappingsType::UpdatedAt),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateDynamicValueMappingRequest {
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
        if self.value_resolver.is_some() {
            len += 1;
        }
        if !self.subject_condition_set_id.is_empty() {
            len += 1;
        }
        if !self.actions.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        if self.metadata_update_behavior != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.dynamicvaluemapping.UpdateDynamicValueMappingRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.value_resolver.as_ref() {
            struct_ser.serialize_field("valueResolver", v)?;
        }
        if !self.subject_condition_set_id.is_empty() {
            struct_ser.serialize_field("subjectConditionSetId", &self.subject_condition_set_id)?;
        }
        if !self.actions.is_empty() {
            struct_ser.serialize_field("actions", &self.actions)?;
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
impl<'de> serde::Deserialize<'de> for UpdateDynamicValueMappingRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "value_resolver",
            "valueResolver",
            "subject_condition_set_id",
            "subjectConditionSetId",
            "actions",
            "metadata",
            "metadata_update_behavior",
            "metadataUpdateBehavior",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ValueResolver,
            SubjectConditionSetId,
            Actions,
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
                            "valueResolver" | "value_resolver" => Ok(GeneratedField::ValueResolver),
                            "subjectConditionSetId" | "subject_condition_set_id" => Ok(GeneratedField::SubjectConditionSetId),
                            "actions" => Ok(GeneratedField::Actions),
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
            type Value = UpdateDynamicValueMappingRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.dynamicvaluemapping.UpdateDynamicValueMappingRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateDynamicValueMappingRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut value_resolver__ = None;
                let mut subject_condition_set_id__ = None;
                let mut actions__ = None;
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
                        GeneratedField::ValueResolver => {
                            if value_resolver__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueResolver"));
                            }
                            value_resolver__ = map_.next_value()?;
                        }
                        GeneratedField::SubjectConditionSetId => {
                            if subject_condition_set_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectConditionSetId"));
                            }
                            subject_condition_set_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Actions => {
                            if actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            actions__ = Some(map_.next_value()?);
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
                Ok(UpdateDynamicValueMappingRequest {
                    id: id__.unwrap_or_default(),
                    value_resolver: value_resolver__,
                    subject_condition_set_id: subject_condition_set_id__.unwrap_or_default(),
                    actions: actions__.unwrap_or_default(),
                    metadata: metadata__,
                    metadata_update_behavior: metadata_update_behavior__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("policy.dynamicvaluemapping.UpdateDynamicValueMappingRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateDynamicValueMappingResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.dynamic_value_mapping.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("policy.dynamicvaluemapping.UpdateDynamicValueMappingResponse", len)?;
        if let Some(v) = self.dynamic_value_mapping.as_ref() {
            struct_ser.serialize_field("dynamicValueMapping", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateDynamicValueMappingResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dynamic_value_mapping",
            "dynamicValueMapping",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DynamicValueMapping,
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
                            "dynamicValueMapping" | "dynamic_value_mapping" => Ok(GeneratedField::DynamicValueMapping),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateDynamicValueMappingResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct policy.dynamicvaluemapping.UpdateDynamicValueMappingResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateDynamicValueMappingResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut dynamic_value_mapping__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DynamicValueMapping => {
                            if dynamic_value_mapping__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicValueMapping"));
                            }
                            dynamic_value_mapping__ = map_.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(UpdateDynamicValueMappingResponse {
                    dynamic_value_mapping: dynamic_value_mapping__,
                })
            }
        }
        deserializer.deserialize_struct("policy.dynamicvaluemapping.UpdateDynamicValueMappingResponse", FIELDS, GeneratedVisitor)
    }
}
