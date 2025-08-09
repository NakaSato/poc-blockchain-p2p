//! Storage Infrastructure
//!
//! Provides abstraction over different storage backends.

use crate::shared::domain::errors::DomainError;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};

/// Generic storage provider trait
#[async_trait]
pub trait StorageProvider: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, DomainError>;
    async fn set(&self, key: &str, value: Vec<u8>) -> Result<(), DomainError>;
    async fn delete(&self, key: &str) -> Result<(), DomainError>;
    async fn exists(&self, key: &str) -> Result<bool, DomainError>;
    async fn list_keys(&self, prefix: &str) -> Result<Vec<String>, DomainError>;
    async fn clear(&self) -> Result<(), DomainError>;
}

/// In-memory storage implementation
pub struct InMemoryStorage {
    data: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl StorageProvider for InMemoryStorage {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, DomainError> {
        let data = self.data.read().await;
        Ok(data.get(key).cloned())
    }

    async fn set(&self, key: &str, value: Vec<u8>) -> Result<(), DomainError> {
        let mut data = self.data.write().await;
        data.insert(key.to_string(), value);
        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<(), DomainError> {
        let mut data = self.data.write().await;
        data.remove(key);
        Ok(())
    }

    async fn exists(&self, key: &str) -> Result<bool, DomainError> {
        let data = self.data.read().await;
        Ok(data.contains_key(key))
    }

    async fn list_keys(&self, prefix: &str) -> Result<Vec<String>, DomainError> {
        let data = self.data.read().await;
        let keys: Vec<String> = data.keys()
            .filter(|k| k.starts_with(prefix))
            .cloned()
            .collect();
        Ok(keys)
    }

    async fn clear(&self) -> Result<(), DomainError> {
        let mut data = self.data.write().await;
        data.clear();
        Ok(())
    }
}

/// File system storage implementation
pub struct FileSystemStorage {
    base_path: std::path::PathBuf,
}

impl FileSystemStorage {
    pub fn new(base_path: impl Into<std::path::PathBuf>) -> Result<Self, DomainError> {
        let base_path = base_path.into();
        
        // Create base directory if it doesn't exist
        std::fs::create_dir_all(&base_path)
            .map_err(|e| DomainError::invalid_operation(format!("Failed to create storage directory: {}", e)))?;
        
        Ok(Self { base_path })
    }
    
    fn key_to_path(&self, key: &str) -> std::path::PathBuf {
        // Simple key-to-path mapping (in production, use more sophisticated approach)
        let safe_key = key.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_");
        self.base_path.join(safe_key)
    }
}

#[async_trait]
impl StorageProvider for FileSystemStorage {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, DomainError> {
        let path = self.key_to_path(key);
        
        match tokio::fs::read(&path).await {
            Ok(data) => Ok(Some(data)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(DomainError::invalid_operation(format!("Failed to read file: {}", e))),
        }
    }

    async fn set(&self, key: &str, value: Vec<u8>) -> Result<(), DomainError> {
        let path = self.key_to_path(key);
        
        // Create parent directory if needed
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await
                .map_err(|e| DomainError::invalid_operation(format!("Failed to create directory: {}", e)))?;
        }
        
        tokio::fs::write(&path, value).await
            .map_err(|e| DomainError::invalid_operation(format!("Failed to write file: {}", e)))
    }

    async fn delete(&self, key: &str) -> Result<(), DomainError> {
        let path = self.key_to_path(key);
        
        match tokio::fs::remove_file(&path).await {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()), // Already deleted
            Err(e) => Err(DomainError::invalid_operation(format!("Failed to delete file: {}", e))),
        }
    }

    async fn exists(&self, key: &str) -> Result<bool, DomainError> {
        let path = self.key_to_path(key);
        Ok(path.exists())
    }

    async fn list_keys(&self, prefix: &str) -> Result<Vec<String>, DomainError> {
        let mut keys = Vec::new();
        let mut dir = tokio::fs::read_dir(&self.base_path).await
            .map_err(|e| DomainError::invalid_operation(format!("Failed to read directory: {}", e)))?;
        
        while let Some(entry) = dir.next_entry().await
            .map_err(|e| DomainError::invalid_operation(format!("Failed to read directory entry: {}", e)))? {
            
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.starts_with(prefix) {
                    keys.push(file_name.to_string());
                }
            }
        }
        
        Ok(keys)
    }

    async fn clear(&self) -> Result<(), DomainError> {
        let mut dir = tokio::fs::read_dir(&self.base_path).await
            .map_err(|e| DomainError::invalid_operation(format!("Failed to read directory: {}", e)))?;
        
        while let Some(entry) = dir.next_entry().await
            .map_err(|e| DomainError::invalid_operation(format!("Failed to read directory entry: {}", e)))? {
            
            if entry.file_type().await
                .map_err(|e| DomainError::invalid_operation(format!("Failed to get file type: {}", e)))?
                .is_file() {
                
                tokio::fs::remove_file(entry.path()).await
                    .map_err(|e| DomainError::invalid_operation(format!("Failed to delete file: {}", e)))?;
            }
        }
        
        Ok(())
    }
}

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub storage_type: StorageType,
    pub file_system_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageType {
    InMemory,
    FileSystem,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            storage_type: StorageType::InMemory,
            file_system_path: None,
        }
    }
}

/// Storage factory
pub struct StorageFactory;

impl StorageFactory {
    pub fn create_storage(config: &StorageConfig) -> Result<Arc<dyn StorageProvider>, DomainError> {
        match config.storage_type {
            StorageType::InMemory => Ok(Arc::new(InMemoryStorage::new())),
            StorageType::FileSystem => {
                let path = config.file_system_path.as_ref()
                    .ok_or_else(|| DomainError::invalid_operation("File system path required for FileSystem storage"))?;
                let storage = FileSystemStorage::new(path)?;
                Ok(Arc::new(storage))
            }
        }
    }
}
