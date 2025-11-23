use crate::application::auth::dtos::PermissionNode;
use crate::domain::shared::DomainResult;

/// RoleService: currently provides a static permission tree. In future this should fetch from DB.
pub struct RoleService {}

impl RoleService {
    pub fn new() -> Self {
        Self {}
    }

    /// Return a static permission tree. If role_id is Some(id) we also return list of permission ids
    /// assigned to that role. The permission tree is expected to come from a DB-backed
    /// source (e.g. `tihc_menu_tree`) and should not be hard-coded here. This method
    /// therefore only returns assigned permission ids (tree is returned empty).
    pub async fn get_permission_tree(
        &self,
        role_id: Option<i64>,
    ) -> DomainResult<(Vec<PermissionNode>, Vec<i64>)> {
        // Tree should be provided from DB (controller reads `tihc_menu_tree`).
        let tree: Vec<PermissionNode> = Vec::new();

        // Simulate assigned permission ids for now. In future this should query
        // a role_permission mapping table in the DB.
        let assigned = match role_id {
            Some(rid) if rid == 1 => vec![1, 21, 22, 31, 32, 3, 4], // super role has everything
            Some(rid) if rid == 2 => vec![1, 21, 31],               // limited role
            _ => vec![],
        };

        Ok((tree, assigned))
    }
}
