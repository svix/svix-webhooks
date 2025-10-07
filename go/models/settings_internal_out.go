// Package svix this file is @generated DO NOT EDIT
package models

type SettingsInternalOut struct {
	ApplicationAlertEvents        []ApplicationAlertEvent `json:"applicationAlertEvents,omitempty"`
	ApplicationAlertsDashboardUrl *string                 `json:"applicationAlertsDashboardUrl,omitempty"`
	ApplicationAlertsLogoUrl      *string                 `json:"applicationAlertsLogoUrl,omitempty"`
	ColorPaletteDark              *CustomColorPalette     `json:"colorPaletteDark,omitempty"`
	ColorPaletteLight             *CustomColorPalette     `json:"colorPaletteLight,omitempty"`
	CustomBaseFontSize            *int64                  `json:"customBaseFontSize,omitempty"`
	CustomColor                   *string                 `json:"customColor,omitempty"`
	CustomFontFamily              *string                 `json:"customFontFamily,omitempty"`
	CustomFontFamilyUrl           *string                 `json:"customFontFamilyUrl,omitempty"`
	CustomLogoUrl                 *string                 `json:"customLogoUrl,omitempty"`
	CustomStringsOverride         *CustomStringsOverride  `json:"customStringsOverride,omitempty"`
	CustomThemeOverride           *CustomThemeOverride    `json:"customThemeOverride,omitempty"`
	DisableEndpointOnFailure      *bool                   `json:"disableEndpointOnFailure,omitempty"`
	DisplayName                   *string                 `json:"displayName,omitempty"`
	EnableApplicationAlerts       *bool                   `json:"enableApplicationAlerts,omitempty"`
	EnableChannels                *bool                   `json:"enableChannels,omitempty"`
	EnableEndpointMtlsConfig      *bool                   `json:"enableEndpointMtlsConfig,omitempty"`
	EnableEndpointOauthConfig     *bool                   `json:"enableEndpointOauthConfig,omitempty"`
	EnableIntegrationManagement   *bool                   `json:"enableIntegrationManagement,omitempty"`
	EnableMessageStream           *bool                   `json:"enableMessageStream,omitempty"`
	EnableMsgAtmptLog             *bool                   `json:"enableMsgAtmptLog,omitempty"`
	EnableOtlp                    *bool                   `json:"enableOtlp,omitempty"`
	EnableTransformations         *bool                   `json:"enableTransformations,omitempty"`
	EnforceHttps                  *bool                   `json:"enforceHttps,omitempty"`
	EventCatalogPublished         *bool                   `json:"eventCatalogPublished,omitempty"`
	ReadOnly                      *bool                   `json:"readOnly,omitempty"`
	RequireEndpointChannel        *bool                   `json:"requireEndpointChannel,omitempty"`
	RequireEndpointFilterTypes    *bool                   `json:"requireEndpointFilterTypes,omitempty"`
	RetryPolicy                   []int32                 `json:"retryPolicy,omitempty"`
	SendSvixWebhookHeaders        *bool                   `json:"sendSvixWebhookHeaders,omitempty"`
	ShowSvixBrandFooter           *bool                   `json:"showSvixBrandFooter,omitempty"`
	ShowUseSvixPlay               *bool                   `json:"showUseSvixPlay,omitempty"`
	WhitelabelHeaders             *bool                   `json:"whitelabelHeaders,omitempty"`
	WhitelabelLogo                *string                 `json:"whitelabelLogo,omitempty"`
	WipeSuccessfulPayload         *bool                   `json:"wipeSuccessfulPayload,omitempty"`
}
