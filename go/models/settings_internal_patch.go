// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"

	"github.com/svix/svix-webhooks/go/utils"
)

type SettingsInternalPatch struct {
	ApplicationAlertEvents        []ApplicationAlertEvent               `json:"applicationAlertEvents,omitempty"`
	ApplicationAlertsDashboardUrl utils.Nullable[string]                `json:"applicationAlertsDashboardUrl"`
	ApplicationAlertsLogoUrl      utils.Nullable[string]                `json:"applicationAlertsLogoUrl"`
	ColorPaletteDark              utils.Nullable[CustomColorPalette]    `json:"colorPaletteDark"`
	ColorPaletteLight             utils.Nullable[CustomColorPalette]    `json:"colorPaletteLight"`
	CustomBaseFontSize            utils.Nullable[int64]                 `json:"customBaseFontSize"`
	CustomColor                   utils.Nullable[string]                `json:"customColor"`
	CustomFontFamily              utils.Nullable[string]                `json:"customFontFamily"`
	CustomFontFamilyUrl           utils.Nullable[string]                `json:"customFontFamilyUrl"`
	CustomLogoUrl                 utils.Nullable[string]                `json:"customLogoUrl"`
	CustomStringsOverride         utils.Nullable[CustomStringsOverride] `json:"customStringsOverride"`
	CustomThemeOverride           utils.Nullable[CustomThemeOverride]   `json:"customThemeOverride"`
	DisableEndpointOnFailure      *bool                                 `json:"disableEndpointOnFailure,omitempty"`
	DisplayName                   utils.Nullable[string]                `json:"displayName"`
	EnableApplicationAlerts       *bool                                 `json:"enableApplicationAlerts,omitempty"`
	EnableChannels                *bool                                 `json:"enableChannels,omitempty"`
	EnableEndpointMtlsConfig      *bool                                 `json:"enableEndpointMtlsConfig,omitempty"`
	EnableEndpointOauthConfig     *bool                                 `json:"enableEndpointOauthConfig,omitempty"`
	EnableIntegrationManagement   *bool                                 `json:"enableIntegrationManagement,omitempty"`
	EnableMessageStream           *bool                                 `json:"enableMessageStream,omitempty"`
	EnableMsgAtmptLog             *bool                                 `json:"enableMsgAtmptLog,omitempty"`
	EnableOtlp                    *bool                                 `json:"enableOtlp,omitempty"`
	EnableTransformations         *bool                                 `json:"enableTransformations,omitempty"`
	EnforceHttps                  *bool                                 `json:"enforceHttps,omitempty"`
	EventCatalogPublished         *bool                                 `json:"eventCatalogPublished,omitempty"`
	ReadOnly                      *bool                                 `json:"readOnly,omitempty"`
	RequireEndpointChannel        *bool                                 `json:"requireEndpointChannel,omitempty"`
	RequireEndpointFilterTypes    *bool                                 `json:"requireEndpointFilterTypes,omitempty"`
	RetryPolicy                   utils.Nullable[[]int32]               `json:"retryPolicy"`
	SendSvixWebhookHeaders        *bool                                 `json:"sendSvixWebhookHeaders,omitempty"`
	ShowFeatureTooltips           *bool                                 `json:"showFeatureTooltips,omitempty"`
	ShowUseSvixPlay               *bool                                 `json:"showUseSvixPlay,omitempty"`
	WhitelabelHeaders             *bool                                 `json:"whitelabelHeaders,omitempty"`
	WipeSuccessfulPayload         *bool                                 `json:"wipeSuccessfulPayload,omitempty"`
}

func (o SettingsInternalPatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.ApplicationAlertEvents != nil {
		toSerialize["applicationAlertEvents"] = o.ApplicationAlertEvents
	}
	if o.ApplicationAlertsDashboardUrl.IsSet() {
		toSerialize["applicationAlertsDashboardUrl"] = o.ApplicationAlertsDashboardUrl
	}
	if o.ApplicationAlertsLogoUrl.IsSet() {
		toSerialize["applicationAlertsLogoUrl"] = o.ApplicationAlertsLogoUrl
	}
	if o.ColorPaletteDark.IsSet() {
		toSerialize["colorPaletteDark"] = o.ColorPaletteDark
	}
	if o.ColorPaletteLight.IsSet() {
		toSerialize["colorPaletteLight"] = o.ColorPaletteLight
	}
	if o.CustomBaseFontSize.IsSet() {
		toSerialize["customBaseFontSize"] = o.CustomBaseFontSize
	}
	if o.CustomColor.IsSet() {
		toSerialize["customColor"] = o.CustomColor
	}
	if o.CustomFontFamily.IsSet() {
		toSerialize["customFontFamily"] = o.CustomFontFamily
	}
	if o.CustomFontFamilyUrl.IsSet() {
		toSerialize["customFontFamilyUrl"] = o.CustomFontFamilyUrl
	}
	if o.CustomLogoUrl.IsSet() {
		toSerialize["customLogoUrl"] = o.CustomLogoUrl
	}
	if o.CustomStringsOverride.IsSet() {
		toSerialize["customStringsOverride"] = o.CustomStringsOverride
	}
	if o.CustomThemeOverride.IsSet() {
		toSerialize["customThemeOverride"] = o.CustomThemeOverride
	}
	if o.DisableEndpointOnFailure != nil {
		toSerialize["disableEndpointOnFailure"] = o.DisableEndpointOnFailure
	}
	if o.DisplayName.IsSet() {
		toSerialize["displayName"] = o.DisplayName
	}
	if o.EnableApplicationAlerts != nil {
		toSerialize["enableApplicationAlerts"] = o.EnableApplicationAlerts
	}
	if o.EnableChannels != nil {
		toSerialize["enableChannels"] = o.EnableChannels
	}
	if o.EnableEndpointMtlsConfig != nil {
		toSerialize["enableEndpointMtlsConfig"] = o.EnableEndpointMtlsConfig
	}
	if o.EnableEndpointOauthConfig != nil {
		toSerialize["enableEndpointOauthConfig"] = o.EnableEndpointOauthConfig
	}
	if o.EnableIntegrationManagement != nil {
		toSerialize["enableIntegrationManagement"] = o.EnableIntegrationManagement
	}
	if o.EnableMessageStream != nil {
		toSerialize["enableMessageStream"] = o.EnableMessageStream
	}
	if o.EnableMsgAtmptLog != nil {
		toSerialize["enableMsgAtmptLog"] = o.EnableMsgAtmptLog
	}
	if o.EnableOtlp != nil {
		toSerialize["enableOtlp"] = o.EnableOtlp
	}
	if o.EnableTransformations != nil {
		toSerialize["enableTransformations"] = o.EnableTransformations
	}
	if o.EnforceHttps != nil {
		toSerialize["enforceHttps"] = o.EnforceHttps
	}
	if o.EventCatalogPublished != nil {
		toSerialize["eventCatalogPublished"] = o.EventCatalogPublished
	}
	if o.ReadOnly != nil {
		toSerialize["readOnly"] = o.ReadOnly
	}
	if o.RequireEndpointChannel != nil {
		toSerialize["requireEndpointChannel"] = o.RequireEndpointChannel
	}
	if o.RequireEndpointFilterTypes != nil {
		toSerialize["requireEndpointFilterTypes"] = o.RequireEndpointFilterTypes
	}
	if o.RetryPolicy.IsSet() {
		toSerialize["retryPolicy"] = o.RetryPolicy
	}
	if o.SendSvixWebhookHeaders != nil {
		toSerialize["sendSvixWebhookHeaders"] = o.SendSvixWebhookHeaders
	}
	if o.ShowFeatureTooltips != nil {
		toSerialize["showFeatureTooltips"] = o.ShowFeatureTooltips
	}
	if o.ShowUseSvixPlay != nil {
		toSerialize["showUseSvixPlay"] = o.ShowUseSvixPlay
	}
	if o.WhitelabelHeaders != nil {
		toSerialize["whitelabelHeaders"] = o.WhitelabelHeaders
	}
	if o.WipeSuccessfulPayload != nil {
		toSerialize["wipeSuccessfulPayload"] = o.WipeSuccessfulPayload
	}
	return json.Marshal(toSerialize)
}
