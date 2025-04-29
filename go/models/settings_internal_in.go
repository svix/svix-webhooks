// Package svix this file is @generated DO NOT EDIT
package models

type SettingsInternalIn struct {
	ColorPaletteDark            *CustomColorPalette    `json:"colorPaletteDark,omitempty"`
	ColorPaletteLight           *CustomColorPalette    `json:"colorPaletteLight,omitempty"`
	CustomBaseFontSize          *int64                 `json:"customBaseFontSize,omitempty"`
	CustomColor                 *string                `json:"customColor,omitempty"`
	CustomFontFamily            *string                `json:"customFontFamily,omitempty"`
	CustomFontFamilyUrl         *string                `json:"customFontFamilyUrl,omitempty"`
	CustomLogoUrl               *string                `json:"customLogoUrl,omitempty"`
	CustomStringsOverride       *CustomStringsOverride `json:"customStringsOverride,omitempty"`
	CustomThemeOverride         *CustomThemeOverride   `json:"customThemeOverride,omitempty"`
	DisableEndpointOnFailure    *bool                  `json:"disableEndpointOnFailure,omitempty"`
	DisplayName                 *string                `json:"displayName,omitempty"`
	EnableChannels              *bool                  `json:"enableChannels,omitempty"`
	EnableEndpointMtlsConfig    *bool                  `json:"enableEndpointMtlsConfig,omitempty"`
	EnableEndpointOauthConfig   *bool                  `json:"enableEndpointOauthConfig,omitempty"`
	EnableIntegrationManagement *bool                  `json:"enableIntegrationManagement,omitempty"`
	EnableMessageStream         *bool                  `json:"enableMessageStream,omitempty"`
	EnableMsgAtmptLog           *bool                  `json:"enableMsgAtmptLog,omitempty"`
	EnableOtlp                  *bool                  `json:"enableOtlp,omitempty"`
	EnableTransformations       *bool                  `json:"enableTransformations,omitempty"`
	EnforceHttps                *bool                  `json:"enforceHttps,omitempty"`
	EventCatalogPublished       *bool                  `json:"eventCatalogPublished,omitempty"`
	ReadOnly                    *bool                  `json:"readOnly,omitempty"`
	RequireEndpointChannel      *bool                  `json:"requireEndpointChannel,omitempty"`
	RequireEndpointFilterTypes  *bool                  `json:"requireEndpointFilterTypes,omitempty"`
	RetryPolicy                 []int32                `json:"retryPolicy,omitempty"`
	ShowUseSvixPlay             *bool                  `json:"showUseSvixPlay,omitempty"`
	WhitelabelHeaders           *bool                  `json:"whitelabelHeaders,omitempty"`
	WipeSuccessfulPayload       *bool                  `json:"wipeSuccessfulPayload,omitempty"`
}
