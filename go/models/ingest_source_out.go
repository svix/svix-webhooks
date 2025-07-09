// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"time"
)

// When creating an IngestSourceOut, use the appropriate config structure based on the Type:
//   - "generic-webhook": No config needed (nil or just ignore the config field)
//   - "adobe-sign": Use AdobeSignConfigOut
//   - "airwallex": Use AirwallexConfigOut
//   - "checkbook": Use CheckbookConfigOut
//   - "cron": Use CronConfig
//   - "docusign": Use DocusignConfigOut
//   - "github": Use GithubConfigOut
//   - "hubspot": Use HubspotConfigOut
//   - "orum-io": Use OrumIoConfigOut
//   - "panda-doc": Use PandaDocConfigOut
//   - "rutter": Use RutterConfigOut
//   - "segment": Use SegmentConfigOut
//   - "shopify": Use ShopifyConfigOut
//   - "slack": Use SlackConfigOut
//   - "stripe": Use StripeConfigOut
//   - "beehiiv","brex","clerk","guesty","incident-io","lithic","nash","pleo","replicate","resend","safebase","sardine","stych","svix","open-ai","render": Use SvixConfigOut
//   - "telnyx": Use TelnyxConfigOut
//   - "veriff": Use VeriffConfigOut
//   - "zoom": Use ZoomConfigOut
type IngestSourceOut struct {
	CreatedAt time.Time             `json:"createdAt"`
	Id        string                `json:"id"` // The Source's ID.
	IngestUrl *string               `json:"ingestUrl,omitempty"`
	Metadata  map[string]string     `json:"metadata"`
	Name      string                `json:"name"`
	Uid       *string               `json:"uid,omitempty"` // The Source's UID.
	UpdatedAt time.Time             `json:"updatedAt"`
	Type      IngestSourceOutType   `json:"type"`
	Config    IngestSourceOutConfig `json:"config"`
}

type IngestSourceOutType string

const (
	IngestSourceOutTypeGenericWebhook IngestSourceOutType = "generic-webhook"
	IngestSourceOutTypeCron           IngestSourceOutType = "cron"
	IngestSourceOutTypeAdobeSign      IngestSourceOutType = "adobe-sign"
	IngestSourceOutTypeBeehiiv        IngestSourceOutType = "beehiiv"
	IngestSourceOutTypeBrex           IngestSourceOutType = "brex"
	IngestSourceOutTypeCheckbook      IngestSourceOutType = "checkbook"
	IngestSourceOutTypeClerk          IngestSourceOutType = "clerk"
	IngestSourceOutTypeDocusign       IngestSourceOutType = "docusign"
	IngestSourceOutTypeGithub         IngestSourceOutType = "github"
	IngestSourceOutTypeGuesty         IngestSourceOutType = "guesty"
	IngestSourceOutTypeHubspot        IngestSourceOutType = "hubspot"
	IngestSourceOutTypeIncidentIo     IngestSourceOutType = "incident-io"
	IngestSourceOutTypeLithic         IngestSourceOutType = "lithic"
	IngestSourceOutTypeNash           IngestSourceOutType = "nash"
	IngestSourceOutTypeOrumIo         IngestSourceOutType = "orum-io"
	IngestSourceOutTypePandaDoc       IngestSourceOutType = "panda-doc"
	IngestSourceOutTypePleo           IngestSourceOutType = "pleo"
	IngestSourceOutTypeReplicate      IngestSourceOutType = "replicate"
	IngestSourceOutTypeResend         IngestSourceOutType = "resend"
	IngestSourceOutTypeRutter         IngestSourceOutType = "rutter"
	IngestSourceOutTypeSafebase       IngestSourceOutType = "safebase"
	IngestSourceOutTypeSardine        IngestSourceOutType = "sardine"
	IngestSourceOutTypeSegment        IngestSourceOutType = "segment"
	IngestSourceOutTypeShopify        IngestSourceOutType = "shopify"
	IngestSourceOutTypeSlack          IngestSourceOutType = "slack"
	IngestSourceOutTypeStripe         IngestSourceOutType = "stripe"
	IngestSourceOutTypeStych          IngestSourceOutType = "stych"
	IngestSourceOutTypeSvix           IngestSourceOutType = "svix"
	IngestSourceOutTypeZoom           IngestSourceOutType = "zoom"
	IngestSourceOutTypeTelnyx         IngestSourceOutType = "telnyx"
	IngestSourceOutTypeOpenAi         IngestSourceOutType = "open-ai"
	IngestSourceOutTypeRender         IngestSourceOutType = "render"
	IngestSourceOutTypeVeriff         IngestSourceOutType = "veriff"
	IngestSourceOutTypeAirwallex      IngestSourceOutType = "airwallex"
)

type IngestSourceOutConfig interface {
	isIngestSourceOutConfig()
}

func (emptyMap) isIngestSourceOutConfig()           {}
func (CronConfig) isIngestSourceOutConfig()         {}
func (AdobeSignConfigOut) isIngestSourceOutConfig() {}
func (SvixConfigOut) isIngestSourceOutConfig()      {}
func (CheckbookConfigOut) isIngestSourceOutConfig() {}
func (DocusignConfigOut) isIngestSourceOutConfig()  {}
func (GithubConfigOut) isIngestSourceOutConfig()    {}
func (HubspotConfigOut) isIngestSourceOutConfig()   {}
func (OrumIoConfigOut) isIngestSourceOutConfig()    {}
func (PandaDocConfigOut) isIngestSourceOutConfig()  {}
func (RutterConfigOut) isIngestSourceOutConfig()    {}
func (SegmentConfigOut) isIngestSourceOutConfig()   {}
func (ShopifyConfigOut) isIngestSourceOutConfig()   {}
func (SlackConfigOut) isIngestSourceOutConfig()     {}
func (StripeConfigOut) isIngestSourceOutConfig()    {}
func (ZoomConfigOut) isIngestSourceOutConfig()      {}
func (TelnyxConfigOut) isIngestSourceOutConfig()    {}
func (VeriffConfigOut) isIngestSourceOutConfig()    {}
func (AirwallexConfigOut) isIngestSourceOutConfig() {}

func (i *IngestSourceOut) UnmarshalJSON(data []byte) error {
	type Alias IngestSourceOut
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
		var c AdobeSignConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "airwallex":
		var c AirwallexConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "checkbook":
		var c CheckbookConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "cron":
		var c CronConfig
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "docusign":
		var c DocusignConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "github":
		var c GithubConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "hubspot":
		var c HubspotConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "orum-io":
		var c OrumIoConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "panda-doc":
		var c PandaDocConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "rutter":
		var c RutterConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "segment":
		var c SegmentConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "shopify":
		var c ShopifyConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "slack":
		var c SlackConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "stripe":
		var c StripeConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "beehiiv", "brex", "clerk", "guesty", "incident-io", "lithic", "nash", "pleo", "replicate", "resend", "safebase", "sardine", "stych", "svix", "open-ai", "render":
		var c SvixConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "telnyx":
		var c TelnyxConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "veriff":
		var c VeriffConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	case "zoom":
		var c ZoomConfigOut
		err = json.Unmarshal(aux.Config, &c)
		i.Config = c
	default:
		// should be unreachable
		return fmt.Errorf("unexpected type %s", i.Type)
	}
	return err
}

var IngestSourceOutTypeWithNoConfig = map[string]bool{
	"generic-webhook": true,
}

func (i IngestSourceOut) MarshalJSON() ([]byte, error) {
	type Alias IngestSourceOut
	if _, found := IngestSourceOutTypeWithNoConfig[string(i.Type)]; found {
		i.Config = emptyMap{}
	}
	return json.Marshal(&struct{ Alias }{Alias: (Alias)(i)})
}

var IngestSourceOutTypeFromString = map[string]IngestSourceOutType{
	"generic-webhook": IngestSourceOutTypeGenericWebhook,
	"cron":            IngestSourceOutTypeCron,
	"adobe-sign":      IngestSourceOutTypeAdobeSign,
	"beehiiv":         IngestSourceOutTypeBeehiiv,
	"brex":            IngestSourceOutTypeBrex,
	"checkbook":       IngestSourceOutTypeCheckbook,
	"clerk":           IngestSourceOutTypeClerk,
	"docusign":        IngestSourceOutTypeDocusign,
	"github":          IngestSourceOutTypeGithub,
	"guesty":          IngestSourceOutTypeGuesty,
	"hubspot":         IngestSourceOutTypeHubspot,
	"incident-io":     IngestSourceOutTypeIncidentIo,
	"lithic":          IngestSourceOutTypeLithic,
	"nash":            IngestSourceOutTypeNash,
	"orum-io":         IngestSourceOutTypeOrumIo,
	"panda-doc":       IngestSourceOutTypePandaDoc,
	"pleo":            IngestSourceOutTypePleo,
	"replicate":       IngestSourceOutTypeReplicate,
	"resend":          IngestSourceOutTypeResend,
	"rutter":          IngestSourceOutTypeRutter,
	"safebase":        IngestSourceOutTypeSafebase,
	"sardine":         IngestSourceOutTypeSardine,
	"segment":         IngestSourceOutTypeSegment,
	"shopify":         IngestSourceOutTypeShopify,
	"slack":           IngestSourceOutTypeSlack,
	"stripe":          IngestSourceOutTypeStripe,
	"stych":           IngestSourceOutTypeStych,
	"svix":            IngestSourceOutTypeSvix,
	"zoom":            IngestSourceOutTypeZoom,
	"telnyx":          IngestSourceOutTypeTelnyx,
	"open-ai":         IngestSourceOutTypeOpenAi,
	"render":          IngestSourceOutTypeRender,
	"veriff":          IngestSourceOutTypeVeriff,
	"airwallex":       IngestSourceOutTypeAirwallex,
}
