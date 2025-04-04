#![allow(unused)]
// TODO: Use it to validate perms

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Permission {
    Inputs,
    Outputs,
    Config,
    AllConfig,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PermissionRequirement {
    And(Vec<PermissionRequirement>),
    Or(Vec<PermissionRequirement>),
    Permission(Permission),
}
impl From<Permission> for PermissionRequirement {
    fn from(value: Permission) -> Self {
        Self::Permission(value)
    }
}

pub trait ValidatePermissions {
    fn require_permissions(&self, perms: &[PermissionRequirement], and_by_default: bool) -> bool;
}

impl ValidatePermissions for &[Permission] {
    fn require_permissions(&self, perms: &[PermissionRequirement], and_by_default: bool) -> bool {
        let any_req = perms.iter().any(|p| {
            let req = match p {
                PermissionRequirement::And(reqs) => self.require_permissions(reqs, true),
                PermissionRequirement::Or(reqs) => self.require_permissions(reqs, false),
                PermissionRequirement::Permission(perm) => self.contains(perm),
            };
            if and_by_default {
                !req
            } else {
                req
            }
        });
        if and_by_default {
            any_req
        } else {
            !any_req
        }
    }
}
