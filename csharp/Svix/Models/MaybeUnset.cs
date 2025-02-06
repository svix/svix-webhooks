using Newtonsoft.Json;

namespace Svix.Models
{
    [JsonConverter(typeof(MaybeUnsetJsonConverterFactory))]
    public readonly struct MaybeUnset<T>
    {
        private readonly T? _value;
        private readonly MaybeUnsetState _state;

        private MaybeUnset(T? value, MaybeUnsetState state)
        {
            _value = value;
            _state = state;
        }

        public static MaybeUnset<T> Unset() => new(default, MaybeUnsetState.Unset);

        public static MaybeUnset<T> Set(T value) => new(value, MaybeUnsetState.Set);

        public static MaybeUnset<T> Null() => new(default, MaybeUnsetState.Null);

        public bool IsUnset => _state == MaybeUnsetState.Unset;

        public bool IsSet => _state == MaybeUnsetState.Set;

        public bool IsNull => _state == MaybeUnsetState.Null;

        public T Value =>
            _state == MaybeUnsetState.Set
                ? _value!
                : throw new InvalidOperationException(
                    "Cannot get value of unset or null MaybeUnset<T>"
                );

        private enum MaybeUnsetState
        {
            Unset,
            Set,
            Null,
        }

        public static implicit operator MaybeUnset<T>(T value) => Set(value);

        public override string ToString()
        {
            return _state switch
            {
                MaybeUnsetState.Unset => "<unset>",
                MaybeUnsetState.Null => "<null>",
                MaybeUnsetState.Set => _value?.ToString() ?? "<null>",
                _ => throw new InvalidOperationException("Invalid state"),
            };
        }
    }
}
