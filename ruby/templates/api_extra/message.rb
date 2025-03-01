# Creates a [`MessageIn`] with a raw string payload.
#
# The payload is not normalized on the server. Normally, payloads are required
# to be JSON, and Svix will minify the payload before sending the webhook
# (for example, by removing extraneous whitespace or unnecessarily escaped
# characters in strings). With this function, the payload will be sent
# "as is", without any minification or other processing.
#
# `attributes[:payload]` must be a string. An extra attribute `content_type`
# is accepted that sets the `content-type` header of the webhook sent by Svix,
# overwriting the default of `application/json` if specified. Other than that,
# the attributes are forwarded [`MessageIn.new`], so all the same ones are
# accepted.
def message_in_raw(attributes = {})
  attributes[:transformations_params] ||= {}
  attributes[:transformations_params][:rawPayload] = attributes[:payload]
  attributes[:payload] = {}

  content_type = attributes.delete(:content_type)
  if content_type != nil
    attributes[:transformations_params][:headers] ||= {}
    attributes[:transformations_params][:headers][:'content-type'] = content_type
  end

  MessageIn.new(attributes)
end
