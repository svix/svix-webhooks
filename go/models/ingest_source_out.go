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
//   - "cron": Use CronConfig
//   - "docusign": Use DocusignConfigOut
//   - "github": Use GithubConfigOut
//   - "hubspot": Use HubspotConfigOut
//   - "panda-doc": Use PandaDocConfigOut
//   - "segment": Use SegmentConfigOut
//   - "shopify": Use ShopifyConfigOut
//   - "slack": Use SlackConfigOut
//   - "stripe": Use StripeConfigOut
//   - "beehiiv","brex","clerk","guesty","incident-io","lithic","nash","pleo","replicate","resend","safebase","sardine","stych","svix": Use SvixConfigOut
//   - "zoom": Use ZoomConfigOut
type IngestSourceOut struct {
	CreatedAt time.Time             `json:"createdAt"`
	Id        string                `json:"id"` // The Source's ID.
	IngestUrl *string               `json:"ingestUrl,omitempty"`
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
	IngestSourceOutTypeClerk          IngestSourceOutType = "clerk"
	IngestSourceOutTypeDocusign       IngestSourceOutType = "docusign"
	IngestSourceOutTypeGithub         IngestSourceOutType = "github"
	IngestSourceOutTypeGuesty         IngestSourceOutType = "guesty"
	IngestSourceOutTypeHubspot        IngestSourceOutType = "hubspot"
	IngestSourceOutTypeIncidentIo     IngestSourceOutType = "incident-io"
	IngestSourceOutTypeLithic         IngestSourceOutType = "lithic"
	IngestSourceOutTypeNash           IngestSourceOutType = "nash"
	IngestSourceOutTypePandaDoc       IngestSourceOutType = "panda-doc"
	IngestSourceOutTypePleo           IngestSourceOutType = "pleo"
	IngestSourceOutTypeReplicate      IngestSourceOutType = "replicate"
	IngestSourceOutTypeResend         IngestSourceOutType = "resend"
	IngestSourceOutTypeSafebase       IngestSourceOutType = "safebase"
	IngestSourceOutTypeSardine        IngestSourceOutType = "sardine"
	IngestSourceOutTypeSegment        IngestSourceOutType = "segment"
	IngestSourceOutTypeShopify        IngestSourceOutType = "shopify"
	IngestSourceOutTypeSlack          IngestSourceOutType = "slack"
	IngestSourceOutTypeStripe         IngestSourceOutType = "stripe"
	IngestSourceOutTypeStych          IngestSourceOutType = "stych"
	IngestSourceOutTypeSvix           IngestSourceOutType = "svix"
	IngestSourceOutTypeZoom           IngestSourceOutType = "zoom"
)

type IngestSourceOutConfig interface {
	isIngestSourceOutConfig()
}

func (emptyMap) isIngestSourceOutConfig()           {}
func (CronConfig) isIngestSourceOutConfig()         {}
func (AdobeSignConfigOut) isIngestSourceOutConfig() {}
func (SvixConfigOut) isIngestSourceOutConfig()      {}
func (DocusignConfigOut) isIngestSourceOutConfig()  {}
func (GithubConfigOut) isIngestSourceOutConfig()    {}
func (HubspotConfigOut) isIngestSourceOutConfig()   {}
func (PandaDocConfigOut) isIngestSourceOutConfig()  {}
func (SegmentConfigOut) isIngestSourceOutConfig()   {}
func (ShopifyConfigOut) isIngestSourceOutConfig()   {}
func (SlackConfigOut) isIngestSourceOutConfig()     {}
func (StripeConfigOut) isIngestSourceOutConfig()    {}
func (ZoomConfigOut) isIngestSourceOutConfig()      {}

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
	case "panda-doc":
		var c PandaDocConfigOut
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
	case "beehiiv", "brex", "clerk", "guesty", "incident-io", "lithic", "nash", "pleo", "replicate", "resend", "safebase", "sardine", "stych", "svix":
		var c SvixConfigOut
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
	"clerk":           IngestSourceOutTypeClerk,
	"docusign":        IngestSourceOutTypeDocusign,
	"github":          IngestSourceOutTypeGithub,
	"guesty":          IngestSourceOutTypeGuesty,
	"hubspot":         IngestSourceOutTypeHubspot,
	"incident-io":     IngestSourceOutTypeIncidentIo,
	"lithic":          IngestSourceOutTypeLithic,
	"nash":            IngestSourceOutTypeNash,
	"panda-doc":       IngestSourceOutTypePandaDoc,
	"pleo":            IngestSourceOutTypePleo,
	"replicate":       IngestSourceOutTypeReplicate,
	"resend":          IngestSourceOutTypeResend,
	"safebase":        IngestSourceOutTypeSafebase,
	"sardine":         IngestSourceOutTypeSardine,
	"segment":         IngestSourceOutTypeSegment,
	"shopify":         IngestSourceOutTypeShopify,
	"slack":           IngestSourceOutTypeSlack,
	"stripe":          IngestSourceOutTypeStripe,
	"stych":           IngestSourceOutTypeStych,
	"svix":            IngestSourceOutTypeSvix,
	"zoom":            IngestSourceOutTypeZoom,
}
