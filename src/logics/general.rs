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
pub enum PortProtocols 
{
    Pwr,
    Gnd,
    Reserved,
    File,
    Gpio,
    Uart,
    I2c,
    Spi,
}

//------------------------- PORT_TYPES
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortTypes 
{
    In,
    Out,
    Tx,
    Rx,
    Sda,
    Scl,
    Mosi,
    Miso,
    Sclk,
    Cs0,
    Cs1,
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
