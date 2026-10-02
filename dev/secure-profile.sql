-- Remove only this profile's base key, grants, and keys. Basic routing/authorization remain.
BEGIN;
DELETE FROM opentdf_policy.base_keys WHERE key_access_server_key_id IN
  (SELECT id FROM opentdf_policy.key_access_server_keys WHERE key_id IN ('profile-r1', 'profile-e1'));
DELETE FROM opentdf_policy.attribute_value_public_key_map WHERE key_access_server_key_id IN
  (SELECT id FROM opentdf_policy.key_access_server_keys WHERE key_id IN ('profile-r1', 'profile-e1'));
DELETE FROM opentdf_policy.key_access_server_keys WHERE key_id IN ('profile-r1', 'profile-e1');
COMMIT;
