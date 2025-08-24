//--------------------------------------------------------------------------------- Location
// src/logics/general.rs

//--------------------------------------------------------------------------------- Description
// General response models

//--------------------------------------------------------------------------------- Import
use serde::{Deserialize, Serialize};

//--------------------------------------------------------------------------------- Models
//------------------------- Output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelOutput<T> 
{
    pub status: bool,
    pub message: String,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ModelOutput<T> 
{
    pub fn success(data: T, message: String) -> Self 
    {
        Self 
        {
            status: true,
            message,
            data: Some(data),
            error: None,
        }
    }

    pub fn success_with_message(message: String) -> Self 
    {
        Self 
        {
            status: true,
            message,
            data: None,
            error: None,
        }
    }

    pub fn error(message: String) -> Self 
    {
        Self 
        {
            status: false,
            message: "Error".to_string(),
            data: None,
            error: Some(message),
        }
    }
}

//------------------------- PORT_PROTOCOLS
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortProtocols {}

impl PortProtocols 
{
    /// Validate if a string represents a valid port protocol
    pub fn is_valid_protocol(protocol_str: &str) -> bool 
    {
        matches!(protocol_str.to_lowercase().as_str(), 
            "pwr" | "gnd" | "reserved" | "file" | "gpio" | "uart" | "i2c" | "spi"
        )
    }

    /// Get all valid port protocol strings
    pub fn valid_protocols() -> Vec<&'static str> 
    {
        vec!["Pwr", "Gnd", "Reserved", "File", "Gpio", "Uart", "I2c", "Spi"]
    }
}

//------------------------- PORT_TYPES
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortTypes {}

impl PortTypes 
{
    /// Validate if a string represents a valid port type
    pub fn is_valid_type(type_str: &str) -> bool 
    {
        matches!(type_str.to_lowercase().as_str(), 
            "in" | "out" | "tx" | "rx" | "sda" | "scl" | 
            "mosi" | "miso" | "sclk" | "cs0" | "cs1"
        )
    }

    /// Get all valid port type strings
    pub fn valid_types() -> Vec<&'static str> 
    {
        vec!["In", "Out", "Tx", "Rx", "Sda", "Scl", "Mosi", "Miso", "Sclk", "Cs0", "Cs1"]
    }
}

//------------------------- IF_TYPES
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IfTypes 
{
    None,
    Equal,
    Unequal,
    Dtu,
    Utd,
}
