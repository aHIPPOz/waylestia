//! Module Security — permission management, sandboxing, audit logging

use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum Permission {
    Filesystem,
    Network,
    Camera,
    Microphone,
    Notifications,
    Printer,
    USB,
    GPU,
}

#[derive(Debug, Clone)]
pub struct AppSandbox {
    pub app_id: String,
    pub permissions: HashSet<Permission>,
    pub denied_permissions: HashSet<Permission>,
    pub allowed_paths: Vec<String>,
    pub denied_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub timestamp: DateTime<Utc>,
    pub app_id: String,
    pub event_type: AuditEventType,
    pub resource: String,
    pub allowed: bool,
    pub message: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AuditEventType {
    PermissionRequest,
    FileAccess,
    NetworkAccess,
    ProcessCreation,
    SecurityViolation,
    SandboxEscape,
}

pub struct SecurityManager {
    sandboxes: HashMap<String, AppSandbox>,
    audit_log: Vec<AuditEvent>,
}

impl SecurityManager {
    pub fn new() -> Self {
        Self {
            sandboxes: HashMap::new(),
            audit_log: Vec::new(),
        }
    }

    /// Register an app with its sandbox configuration
    pub fn register_sandbox(&mut self, sandbox: AppSandbox) {
        self.sandboxes.insert(sandbox.app_id.clone(), sandbox);
    }

    /// Check if an app has permission
    pub fn check_permission(&mut self, app_id: &str, permission: Permission) -> bool {
        if let Some(sandbox) = self.sandboxes.get(app_id) {
            let allowed = sandbox.permissions.contains(&permission);
            self.audit_log(
                app_id.to_string(),
                AuditEventType::PermissionRequest,
                format!("{:?}", permission),
                allowed,
            );
            allowed
        } else {
            // Unknown app - deny by default
            self.audit_log(
                app_id.to_string(),
                AuditEventType::SecurityViolation,
                "Unknown app".to_string(),
                false,
            );
            false
        }
    }

    /// Check file access
    pub fn check_file_access(&mut self, app_id: &str, path: &str, write: bool) -> bool {
        if let Some(sandbox) = self.sandboxes.get(app_id) {
            // Check allowed paths
            let is_allowed = sandbox.allowed_paths.iter().any(|p| path.starts_with(p));
            
            // Check denied paths
            let is_denied = sandbox.denied_paths.iter().any(|p| path.starts_with(p));
            
            let allowed = is_allowed && !is_denied && sandbox.permissions.contains(&Permission::Filesystem);
            
            self.audit_log(
                app_id.to_string(),
                AuditEventType::FileAccess,
                format!("{} (write: {})", path, write),
                allowed,
            );
            allowed
        } else {
            false
        }
    }

    /// Check network access
    pub fn check_network_access(&mut self, app_id: &str, url: &str) -> bool {
        if let Some(sandbox) = self.sandboxes.get(app_id) {
            let allowed = sandbox.permissions.contains(&Permission::Network);
            self.audit_log(
                app_id.to_string(),
                AuditEventType::NetworkAccess,
                url.to_string(),
                allowed,
            );
            allowed
        } else {
            false
        }
    }

    /// Log an audit event
    fn audit_log(
        &mut self,
        app_id: String,
        event_type: AuditEventType,
        resource: String,
        allowed: bool,
    ) {
        let event = AuditEvent {
            timestamp: Utc::now(),
            app_id,
            event_type,
            resource,
            allowed,
            message: format!("Event: {:?}, Allowed: {}", event_type, allowed),
        };
        self.audit_log.push(event);
    }

    /// Get audit log
    pub fn get_audit_log(&self) -> Vec<AuditEvent> {
        self.audit_log.clone()
    }

    /// Get audit log for specific app
    pub fn get_app_audit_log(&self, app_id: &str) -> Vec<AuditEvent> {
        self.audit_log
            .iter()
            .filter(|e| e.app_id == app_id)
            .cloned()
            .collect()
    }

    /// Clear old audit logs (older than N days)
    pub fn cleanup_old_logs(&mut self, days: i64) {
        let cutoff = Utc::now() - chrono::Duration::days(days);
        self.audit_log.retain(|e| e.timestamp > cutoff);
    }

    /// Get sandbox configuration
    pub fn get_sandbox(&self, app_id: &str) -> Option<AppSandbox> {
        self.sandboxes.get(app_id).cloned()
    }

    /// Revoke a permission
    pub fn revoke_permission(&mut self, app_id: &str, permission: Permission) {
        if let Some(sandbox) = self.sandboxes.get_mut(app_id) {
            sandbox.permissions.remove(&permission);
        }
    }

    /// Grant a permission
    pub fn grant_permission(&mut self, app_id: &str, permission: Permission) {
        if let Some(sandbox) = self.sandboxes.get_mut(app_id) {
            sandbox.permissions.insert(permission);
        }
    }
}

impl Default for SecurityManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_manager() {
        let mut manager = SecurityManager::new();
        
        let mut sandbox = AppSandbox {
            app_id: "test-app".to_string(),
            permissions: HashSet::new(),
            denied_permissions: HashSet::new(),
            allowed_paths: vec!["/tmp".to_string()],
            denied_paths: vec![],
        };
        sandbox.permissions.insert(Permission::Filesystem);
        
        manager.register_sandbox(sandbox);
        
        assert!(manager.check_permission("test-app", Permission::Filesystem));
        assert!(!manager.check_permission("test-app", Permission::Network));
    }

    #[test]
    fn test_audit_logging() {
        let mut manager = SecurityManager::new();
        
        let mut sandbox = AppSandbox {
            app_id: "test-app".to_string(),
            permissions: HashSet::new(),
            denied_permissions: HashSet::new(),
            allowed_paths: vec![],
            denied_paths: vec![],
        };
        sandbox.permissions.insert(Permission::Filesystem);
        manager.register_sandbox(sandbox);
        
        let _ = manager.check_permission("test-app", Permission::Network);
        let log = manager.get_app_audit_log("test-app");
        assert!(!log.is_empty());
    }
}
