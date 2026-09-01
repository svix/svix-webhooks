// Package svix this file is @generated DO NOT EDIT
package models

type SettingsInternalUpdateOut struct {
	RetryPolicy            []int32                `json:"retryPolicy,omitempty"`
	RequireEndpointChannel *bool                  `json:"requireEndpointChannel,omitempty"`
	WhitelabelHeaders      *bool                  `json:"whitelabelHeaders,omitempty"`
	SendSvixWebhookHeaders *bool                  `json:"sendSvixWebhookHeaders,omitempty"`
	EnableMsgAtmptLog      *bool                  `json:"enableMsgAtmptLog,omitempty"`
	EnableOtlp             *bool                  `json:"enableOtlp,omitempty"`
	CustomColor            *string                `json:"customColor,omitempty"`
	CustomLogoUrl          *string                `json:"customLogoUrl,omitempty"`
	CustomThemeOverride    *CustomThemeOverride   `json:"customThemeOverride,omitempty"`
	CustomStringsOverride  *CustomStringsOverride `json:"customStringsOverride,omitempty"`
	CustomBaseFontSize     *int64                 `json:"customBaseFontSize,omitempty"`
	CustomFontFamily       *string                `json:"customFontFamily,omitempty"`
	CustomFontFamilyUrl    *string                `json:"customFontFamilyUrl,omitempty"`
	// A CSS `font-family` value, used as-is by the App Portal.
	//
	// Can be combined with `customFontFamily`, which only accepts one of the fonts the App Portal knows how to load: when both are set, that font is placed first and this stack provides the fallbacks.
	CustomFontStack               *string                 `json:"customFontStack,omitempty"`
	DisableEndpointOnFailure      *bool                   `json:"disableEndpointOnFailure,omitempty"`
	DisplayName                   *string                 `json:"displayName,omitempty"`
	EventCatalogPublished         *bool                   `json:"eventCatalogPublished,omitempty"`
	EnforceHttps                  *bool                   `json:"enforceHttps,omitempty"`
	EnableChannels                *bool                   `json:"enableChannels,omitempty"`
	EnableMessageStream           *bool                   `json:"enableMessageStream,omitempty"`
	AdvancedEndpointTypes         []AdvancedEndpointType  `json:"advancedEndpointTypes,omitempty"`
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
	EndpointsPerAppLimit          *uint32                 `json:"endpointsPerAppLimit,omitempty"`
	McpToken                      *bool                   `json:"mcpToken,omitempty"`
}
