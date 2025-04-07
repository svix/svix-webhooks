// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { CompletionChoice, CompletionChoiceSerializer } from "./completionChoice";

export interface GenerateOut {
  choices: CompletionChoice[];
  created: number;
  id: string;
  model: string;
  object: string;
}

export const GenerateOutSerializer = {
  _fromJsonObject(object: any): GenerateOut {
    return {
      choices: object["choices"]?.map((item: CompletionChoice) =>
        CompletionChoiceSerializer._fromJsonObject(item)
      ),
      created: object["created"],
      id: object["id"],
      model: object["model"],
      object: object["object"],
    };
  },

  _toJsonObject(self: GenerateOut): any {
    return {
      choices: self.choices.map((item) => CompletionChoiceSerializer._toJsonObject(item)),
      created: self.created,
      id: self.id,
      model: self.model,
      object: self.object,
    };
  },
};
