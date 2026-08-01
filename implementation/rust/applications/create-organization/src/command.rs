//! The `CreateOrganizationCommand` represents the intent to create a new Organization.
//!
//! Commands are immutable data structures that capture user intent. They are
//! validated by the Application Layer before being passed to the Domain Layer.

/// Command to create a new Organization.
///
/// Encapsulates all input required to create an Organization aggregate.
/// The command is validated by the application service before domain execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateOrganizationCommand {
    /// The desired name for the Organization.
    pub name: String,

    /// The type classification for the Organization.
    pub organization_type: String,
}

impl CreateOrganizationCommand {
    /// Creates a new CreateOrganizationCommand with the provided name and type.
    pub fn new(name: impl Into<String>, organization_type: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            organization_type: organization_type.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_captures_name_and_type() {
        let command = CreateOrganizationCommand::new("ForgeOS", "foundation");

        assert_eq!(command.name, "ForgeOS");
        assert_eq!(command.organization_type, "foundation");
    }

    #[test]
    fn command_converts_into_string() {
        let name = String::from("ForgeOS");
        let org_type = String::from("foundation");
        let command = CreateOrganizationCommand::new(name, org_type);

        assert_eq!(command.name, "ForgeOS");
        assert_eq!(command.organization_type, "foundation");
    }
}