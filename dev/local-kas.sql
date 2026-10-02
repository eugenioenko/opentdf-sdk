-- Local development fixture routing only. Preserve attribute/subject authorization fixtures.
-- Delete external fixture keys/grants; one local KAS becomes the default and explicit CLI destination.
BEGIN;
DELETE FROM opentdf_policy.key_access_server_keys;
DELETE FROM opentdf_policy.key_access_servers;
INSERT INTO opentdf_policy.key_access_servers (id, uri, name, public_key)
VALUES ('34f2acdc-3d9c-4e92-80b6-90fe4dc9afcb', 'http://localhost:8080/kas', 'tdf-sdk-local',
        '{"remote":"http://localhost:8080/kas.AccessService/PublicKey"}'::jsonb);
COMMIT;
