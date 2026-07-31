import argparse
import json
from enum import Enum
import dataclasses
import logging
import sys
import re
from collections.abc import Iterable
from typing import Self, Generic, TypeVar, Iterator


class CheckMode(Enum):
    ARGUMENT = "ARGUMENT"
    RESPONSE = "RESPONSE"
    INVARIANT = "INVARIANT"


K = TypeVar("K")
V = TypeVar("V")


class frozendict(Generic[K, V]):
    def __init__(self, inner: dict[K, V]):
        self._inner = inner

    def items(self) -> Iterable[tuple[K, V]]:
        return self._inner.items()

    def keys(self) -> Iterable[K]:
        return self._inner.keys()

    def values(self) -> Iterable[V]:
        return self._inner.values()

    def get(self, key: K) -> V | None:
        return self._inner.get(key)

    def __getitem__(self, key: K) -> V:
        return self._inner[key]

    def __hash__(self):
        value = 0
        for k, v in sorted(self._inner.items()):
            value ^= hash((k, v))
        return value


T = TypeVar("T")


class frozenlist(Generic[T]):
    def __init__(self, inner: list[T]):
        self._inner = inner

    def __hash__(self):
        value = 0
        for item in self._inner:
            value ^= hash(item)
        return value

    def __iter__(self) -> Iterator[T]:
        return iter(self._inner)


class Schema:
    @classmethod
    def parse(cls, json: dict) -> "Schema":
        if "$ref" in json:
            obj = RefSchema(ref=json["$ref"])
        elif "oneOf" in json:
            obj = SumSchema(
                choices=frozenset(Schema.parse(i) for i in json["oneOf"]), mode="oneOf"
            )
        elif "anyOf" in json:
            obj = SumSchema(
                choices=frozenset(Schema.parse(i) for i in json["anyOf"]), mode="anyOf"
            )
        elif "type" not in json:
            raise ValueError(json)
        elif json["type"] == "object":
            obj = ObjectSchema.parse(json)
        elif json["type"] == "array":
            if isinstance(json.get("items"), list):
                obj = TupleSchema.parse(json)
            else:
                obj = ArraySchema.parse(json)
        elif json["type"] == "string":
            if "enum" in json:
                obj = EnumSchema(values=frozenset(json["enum"]))
            else:
                obj = StringSchema(pattern=json.get("pattern"))
        elif json["type"] == "integer":
            obj = IntSchema.parse(json)
        elif json["type"] == "boolean":
            obj = BooleanSchema()
        else:
            raise ValueError(f"Not implemented: {json}")
        if json.get("nullable", False):
            return NullableSchema(inner=obj)
        else:
            return obj

    def resolve(self, openapi: "OpenAPI") -> Self:
        _ = openapi
        return self

    def check_compatibility(self, other: "Schema", context: "Context"):
        _ = (other, context)
        raise ValueError(f"compatibility check not implemented for {self} {other}")

    def unwrap(self, check_mode: CheckMode) -> Self:
        _ = check_mode
        return self


@dataclasses.dataclass(eq=True, frozen=True)
class NullableSchema(Schema):
    inner: Schema

    def unwrap(self, check_mode: CheckMode) -> Schema:
        # for arguments, if the libs are nullable but OSS isn't, it's okay, so just return inner
        if check_mode == CheckMode.ARGUMENT:
            return self.inner
        return self

    def resolve(self, openapi: "OpenAPI") -> Self:
        return self.__class__(inner=self.inner.resolve(openapi))

    def check_compatibility(self, other: Schema, context: "Context"):
        if isinstance(other, NullableSchema):
            return self.inner.check_compatibility(other.inner, context)
        else:
            return self.inner.check_compatibility(other, context)


@dataclasses.dataclass(eq=True, frozen=True)
class SumSchema(Schema):
    mode: str
    choices: frozenset[Schema]

    def resolve(self, openapi: "OpenAPI") -> Self:
        return self.__class__(
            mode=self.mode, choices=frozenset(c.resolve(openapi) for c in self.choices)
        )

    def check_compatibility(self, other: Schema, context: "Context"):
        other = other.unwrap(context.check_mode)
        if not isinstance(other, SumSchema):
            return context.error(f"Sum is not compatible with {other}")
        if self.mode != other.mode:
            return context.error(f"{self.mode} is not compatible with {other.mode}")
        if context.check_mode == CheckMode.ARGUMENT:
            if extras := other.choices - self.choices:
                return context.error(f"Libs send extra argument options: {extras}")
        elif context.check_mode == CheckMode.RESPONSE:
            if extras := self.choices - other.choices:
                return context.error("OSS sends extra response options: {extras}")
        elif self.choices != other.choices:
            return context.error(
                "Incompatible sum type: {self.choices} != {other.choices}"
            )


@dataclasses.dataclass(eq=True, frozen=True)
class StringSchema(Schema):
    pattern: str | None

    def check_compatibility(self, other: "Schema", context: "Context"):
        other = other.unwrap(context.check_mode)
        if not isinstance(other, StringSchema):
            return context.error(f"string schema is not compatible with {other}")
        # TODO: handle the pattern
        # technically, it's a regular expression which generates an infinite set of strings, so there is some
        # subset relationship possible to define, but that sounds like real work...


@dataclasses.dataclass(eq=True, frozen=True)
class RefSchema(Schema):
    ref: str
    inner: Schema | None = None

    def resolve(self, openapi: "OpenAPI") -> Self:
        return self.__class__(
            ref=self.ref, inner=openapi.resolve_schema_ref(self.ref).resolve(openapi)
        )

    def unwrap(self, check_mode: CheckMode) -> Schema:
        _ = check_mode
        if self.inner is None:
            raise RuntimeError(f"Failed to resolve ref: {self.ref}")
        return self.inner

    def check_compatibility(self, other: "Schema", context: "Context"):
        self.unwrap(context.check_mode).check_compatibility(
            other, context.child(f"(*{self.ref})")
        )


@dataclasses.dataclass(eq=True, frozen=True)
class ArraySchema(Schema):
    item_type: Schema
    is_set: bool

    @classmethod
    def parse(cls, json: dict) -> "ArraySchema":
        return cls(
            item_type=Schema.parse(json["items"]),
            is_set=json.get("uniqueItems", False),
        )

    def resolve(self, openapi: "OpenAPI") -> Schema:
        return self.__class__(
            is_set=self.is_set, item_type=self.item_type.resolve(openapi)
        )

    def check_compatibility(self, other: "Schema", context: "Context"):
        other = other.unwrap(context.check_mode)
        if not isinstance(other, ArraySchema):
            return context.error(f"ArraySchema is not compatible with {other}")
        if self.is_set != other.is_set:
            context.error("OSS array schema is a set, Libs is not")
        self.item_type.check_compatibility(other.item_type, context.child("item_type"))


@dataclasses.dataclass(eq=True, frozen=True)
class TupleSchema(Schema):
    item_types: frozenlist[Schema]

    @classmethod
    def parse(cls, json: dict) -> "TupleSchema":
        return cls(item_types=frozenlist([Schema.parse(i) for i in json["items"]]))

    def resolve(self, openapi: "OpenAPI") -> Schema:
        return self.__class__(
            item_types=frozenlist([i.resolve(openapi) for i in self.item_types])
        )

    def check_compatibility(self, other: "Schema", context: "Context"):
        other = other.unwrap(context.check_mode)
        if not isinstance(other, TupleSchema):
            return context.error(f"TupleSchema is not compatible with {other}")
        if self.item_types != other.item_types:
            context.error(
                f"Incompatible tuple item types: {self.item_types} != {other.item_types}"
            )


@dataclasses.dataclass(eq=True, frozen=True)
class EnumSchema(Schema):
    values: frozenset[str]

    def check_compatibility(self, other: "Schema", context: "Context"):
        other = other.unwrap(context.check_mode)
        if not isinstance(other, EnumSchema):
            return context.error(f"EnumSchema is not compatible with {other}")
        match context.check_mode:
            case CheckMode.ARGUMENT:
                if not other.values.issubset(self.values):
                    return context.error(
                        f"OSS enum expects values {self.values}, but Libs can send {other.values}"
                    )
            case CheckMode.RESPONSE:
                if not self.values.issubset(other.values):
                    return context.error(
                        f"OSS enum returns values {self.values}, but Libs expects {other.values}"
                    )
            case CheckMode.INVARIANT:
                if not self.values == other.values:
                    return context.error(
                        f"OSS enum has values {self.values}, but Libs expects {other.values}"
                    )


@dataclasses.dataclass(eq=True, frozen=True)
class IntSchema(Schema):
    signed: bool
    bits: int
    format: str

    REGEX = re.compile(r"^(u?)int([0-9]*)$")

    @classmethod
    def parse(cls, json: dict) -> "IntSchema":
        format = json.get("format", "int32")
        md = cls.REGEX.match(format)
        if md is None:
            raise ValueError(f"Unhandled in format {format}")
        signed = md.group(1) == "u"
        bits = int(md.group(2) or 32)
        return cls(signed, bits, format)

    def check_compatibility(self, other: "Schema", context: "Context"):
        other = other.unwrap(context.check_mode)
        if not isinstance(other, IntSchema):
            return context.error(
                f"integer schemas are only compatible with other integer schemas (got {other})"
            )
        if context.check_mode == CheckMode.INVARIANT:
            return context.error(
                f"incompatible integer formats: {self.format} != {other.format}"
            )
        elif context.check_mode == CheckMode.ARGUMENT:
            if self.bits < other.bits:
                context.error(
                    "OSS expects an argument with {self.bits} bits, but libs send {other.bits}"
                )
        elif context.check_mode == CheckMode.RESPONSE:
            if self.bits > other.bits:
                context.error(
                    "OSS returns {self.bits} bits, but libs expecft {other.bits}"
                )
        # TODO: handle signedness


@dataclasses.dataclass(eq=True, frozen=True)
class BooleanSchema(Schema):
    def check_compatibility(self, other: "Schema", context: "Context"):
        other = other.unwrap(context.check_mode)
        if not isinstance(other, BooleanSchema):
            return context.error(
                f"boolean schemas are only compatible with other boolean schemas (got {other})"
            )


@dataclasses.dataclass(eq=True, frozen=True)
class ObjectSchema(Schema):
    properties: frozendict[str, Schema]
    required: frozenset[str]

    @classmethod
    def parse(cls, json: dict) -> "ObjectSchema":
        return cls(
            properties=frozendict(
                {k: Schema.parse(v) for k, v in json.get("properties", {}).items()}
            ),
            required=frozenset(json.get("required", [])),
        )

    def resolve(self, openapi: "OpenAPI") -> Schema:
        return self.__class__(
            required=self.required,
            properties=frozendict(
                {k: v.resolve(openapi) for (k, v) in self.properties.items()}
            ),
        )

    def check_compatibility(self, other: "Schema", context: "Context"):
        other = other.unwrap(context.check_mode)
        if not isinstance(other, ObjectSchema):
            return context.error(f"ObjectSchema is not compatible with {other}")
        for property in sorted(
            set(self.properties.keys()) | set(other.properties.keys())
        ):
            ours = self.properties.get(property)
            ours_is_required = property in self.required
            theirs = other.properties.get(property)
            theirs_is_required = property in other.required
            # TODO: handle covariance correctly instead of treating as always-invariant
            match (ours, theirs):
                case (None, None):
                    pass
                case (None, theirs):
                    if theirs_is_required:
                        context.error(
                            f"property {property} is required by libs, not in OSS"
                        )
                        continue
                case (ours, None):
                    if ours_is_required:
                        context.error(
                            f"property {property} is required by OSS, but not in libs"
                        )
                    continue
                case (ours, theirs):
                    ours.check_compatibility(theirs, context.child(property))


@dataclasses.dataclass(eq=True, frozen=True)
class ParameterKey:
    in_kind: str
    name: str
    style: str | None

    def __str__(self):
        return f"{self.in_kind}({self.name})"


@dataclasses.dataclass(eq=True, frozen=True)
class Parameter:
    in_kind: str
    name: str
    schema: Schema
    style: str | None = None

    def key(self) -> ParameterKey:
        return ParameterKey(in_kind=self.in_kind, name=self.name, style=self.style)

    @classmethod
    def parse(cls, json: dict) -> "Parameter":
        return cls(
            in_kind=json.get("in", "query"),
            name=json.get("name", ""),
            schema=Schema.parse(json["schema"]),
            style=json.get("style"),
        )

    def resolve(self, openapi: "OpenAPI") -> Self:
        return self.__class__(
            in_kind=self.in_kind,
            name=self.name,
            schema=self.schema.resolve(openapi),
            style=self.style,
        )

    def check_compatibility(self, other: Self, context: "Context"):
        return self.schema.check_compatibility(other.schema, context)


class Body:
    @classmethod
    def parse(cls, json: dict) -> "Body":
        types = set(json.keys())
        if types == {"application/octet-stream"}:
            return OpaqueBody()
        elif types == {"application/json"}:
            return JsonBody.parse(json["application/json"])
        else:
            raise ValueError(f"Unsupported content type {json}")

    def resolve(self, openapi: "OpenAPI") -> Self:
        _ = openapi
        return self

    def check_compatibility(self, other: Self, context: "Context"):
        if not isinstance(other, self.__class__):
            context.error(f"Type mismatch: {self.__class__} != {other.__class__}")


@dataclasses.dataclass(eq=True, frozen=True)
class EmptyBody(Body):
    pass


@dataclasses.dataclass(eq=True, frozen=True)
class JsonBody(Body):
    schema: Schema

    @classmethod
    def parse(cls, json: dict) -> "JsonBody":
        return cls(schema=Schema.parse(json["schema"]))

    def resolve(self, openapi: "OpenAPI") -> Self:
        return self.__class__(schema=self.schema.resolve(openapi))

    def check_compatibility(self, other: Body, context: "Context"):
        if not isinstance(other, self.__class__):
            return context.error(
                f"Type mismatch: {self.__class__} != {other.__class__}"
            )
        self.schema.check_compatibility(other.schema, context)


@dataclasses.dataclass(eq=True, frozen=True)
class OpaqueBody(Body):
    pass


@dataclasses.dataclass(eq=True, frozen=True)
class Operation:
    path: str
    method: str
    operationId: str
    parameters: frozendict[ParameterKey, Parameter]
    ok_responses: frozendict[int, Body]
    body: Body

    @classmethod
    def parse(cls, path: str, method: str, json: dict) -> "Operation":
        body = EmptyBody()
        if json.get("requestBody", {}).get("content", None):
            body = Body.parse(json["requestBody"]["content"])
        ok_responses = {}
        for k, v in json.get("responses", {}).items():
            if int(k) < 300:
                if v.get("content"):
                    ok_responses[k] = Body.parse(v["content"])
                else:
                    ok_responses[k] = EmptyBody()
        parameters = {}
        for props in json.get("parameters", []):
            p = Parameter.parse(props)
            parameters[p.key()] = p

        return cls(
            path=path,
            method=method,
            operationId=json.get("operationId"),
            parameters=frozendict(parameters),
            body=body,
            ok_responses=frozendict(ok_responses),
        )

    def resolve(self, openapi: "OpenAPI") -> Self:
        return dataclasses.replace(
            self,
            parameters=frozendict(
                {p.key(): p.resolve(openapi) for p in self.parameters.values()}
            ),
            ok_responses=frozendict(
                {i: b.resolve(openapi) for (i, b) in self.ok_responses.items()}
            ),
            body=self.body.resolve(openapi),
        )

    def check_compatibility(self, other: Self, context: "Context"):
        context = context.child(self.operationId)
        # assume unrecognized parameters are ignored
        for parameter_key in set(self.parameters.keys()) & set(other.parameters.keys()):
            ours = self.parameters[parameter_key]
            theirs = other.parameters[parameter_key]
            ours.check_compatibility(
                theirs, context.child(str(parameter_key), CheckMode.ARGUMENT)
            )
        # check the body
        if type(self.body) is type(other.body):
            self.body.check_compatibility(
                other.body, context.child("body", CheckMode.ARGUMENT)
            )
        else:
            context.error(
                "Incompatible body types: {type(self.body)} != {type(other.body)}"
            )
        # check the responses
        for code, response in other.ok_responses.items():
            ours = self.ok_responses.get(code)
            if ours is None:
                logging.debug("Response %s is in cloud, but not OSS", code)
                continue
            ours.check_compatibility(
                response, context.child(f"response {code}", CheckMode.RESPONSE)
            )


@dataclasses.dataclass
class OpenAPI:
    paths: dict[str, dict[str, Operation]]
    components_schemas: dict[str, Schema]

    def __init__(self):
        self.paths = {}
        self.components_schemas = {}

    @classmethod
    def parse(cls, json: dict):
        self = cls()
        for name, value in json.get("components", {}).get("schemas", {}).items():
            self.components_schemas[name] = Schema.parse(value)
        for path, methods in json.get("paths", {}).items():
            for method, props in methods.items():
                self.paths.setdefault(path, {})[method] = Operation.parse(
                    path, method, props
                )
        return self

    def paths_and_methods(self):
        for path, methods in self.paths.items():
            for method in methods:
                yield (path, method)

    def resolve_schema_ref(self, ref: str) -> Schema:
        if ref.startswith("#/components/schemas"):
            schema_name = ref.removeprefix("#/components/schemas/")
            return self.components_schemas[schema_name]
        else:
            raise ValueError(r"Unhandled ref type {ref}")


@dataclasses.dataclass
class RootContext:
    warnings: int = 0
    errors: int = 0

    def warning(self, message: str):
        logging.warning(message)
        self.warnings += 1

    def child(self, tag: str, check_mode: CheckMode = CheckMode.INVARIANT) -> "Context":
        return Context(tags=[tag], root=self, check_mode=check_mode)


@dataclasses.dataclass
class Context:
    tags: list[str]
    root: RootContext
    check_mode: CheckMode

    def error(self, message: str):
        tags = "/".join(self.tags)
        logging.error(f"[{tags}]: {message}")
        self.root.errors += 1

    def warning(self, message: str):
        tags = "/".join(self.tags)
        logging.warning(f"[{tags}]: {message}")
        self.root.warnings += 1

    def child(self, tag: str, check_mode: CheckMode | None = None) -> Self:
        if check_mode is None:
            check_mode = self.check_mode
        return self.__class__(
            tags=self.tags + [tag], root=self.root, check_mode=check_mode
        )


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("-v", "--verbose", action="store_true")
    parser.add_argument("oss_openapi_json")
    parser.add_argument("libs_openapi_json")
    args = parser.parse_args()

    logging.basicConfig(
        level=logging.DEBUG if args.verbose else logging.INFO,
        stream=sys.stderr,
        format="%(levelname)s %(message)s",
    )
    logger = logging.getLogger("validate")

    oss = OpenAPI.parse(json.load(open(args.oss_openapi_json, "r")))
    libs = OpenAPI.parse(json.load(open(args.libs_openapi_json, "r")))

    context = RootContext()

    for path, method in sorted(
        set(oss.paths_and_methods()) | set(libs.paths_and_methods())
    ):
        oss_op = oss.paths.get(path, {}).get(method)
        lib_op = libs.paths.get(path, {}).get(method)
        if oss_op is None:
            logger.debug(f"Path only in Cloud, not in OSS: {method} {path}")
            continue
        if lib_op is None:
            continue
        oss_resolved = oss_op.resolve(oss)
        libs_resolved = lib_op.resolve(libs)
        oss_resolved.check_compatibility(libs_resolved, context)

    if context.errors > 0:
        return 1
    else:
        print("🎉 No errors")
        return 0


sys.exit(main())
