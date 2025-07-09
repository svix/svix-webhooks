// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
)

// When creating an IngestSourceIn, use the appropriate config structure based on the Type:
//   - "generic-webhook": No config needed (nil or just ignore the config field)
//   - "adobe-sign": Use AdobeSignConfig
//   - "airwallex": Use AirwallexConfig
//   - "checkbook": Use CheckbookConfig
//   - "cron": Use CronConfig
//   - "docusign": Use DocusignConfig
//   - "github": Use GithubConfig
//   - "hubspot": Use HubspotConfig
//   - "orum-io": Use OrumIoConfig
//   - "panda-doc": Use PandaDocConfig
//   - "rutter": Use RutterConfig
//   - "segment": Use SegmentConfig
//   - "shopify": Use ShopifyConfig
//   - "slack": Use SlackConfig
//   - "stripe": Use StripeConfig
//   - "beehiiv","brex","clerk","guesty","incident-io","lithic","nash","pleo","replicate","resend","safebase","sardine","stych","svix","open-ai","render": Use SvixConfig
//   - "telnyx": Use TelnyxConfig
//   - "veriff": Use VeriffConfig
//   - "zoom": Use ZoomConfig
type IngestSourceIn struct {
	Metadata *map[string]string   `json:"metadata,omitempty"`
	Name     string               `json:"name"`
	Uid      *string              `json:"uid,omitempty"` // The Source's UID.
	Type     IngestSourceInType   `json:"type"`
	Config   IngestSourceInConfig `json:"config"`
}

type IngestSourceInType string

const (
	IngestSourceInTypeGenericWebhook IngestSourceInType = "generic-webhook"
	IngestSourceInTypeCron           IngestSourceInType = "cron"
	IngestSourceInTypeAdobeSign      IngestSourceInType = "adobe-sign"
	IngestSourceInTypeBeehiiv        IngestSourceInType = "beehiiv"
	IngestSourceInTypeBrex           IngestSourceInType = "brex"
	IngestSourceInTypeCheckbook      IngestSourceInType = "checkbook"
	IngestSourceInTypeClerk          IngestSourceInType = "clerk"
	IngestSourceInTypeDocusign       IngestSourceInType = "docusign"
	IngestSourceInTypeGithub         IngestSourceInType = "github"
	IngestSourceInTypeGuesty         IngestSourceInType = "guesty"
	IngestSourceInTypeHubspot        IngestSourceInType = "hubspot"
	IngestSourceInTypeIncidentIo     IngestSourceInType = "incident-io"
	IngestSourceInTypeLithic         IngestSourceInType = "lithic"
	IngestSourceInTypeNash           IngestSourceInType = "nash"
	IngestSourceInTypeOrumIo         IngestSourceInType = "orum-io"
	IngestSourceInTypePandaDoc       IngestSourceInType = "panda-doc"
	IngestSourceInTypePleo           IngestSourceInType = "pleo"
	IngestSourceInTypeReplicate      IngestSourceInType = "replicate"
	IngestSourceInTypeResend         IngestSourceInType = "resend"
	IngestSourceInTypeRutter         IngestSourceInType = "rutter"
	IngestSourceInTypeSafebase       IngestSourceInType = "safebase"
	IngestSourceInTypeSardine        IngestSourceInType = "sardine"
	IngestSourceInTypeSegment        IngestSourceInType = "segment"
	IngestSourceInTypeShopify        IngestSourceInType = "shopify"
	IngestSourceInTypeSlack          IngestSourceInType = "slack"
	IngestSourceInTypeStripe         IngestSourceInType = "stripe"
	IngestSourceInTypeStych          IngestSourceInType = "stych"
	IngestSourceInTypeSvix           IngestSourceInType = "svix"
	IngestSourceInTypeZoom           IngestSourceInType = "zoom"
	IngestSourceInTypeTelnyx         IngestSourceInType = "telnyx"
	IngestSourceInTypeOpenAi         IngestSourceInType = "open-ai"
	IngestSourceInTypeRender         IngestSourceInType = "render"
	IngestSourceInTypeVeriff         IngestSourceInType = "veriff"
	IngestSourceInTypeAirwallex      IngestSourceInType = "airwallex"
)

type IngestSourceInConfig interface {
	isIngestSourceInConfig()
}

func (emptyMap) isIngestSourceInConfig()        {}
func (CronConfig) isIngestSourceInConfig()      {}
func (AdobeSignConfig) isIngestSourceInConfig() {}
func (SvixConfig) isIngestSourceInConfig()      {}
func (CheckbookConfig) isIngestSourceInConfig() {}
func (DocusignConfig) isIngestSourceInConfig()  {}
func (GithubConfig) isIngestSourceInConfig()    {}
func (HubspotConfig) isIngestSourceInConfig()   {}
func (OrumIoConfig) isIngestSourceInConfig()    {}
func (PandaDocConfig) isIngestSourceInConfig()  {}
func (RutterConfig) isIngestSourceInConfig()    {}
func (SegmentConfig) isIngestSourceInConfig()   {}
func (ShopifyConfig) isIngestSourceInConfig()   {}
func (SlackConfig) isIngestSourceInConfig()     {}
func (StripeConfig) isIngestSourceInConfig()    {}
func (ZoomConfig) isIngestSourceInConfig()      {}
func (TelnyxConfig) isIngestSourceInConfig()    {}
func (VeriffConfig) isIngestSourceInConfig()    {}
func (AirwallexConfig) isIngestSourceInConfig() {}

func (i *IngestSourceIn) UnmarshalJSON(data []byte) error {
	type Alias IngestSourceIn
	aux := struct {
		*Alias
		Config json.RawMessage `json:"config"`
	}{Alias: (*Alias)(i)}

	if err := json.Unmarshal(data, &aux); err != nil {
		return err
	}

	var err error
	switch i.Type {
	case "generic-webhook":
	case "adobe-sign":
		var c AdobeSignConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "airwallex":
		var c AirwallexConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "checkbook":
		var c CheckbookConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "cron":
		var c CronConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "docusign":
		var c DocusignConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "github":
		var c GithubConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "hubspot":
		var c HubspotConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "orum-io":
		var c OrumIoConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "panda-doc":
		var c PandaDocConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "rutter":
		var c RutterConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "segment":
		var c SegmentConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "shopify":
		var c ShopifyConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "slack":
		var c SlackConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "stripe":
		var c StripeConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "beehiiv", "brex", "clerk", "guesty", "incident-io", "lithic", "nash", "pleo", "replicate", "resend", "safebase", "sardine", "stych", "svix", "open-ai", "render":
		var c SvixConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "telnyx":
		var c TelnyxConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "veriff":
		var c VeriffConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "zoom":
		var c ZoomConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	default:
		// should be unreachable
		return fmt.Errorf("unexpected type %s", i.Type)
	}
	return err
}

var IngestSourceInTypeWithNoConfig = map[string]bool{
	"generic-webhook": true,
}

func (i IngestSourceIn) MarshalJSON() ([]byte, error) {
	type Alias IngestSourceIn
	if _, found := IngestSourceInTypeWithNoConfig[string(i.Type)]; found {
		i.Config = emptyMap{}
	}
	return json.Marshal(&struct{ Alias }{Alias: (Alias)(i)})
}

var IngestSourceInTypeFromString = map[string]IngestSourceInType{
	"generic-webhook": IngestSourceInTypeGenericWebhook,
	"cron":            IngestSourceInTypeCron,
	"adobe-sign":      IngestSourceInTypeAdobeSign,
	"beehiiv":         IngestSourceInTypeBeehiiv,
	"brex":            IngestSourceInTypeBrex,
	"checkbook":       IngestSourceInTypeCheckbook,
	"clerk":           IngestSourceInTypeClerk,
	"docusign":        IngestSourceInTypeDocusign,
	"github":          IngestSourceInTypeGithub,
	"guesty":          IngestSourceInTypeGuesty,
	"hubspot":         IngestSourceInTypeHubspot,
	"incident-io":     IngestSourceInTypeIncidentIo,
	"lithic":          IngestSourceInTypeLithic,
	"nash":            IngestSourceInTypeNash,
	"orum-io":         IngestSourceInTypeOrumIo,
	"panda-doc":       IngestSourceInTypePandaDoc,
	"pleo":            IngestSourceInTypePleo,
	"replicate":       IngestSourceInTypeReplicate,
	"resend":          IngestSourceInTypeResend,
	"rutter":          IngestSourceInTypeRutter,
	"safebase":        IngestSourceInTypeSafebase,
	"sardine":         IngestSourceInTypeSardine,
	"segment":         IngestSourceInTypeSegment,
	"shopify":         IngestSourceInTypeShopify,
	"slack":           IngestSourceInTypeSlack,
	"stripe":          IngestSourceInTypeStripe,
	"stych":           IngestSourceInTypeStych,
	"svix":            IngestSourceInTypeSvix,
	"zoom":            IngestSourceInTypeZoom,
	"telnyx":          IngestSourceInTypeTelnyx,
	"open-ai":         IngestSourceInTypeOpenAi,
	"render":          IngestSourceInTypeRender,
	"veriff":          IngestSourceInTypeVeriff,
	"airwallex":       IngestSourceInTypeAirwallex,
}
