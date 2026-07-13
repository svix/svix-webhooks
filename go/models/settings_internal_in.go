// Package svix this file is @generated DO NOT EDIT
package models

type SettingsInternalIn struct {
	RetryPolicy                   []int32                 `json:"retryPolicy,omitempty"`
	RequireEndpointChannel        *bool                   `json:"requireEndpointChannel,omitempty"`
	WhitelabelHeaders             *bool                   `json:"whitelabelHeaders,omitempty"`
	SendSvixWebhookHeaders        *bool                   `json:"sendSvixWebhookHeaders,omitempty"`
	EnableMsgAtmptLog             *bool                   `json:"enableMsgAtmptLog,omitempty"`
	EnableOtlp                    *bool                   `json:"enableOtlp,omitempty"`
	CustomColor                   *string                 `json:"customColor,omitempty"`
	CustomLogoUrl                 *string                 `json:"customLogoUrl,omitempty"`
	CustomThemeOverride           *CustomThemeOverride    `json:"customThemeOverride,omitempty"`
	CustomStringsOverride         *CustomStringsOverride  `json:"customStringsOverride,omitempty"`
	CustomBaseFontSize            *int64                  `json:"customBaseFontSize,omitempty"`
	CustomFontFamily              *string                 `json:"customFontFamily,omitempty"`
	CustomFontFamilyUrl           *string                 `json:"customFontFamilyUrl,omitempty"`
	DisableEndpointOnFailure      *bool                   `json:"disableEndpointOnFailure,omitempty"`
	DisplayName                   *string                 `json:"displayName,omitempty"`
	EventCatalogPublished         *bool                   `json:"eventCatalogPublished,omitempty"`
	EnforceHttps                  *bool                   `json:"enforceHttps,omitempty"`
	EnableChannels                *bool                   `json:"enableChannels,omitempty"`
	EnableMessageStream           *bool                   `json:"enableMessageStream,omitempty"`
	ReadOnly                      *bool                   `json:"readOnly,omitempty"`
	EnableIntegrationManagement   *bool                   `json:"enableIntegrationManagement,omitempty"`
	EnableTransformations         *bool                   `json:"enableTransformations,omitempty"`
	RequireEndpointFilterTypes    *bool                   `json:"requireEndpointFilterTypes,omitempty"`
	EnableEndpointOauthConfig     *bool                   `json:"enableEndpointOauthConfig,omitempty"`
	EnableEndpointMtlsConfig      *bool                   `json:"enableEndpointMtlsConfig,omitempty"`
	ColorPaletteLight             *CustomColorPalette     `json:"colorPaletteLight,omitempty"`
	ColorPaletteDark              *CustomColorPalette     `json:"colorPaletteDark,omitempty"`
	ShowUseSvixPlay               *bool                   `json:"showUseSvixPlay,omitempty"`
	WipeSuccessfulPayload         *bool                   `json:"wipeSuccessfulPayload,omitempty"`
	EnableApplicationAlerts       *bool                   `json:"enableApplicationAlerts,omitempty"`
	ApplicationAlertEvents        []ApplicationAlertEvent `json:"applicationAlertEvents,omitempty"`
	ApplicationAlertsDashboardUrl *string                 `json:"applicationAlertsDashboardUrl,omitempty"`
	ApplicationAlertsLogoUrl      *string                 `json:"applicationAlertsLogoUrl,omitempty"`
	ShowFeatureTooltips           *bool                   `json:"showFeatureTooltips,omitempty"`
	WebhooksAutoConfig            *bool                   `json:"webhooksAutoConfig,omitempty"`
	McpToken                      *bool                   `json:"mcpToken,omitempty"`
}
