//--------------------------------------------------------------------------------- Location
// src/doc.rs

//--------------------------------------------------------------------------------- Description
// OpenAPI documentation configuration for the Raspberry Pi IoT Core API

//--------------------------------------------------------------------------------- Import
use utoipa::OpenApi;

//--------------------------------------------------------------------------------- OpenAPI Documentation
#[derive(OpenApi)]
#[openapi(
    info(
        title = "🚀 Raspberry Pi IoT Core API",
        version = "1.0.0",
        description = "A comprehensive IoT management API for Raspberry Pi devices with full CRUD operations for all system components.",
        terms_of_service = "https://example.com/terms",
        contact(
            name = "IoT Core API Support",
            email = "support@iotcore.com",
            url = "https://iotcore.com/support"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    servers(
        (url = "http://localhost:3000", description = "Local development server")
    ),
    paths(
        // User endpoints
        crate::api::handlers::user::list_users,
        crate::api::handlers::user::get_user,
        crate::api::handlers::user::create_user,
        crate::api::handlers::user::update_user,
        crate::api::handlers::user::delete_user,
        // Device endpoints
        crate::api::handlers::device::list_devices,
        crate::api::handlers::device::get_device,
        crate::api::handlers::device::create_device,
        crate::api::handlers::device::update_device,
        crate::api::handlers::device::delete_device,
        // Config endpoints
        crate::api::handlers::config::list_configs,
        crate::api::handlers::config::get_config,
        crate::api::handlers::config::create_config,
        crate::api::handlers::config::update_config,
        crate::api::handlers::config::delete_config,
        // Zone endpoints
        crate::api::handlers::zone::list_zones,
        crate::api::handlers::zone::get_zone,
        crate::api::handlers::zone::create_zone,
        crate::api::handlers::zone::update_zone,
        crate::api::handlers::zone::delete_zone,
        // Device Command endpoints
        crate::api::handlers::device_command::list_device_commands,
        crate::api::handlers::device_command::get_device_command,
        crate::api::handlers::device_command::create_device_command,
        crate::api::handlers::device_command::update_device_command,
        crate::api::handlers::device_command::delete_device_command,
        // Log endpoints
        crate::api::handlers::log::list_logs,
        crate::api::handlers::log::get_log,
        crate::api::handlers::log::create_log,
        crate::api::handlers::log::update_log,
        crate::api::handlers::log::delete_log,
        // Port endpoints
        crate::api::handlers::port::list_ports,
        crate::api::handlers::port::get_port,
        crate::api::handlers::port::create_port,
        crate::api::handlers::port::update_port,
        crate::api::handlers::port::delete_port,
        // Timer endpoints
        crate::api::handlers::timer::list_timers,
        crate::api::handlers::timer::get_timer,
        crate::api::handlers::timer::create_timer,
        crate::api::handlers::timer::update_timer,
        crate::api::handlers::timer::delete_timer,
        // Timer Device endpoints (temporarily commented out)
        crate::api::handlers::timer_device::list_timer_devices,
        crate::api::handlers::timer_device::get_timer_device,
        crate::api::handlers::timer_device::create_timer_device,
        crate::api::handlers::timer_device::update_timer_device,
        crate::api::handlers::timer_device::delete_timer_device,
        // Timer Item endpoints
        crate::api::handlers::timer_item::list_timer_items,
        crate::api::handlers::timer_item::get_timer_item,
        crate::api::handlers::timer_item::create_timer_item,
        crate::api::handlers::timer_item::update_timer_item,
        crate::api::handlers::timer_item::delete_timer_item,
        // Timer Limit endpoints
        crate::api::handlers::timer_limit::list_timer_limits,
        crate::api::handlers::timer_limit::get_timer_limit,
        crate::api::handlers::timer_limit::create_timer_limit,
        crate::api::handlers::timer_limit::update_timer_limit,
        crate::api::handlers::timer_limit::delete_timer_limit,
        // Zone Command endpoints
        crate::api::handlers::zone_command::list_zone_commands,
        crate::api::handlers::zone_command::get_zone_command,
        crate::api::handlers::zone_command::create_zone_command,
        crate::api::handlers::zone_command::update_zone_command,
        crate::api::handlers::zone_command::delete_zone_command,
        // Zone Command Action endpoints
        crate::api::handlers::zone_command_action::list_zone_command_actions,
        crate::api::handlers::zone_command_action::get_zone_command_action,
        crate::api::handlers::zone_command_action::create_zone_command_action,
        crate::api::handlers::zone_command_action::update_zone_command_action,
        crate::api::handlers::zone_command_action::delete_zone_command_action,
        // Zone Command If endpoints
        crate::api::handlers::zone_command_if::list_zone_command_ifs,
        crate::api::handlers::zone_command_if::get_zone_command_if,
        crate::api::handlers::zone_command_if::create_zone_command_if,
        crate::api::handlers::zone_command_if::update_zone_command_if,
        crate::api::handlers::zone_command_if::delete_zone_command_if,
    ),
    components(
        schemas(
            // Model schemas
            crate::orm::models::user::Model,
            crate::orm::models::device::Model,
            crate::orm::models::zone::Model,
            crate::orm::models::config::Model,
            crate::orm::models::device_command::Model,
            crate::orm::models::log::Model,
            crate::orm::models::port::Model,
            crate::orm::models::timer::Model,
            crate::orm::models::timer_device::Model,
            crate::orm::models::timer_item::Model,
            crate::orm::models::timer_limit::Model,
            crate::orm::models::zone_command::Model,
            crate::orm::models::zone_command_action::Model,
            crate::orm::models::zone_command_if::Model,
        )
    ),
    tags(
        (name = "👥 Users", description = "User management and authentication operations"),
        (name = "🔧 Devices", description = "IoT device management and control operations"),
        (name = "⚙️ Configs", description = "System configuration management operations"),
        (name = "🏠 Zones", description = "Zone management for organizing devices"),
        (name = "📡 Device Commands", description = "Device command execution and management"),
        (name = "📋 Logs", description = "System logging and audit trail operations"),
        (name = "🔌 Ports", description = "Hardware port configuration and management"),
        (name = "⏰ Timers", description = "Timer and scheduling operations"),
        (name = "🔗 Timer Devices", description = "Timer-device relationship management"),
        (name = "📝 Timer Items", description = "Individual timer item configuration"),
        (name = "⏱️ Timer Limits", description = "Timer limitation and constraint management"),
        (name = "🎯 Zone Commands", description = "Zone-level command operations"),
        (name = "⚡ Zone Command Actions", description = "Zone command action execution"),
        (name = "🔀 Zone Command Conditions", description = "Zone command conditional logic")
    ),
    external_docs(
        description = "Find more info about IoT Core API",
        url = "https://docs.iotcore.com"
    )
)]
pub struct ApiDoc;
