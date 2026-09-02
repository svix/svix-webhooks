# frozen_string_literal: true

require "json"
require "net/http"
require "uri"

require "svix"
require "svix/internal"
require "svix/models/poller_v2_poll_out"
require "svix/models/poller_v2_message_out"
require "svix/models/starting_position"

fixtures = JSON.parse(File.read(ENV.fetch("AUTOCONFIG_FIXTURES")))
lang = fixtures.fetch("languages").fetch("ruby")

SERVER_URL = fixtures.fetch("serverUrl")
ORG_TOKEN = fixtures.fetch("orgToken")
EVENT_TYPE = fixtures.fetch("eventType")
HTTP_URL = fixtures.fetch("httpUrl")
CONSUMER_ID = fixtures.fetch("consumerId")
APP_ID = lang.fetch("appId")

results = {}

def one_line(err)
  if err.respond_to?(:code) && err.respond_to?(:response_body)
    body = err.response_body.to_s.tr("\n", " ")[0, 240]
    "#{err.class}: #{err.code} #{body}"
  else
    "#{err.class}: #{err.message}".tr("\n", " ")[0, 240]
  end
end

def send_msg(src)
  uri = URI("#{SERVER_URL}/api/v1/app/#{APP_ID}/msg/")
  req = Net::HTTP::Post.new(uri)
  req["Authorization"] = "Bearer #{ORG_TOKEN}"
  req["Content-Type"] = "application/json"
  req.body = JSON.dump(
    "eventType" => EVENT_TYPE,
    "payload" => { "ok" => true, "src" => src }
  )
  res = Net::HTTP.start(uri.hostname, uri.port) { |http| http.request(req) }
  code = Integer(res.code)
  unless (200..299).cover?(code)
    raise "send msg failed #{code} #{res.body.to_s.tr("\n", " ")[0, 200]}"
  end
end

def payload_src(msg)
  payload = msg.payload
  if payload.is_a?(Hash)
    payload["src"] || payload[:src]
  else
    payload
  end
end

def receive_once(consumer)
  consumer.receive(
    CONSUMER_ID,
    "starting_position" => "earliest",
    "lease_duration_ms" => 2000
  )
end

def receive_with_retry(consumer, src)
  last = nil
  10.times do
    begin
      last = receive_once(consumer)
    rescue Svix::ApiError => e
      raise unless e.code.to_i == 423

      sleep 2
      next
    end
    data = last.data || []
    match = data.find { |m| m.event_type == EVENT_TYPE && payload_src(m).to_s == src }
    return [last, match] if match

    sleep 2
  end
  [last, nil]
end

def run_http(token)
  ep = Svix::AutoConfig.new(
    token,
    Svix::EndpointIn.new("url" => HTTP_URL, "event_types" => [EVENT_TYPE])
  ).subscribe
  if ep.is_a?(Svix::EndpointOut) && ep.id.to_s != "" && ep.url == HTTP_URL
    "PASS — EndpointOut id=#{ep.id} url=#{ep.url}"
  else
    "FAIL — unexpected subscribe result id=#{ep&.id.inspect} url=#{ep&.url.inspect}"
  end
rescue StandardError => e
  "FAIL — #{one_line(e)}"
end

def run_poller_existing(token, src)
  binder = Svix::AutoConfigConsumer.new(
    token,
    Svix::SinkInCommon.new("event_types" => [EVENT_TYPE])
  )
  dest = binder.subscribe
  dest_id = dest.id
  if dest_id.to_s.empty?
    return "FAIL — subscribe returned empty dest id"
  end

  consumer = Svix::AutoConfigConsumer.new(
    token,
    Svix::SinkInCommon.new("event_types" => [EVENT_TYPE])
  )
  send_msg(src)
  _poll, match = receive_with_retry(consumer, src)
  if match.nil?
    empty = (_poll&.data || []).map { |m| "#{m.offset}:#{m.event_type}" }.join(",")
    return "FAIL — no matching message after 10 retries dest=#{dest_id} seen=[#{empty}]"
  end

  offset = match.offset
  consumer.commit(CONSUMER_ID, offset)
  sleep 2
  after = begin
    receive_once(consumer)
  rescue Svix::ApiError => e
    raise unless e.code.to_i == 423

    nil
  end
  again = after && (after.data || []).any? { |m| m.offset == offset }
  if again
    "FAIL — dest=#{dest_id} offset=#{offset} still returned after commit"
  else
    "PASS — dest=#{dest_id} offset=#{offset} no subscribe on receiver"
  end
rescue StandardError => e
  line = one_line(e)
  if line.include?("501")
    "FAIL — DIOM_MISSING #{line}"
  else
    "FAIL — #{line}"
  end
end

def run_poller(token, src)
  consumer = Svix::AutoConfigConsumer.new(
    token,
    Svix::SinkInCommon.new("event_types" => [EVENT_TYPE])
  )
  dest = consumer.subscribe
  dest_id = dest.id
  dest_type = dest.serialize["type"]
  if dest.config.nil?
    return "FAIL — DestinationOut missing config dest=#{dest_id} type=#{dest_type}"
  end
  if dest_id.to_s.empty?
    return "FAIL — subscribe returned empty dest id"
  end

  send_msg(src)
  _poll, match = receive_with_retry(consumer, src)
  if match.nil?
    empty = (_poll&.data || []).map { |m| "#{m.offset}:#{m.event_type}" }.join(",")
    return "FAIL — no matching message after 10 retries dest=#{dest_id} seen=[#{empty}]"
  end

  offset = match.offset
  consumer.commit(CONSUMER_ID, offset)
  sleep 2
  after = begin
    receive_once(consumer)
  rescue Svix::ApiError => e
    raise unless e.code.to_i == 423

    nil
  end
  again = after && (after.data || []).any? { |m| m.offset == offset }
  if again
    "FAIL — dest=#{dest_id} offset=#{offset} still returned after commit"
  else
    "PASS — dest=#{dest_id} type=#{dest_type} offset=#{offset}"
  end
rescue StandardError => e
  line = one_line(e)
  if line.include?("Missing required field config") || line.include?("Required config field")
    "FAIL — DestinationOut missing config — #{line}"
  elsif line.include?("501")
    "FAIL — DIOM_MISSING #{line}"
  else
    "FAIL — #{line}"
  end
end

results["A"] = run_http(lang.fetch("v1Http"))
results["B"] = run_http(lang.fetch("v2Http"))
results["C"] = run_poller(lang.fetch("v1Poller"), "ruby-v1")
results["D"] = run_poller(lang.fetch("v2Poller"), "ruby-v2")
results["E"] = run_poller_existing(lang.fetch("v2PollerExisting"), "ruby-e")

puts "LANG: ruby"
puts "A: #{results["A"]}"
puts "B: #{results["B"]}"
puts "C: #{results["C"]}"
puts "D: #{results["D"]}"
puts "E: #{results["E"]}"
puts "Notes: RUBYLIB=ruby/lib; require svix/internal"
