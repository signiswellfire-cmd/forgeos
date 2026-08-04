//! Presentation composition root (ISP-0007, ARCH-0003).
//!
//! The composition root wires the Presentation Layer for the Create
//! Organization vertical slice: it constructs the Create Organization view
//! model. The IPC dispatch contract for the `createOrganization` command is
//! provided by the `ipc` module and the (deferred) frontend framework binding
//! per `TDR-0002` supplies the concrete IPC transport.

use crate::view_model::CreateOrganizationViewModel;

/// Presentation composition root (ISP-0007).
///
/// Constructs the Create Organization view model. No runtime state is held;
/// the IPC transport binding remains the responsibility of the deferred
/// frontend framework (TDR-0002).
#[derive(Debug, Clone, Default)]
pub struct PresentationCompositionRoot;

impl PresentationCompositionRoot {
    /// Constructs the presentation composition root (ISP-0007).
    pub fn new() -> Self {
        Self
    }

    /// Creates a fresh Create Organization view model (ARCH-0002, TDR-0002).
    pub fn create_view_model(&self) -> CreateOrganizationViewModel {
        CreateOrganizationViewModel::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composition_root_creates_idle_view_model() {
        let root = PresentationCompositionRoot::new();
        let vm = root.create_view_model();

        assert_eq!(vm.name(), "");
        assert_eq!(vm.organization_type(), "");
    }
}