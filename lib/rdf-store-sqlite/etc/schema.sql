-- This is free and unencumbered software released into the public domain.

DROP TABLE IF EXISTS "rdf_quad";
DROP TABLE IF EXISTS "rdf_triple_str";
DROP TABLE IF EXISTS "rdf_triple_num";
DROP TABLE IF EXISTS "rdf_triple";
DROP TABLE IF EXISTS "rdf_node";
DROP TABLE IF EXISTS "rdf_config";

CREATE TABLE IF NOT EXISTS "rdf_config" (
    "key" text NOT NULL,
    "val" numeric NOT NULL,
    PRIMARY KEY ("key"),
    CHECK (length("key") > 0)
);

INSERT INTO "rdf_config" ("key", "val") VALUES ('schema', 1);
INSERT INTO "rdf_config" ("key", "val") VALUES ('epoch', 1767225600); -- 2026-01-01T00:00:00Z

CREATE TABLE IF NOT EXISTS "rdf_node" (
    "id" integer NOT NULL,
    "kind" integer NOT NULL,
    "val" text NOT NULL,
    PRIMARY KEY ("id" AUTOINCREMENT),
    UNIQUE ("kind", "val"),
    CHECK ("kind" IN (0, 1)),
    CHECK (length("val") > 0)
);

CREATE TABLE IF NOT EXISTS "rdf_triple" (
    "s" integer NOT NULL REFERENCES "rdf_node"("id"),
    "p" integer NOT NULL REFERENCES "rdf_node"("id"),
    "o" integer NOT NULL REFERENCES "rdf_node"("id"),
    PRIMARY KEY ("s", "p", "o")
); -- WITHOUT ROWID;

CREATE TABLE IF NOT EXISTS "rdf_triple_num" (
    "s" integer NOT NULL REFERENCES "rdf_node"("id"),
    "p" integer NOT NULL REFERENCES "rdf_node"("id"),
    "o_dt" integer NOT NULL DEFAULT 0,
    "o_val" numeric NOT NULL,
    PRIMARY KEY ("s", "p", "o_dt", "o_val")
); -- WITHOUT ROWID;

CREATE TABLE IF NOT EXISTS "rdf_triple_str" (
    "s" integer NOT NULL REFERENCES "rdf_node"("id"),
    "p" integer NOT NULL REFERENCES "rdf_node"("id"),
    "o_dt" integer NOT NULL DEFAULT 0,
    "o_lang" text NOT NULL DEFAULT '',
    "o_val" text NOT NULL,
    PRIMARY KEY ("s", "p", "o_dt", "o_lang", "o_val")
); -- WITHOUT ROWID;

CREATE TABLE IF NOT EXISTS "rdf_quad" (
    "g" integer NOT NULL REFERENCES "rdf_node"("id"),
    "s" integer NOT NULL REFERENCES "rdf_node"("id"),
    "p" integer NOT NULL REFERENCES "rdf_node"("id"),
    "o" integer NOT NULL REFERENCES "rdf_node"("id"),
    PRIMARY KEY ("g", "s", "p", "o")
); -- WITHOUT ROWID;
