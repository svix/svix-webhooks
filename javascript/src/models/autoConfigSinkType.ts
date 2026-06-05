// this file is @generated
import { type EndpointIn, EndpointInSerializer } from "./endpointIn";
import { type SinkInCommon, SinkInCommonSerializer } from "./sinkInCommon";
// biome-ignore lint/suspicious/noEmptyInterface: backwards compat
interface _AutoConfigSinkTypeFields {}

interface AutoConfigSinkTypePoller {
  type: "poller";
  config: SinkInCommon;
}

interface AutoConfigSinkTypeHttp {
  type: "http";
  config: EndpointIn;
}

export type AutoConfigSinkType = _AutoConfigSinkTypeFields &
  (AutoConfigSinkTypePoller | AutoConfigSinkTypeHttp);

export const AutoConfigSinkTypeSerializer = {
  _fromJsonObject(object: any): AutoConfigSinkType {
    const type = object["type"];

    function getConfig(type: string): any {
      switch (type) {
        case "poller":
          return SinkInCommonSerializer._fromJsonObject(object["config"]);
        case "http":
          return EndpointInSerializer._fromJsonObject(object["config"]);
        default:
          throw new Error(`Unexpected type: ${type}`);
      }
    }

    return {
      type,
      config: getConfig(type),
    };
  },

  _toJsonObject(self: AutoConfigSinkType): any {
    // biome-ignore lint/suspicious/noImplicitAnyLet: the return type needs to be any
    let config;
    switch (self.type) {
      case "poller":
        config = SinkInCommonSerializer._toJsonObject(self.config);
        break;
      case "http":
        config = EndpointInSerializer._toJsonObject(self.config);
        break;
    }

    return {
      type: self.type,
      config: config,
    };
  },
};
