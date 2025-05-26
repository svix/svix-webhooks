// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

type ConnectorKind string

const (
	CONNECTORKIND_CUSTOM      ConnectorKind = "Custom"
	CONNECTORKIND_CLOSE_CRM   ConnectorKind = "CloseCRM"
	CONNECTORKIND_CUSTOMER_IO ConnectorKind = "CustomerIO"
	CONNECTORKIND_DISCORD     ConnectorKind = "Discord"
	CONNECTORKIND_HUBSPOT     ConnectorKind = "Hubspot"
	CONNECTORKIND_INNGEST     ConnectorKind = "Inngest"
	CONNECTORKIND_LOOPS       ConnectorKind = "Loops"
	CONNECTORKIND_RESEND      ConnectorKind = "Resend"
	CONNECTORKIND_SALESFORCE  ConnectorKind = "Salesforce"
	CONNECTORKIND_SEGMENT     ConnectorKind = "Segment"
	CONNECTORKIND_SENDGRID    ConnectorKind = "Sendgrid"
	CONNECTORKIND_SLACK       ConnectorKind = "Slack"
	CONNECTORKIND_TEAMS       ConnectorKind = "Teams"
	CONNECTORKIND_TRIGGER_DEV ConnectorKind = "TriggerDev"
	CONNECTORKIND_WINDMILL    ConnectorKind = "Windmill"
	CONNECTORKIND_ZAPIER      ConnectorKind = "Zapier"
)

var allowedConnectorKind = []ConnectorKind{
	"Custom",
	"CloseCRM",
	"CustomerIO",
	"Discord",
	"Hubspot",
	"Inngest",
	"Loops",
	"Resend",
	"Salesforce",
	"Segment",
	"Sendgrid",
	"Slack",
	"Teams",
	"TriggerDev",
	"Windmill",
	"Zapier",
}

func (v *ConnectorKind) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := ConnectorKind(value)
	if slices.Contains(allowedConnectorKind, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid ConnectorKind", value)

}

var ConnectorKindFromString = map[string]ConnectorKind{
	"Custom":     CONNECTORKIND_CUSTOM,
	"CloseCRM":   CONNECTORKIND_CLOSE_CRM,
	"CustomerIO": CONNECTORKIND_CUSTOMER_IO,
	"Discord":    CONNECTORKIND_DISCORD,
	"Hubspot":    CONNECTORKIND_HUBSPOT,
	"Inngest":    CONNECTORKIND_INNGEST,
	"Loops":      CONNECTORKIND_LOOPS,
	"Resend":     CONNECTORKIND_RESEND,
	"Salesforce": CONNECTORKIND_SALESFORCE,
	"Segment":    CONNECTORKIND_SEGMENT,
	"Sendgrid":   CONNECTORKIND_SENDGRID,
	"Slack":      CONNECTORKIND_SLACK,
	"Teams":      CONNECTORKIND_TEAMS,
	"TriggerDev": CONNECTORKIND_TRIGGER_DEV,
	"Windmill":   CONNECTORKIND_WINDMILL,
	"Zapier":     CONNECTORKIND_ZAPIER,
}
