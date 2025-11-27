<?php

// this file is @generated
declare(strict_types=1);

namespace Svix\Models;

enum ConnectorKind: string implements \JsonSerializable
{
    case CUSTOM = 'Custom';
    case AGENTIC_COMMERCE_PROTOCOL = 'AgenticCommerceProtocol';
    case CLOSE_CRM = 'CloseCRM';
    case CUSTOMER_IO = 'CustomerIO';
    case DISCORD = 'Discord';
    case HUBSPOT = 'Hubspot';
    case INNGEST = 'Inngest';
    case LOOPS = 'Loops';
    case OTEL = 'Otel';
    case RESEND = 'Resend';
    case SALESFORCE = 'Salesforce';
    case SEGMENT = 'Segment';
    case SENDGRID = 'Sendgrid';
    case SLACK = 'Slack';
    case TEAMS = 'Teams';
    case TRIGGER_DEV = 'TriggerDev';
    case WINDMILL = 'Windmill';
    case ZAPIER = 'Zapier';

    /**
     * Create an instance from a mixed obj.
     */
    public static function fromMixed(mixed $data): self
    {
        return self::from($data);
    }

    public function jsonSerialize(): mixed
    {
        return $this->value;
    }
}
