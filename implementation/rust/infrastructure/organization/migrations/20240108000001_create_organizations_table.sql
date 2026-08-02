-- Create Organization Infrastructure Layer Migration
-- Milestone 1.7 - Organization Infrastructure Layer
-- Creates the organizations table with singleton enforcement

-- Create the organizations table
CREATE TABLE IF NOT EXISTS organizations (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    organization_type TEXT NOT NULL,
    status TEXT NOT NULL,
    version INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Create index on id for fast lookups
CREATE INDEX IF NOT EXISTS idx_organizations_id ON organizations(id);

-- Enforce singleton constraint: only one organization can exist
-- This trigger prevents insertion if an organization already exists
CREATE TRIGGER IF NOT EXISTS enforce_singleton_organization
BEFORE INSERT ON organizations
WHEN (SELECT COUNT(*) FROM organizations) >= 1
BEGIN
    SELECT RAISE(ABORT, 'organization already exists - singleton constraint violation');
END;