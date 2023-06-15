//! This module contains handler stubs for operations that are implemented
//! elsewhere (most likely in Python) but need to be part of the OpenAPI
//! specification.
//!
//! The handlers are named the same as in Python for easier cross-referencing.

use crate::{error::Result, AppState};
use aide::{
    axum::{routing::get_with, ApiRouter},
    transform::TransformPathItem,
};
use axum::Json;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use svix_server_derive::aide_annotate;

fn custom_font_family_example() -> &'static str {
    "Open Sans"
}
fn default_false() -> bool {
    false
}

#[derive(Deserialize, Serialize)]
struct Color(String);

impl JsonSchema for Color {
    fn schema_name() -> String {
        "Color".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        use schemars::schema::Schema;

        let mut schema = String::json_schema(gen);
        if let Schema::Object(obj) = &mut schema {
            obj.format = Some("color".to_string());
        }

        schema
    }

    fn is_referenceable() -> bool {
        false
    }
}

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct EnvironmentSettingsOut {
    custom_color: Option<Color>,

    #[schemars(url, length(min = 1, max = 65_536))]
    custom_logo_url: Option<String>,

    #[schemars(
        regex(pattern = r"^[a-zA-Z0-9\-_ ]+$"),
        example = "custom_font_family_example"
    )]
    custom_font_family: Option<String>,

    custom_theme_override: Option<CustomThemeOverride>,

    #[schemars(default = "default_false")]
    enable_channels: bool,

    #[schemars(default = "default_false")]
    enable_integration_management: bool,

    #[schemars(default = "default_false")]
    enable_transformations: bool,

    color_palette_light: Option<CustomColorPalette>,
    color_palette_dark: Option<CustomColorPalette>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct CustomThemeOverride {
    border_radius: Option<BorderRadiusConfig>,
    font_size: Option<FontSizeConfig>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct BorderRadiusConfig {
    button: Option<BorderRadiusEnum>,
    card: Option<BorderRadiusEnum>,
    input: Option<BorderRadiusEnum>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
enum BorderRadiusEnum {
    #[serde(rename = "none")]
    Zero,
    Lg,
    Md,
    Sm,
    Full,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct FontSizeConfig {
    base: Option<isize>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct CustomColorPalette {
    background_primary: Option<Color>,
    background_secondary: Option<Color>,
    background_hover: Option<Color>,
    text_primary: Option<Color>,
    text_danger: Option<Color>,
    interactive_accent: Option<Color>,
}

/// Get the environment's settings
#[aide_annotate(op_id = "v1.environment.get-settings")]
async fn get_org_settings() -> Result<Json<EnvironmentSettingsOut>> {
    Ok(Json(EnvironmentSettingsOut {
        custom_color: None,
        custom_logo_url: None,
        custom_font_family: None,
        custom_theme_override: None,
        enable_channels: false,
        enable_integration_management: false,
        enable_transformations: false,
        color_palette_light: None,
        color_palette_dark: None,
    }))
}

fn default_true() -> bool {
    true
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct SettingsCommon {
    custom_color: Option<Color>,

    #[schemars(url, length(min = 1, max = 65_536))]
    custom_logo_url: Option<String>,

    custom_theme_override: Option<CustomThemeOverride>,

    custom_base_font_size: Option<isize>,

    #[schemars(
        regex(pattern = r"^[a-zA-Z0-9\-_ ]+$"),
        example = "custom_font_family_example"
    )]
    custom_font_family: Option<String>,

    #[schemars(default = "default_true")]
    disable_endpoint_on_failure: bool,

    display_name: Option<String>,

    #[schemars(default = "default_false")]
    event_catalog_published: bool,

    #[schemars(default = "default_true")]
    enforce_https: bool,

    #[schemars(default = "default_false")]
    enable_channels: bool,

    #[schemars(default = "default_false")]
    read_only: bool,

    #[schemars(default = "default_false")]
    enable_integration_management: bool,

    #[schemars(default = "default_false")]
    enable_transformations: bool,

    #[schemars(skip, default = "default_false")]
    require_endpoint_filter_types: bool,

    color_palette_light: Option<CustomColorPalette>,
    color_palette_dark: Option<CustomColorPalette>,
}

/// If `hide` is set to true then the routes registered by this router will be
/// hidden from documentation.
pub fn router(hide: bool) -> ApiRouter<AppState> {
    fn hide_with_tag(
        hide: bool,
        tag: &str,
    ) -> impl Fn(TransformPathItem) -> TransformPathItem + '_ {
        move |op| op.tag(tag).hidden(hide)
    }

    ApiRouter::new().api_route_with(
        "/environment/settings/",
        get_with(get_org_settings, get_org_settings_operation),
        hide_with_tag(hide, "Environment-Settings"),
    )
}
