import svix
from svix.api import MessageListOptions
import httpretty

@httpretty.activate(verbose=True, allow_net_connect=False)
def test_octothorpe_in_query_param():
    svx = svix.Svix("token",svix.SvixOptions(server_url="http://test.example"))
    httpretty.register_uri(
        httpretty.GET,
        "http://test.example/api/v1/app/app_id/msg?tag=test%23test",
        body='{"data":[],"iterator":null,"prevIterator":null,"done":true}'
    )
    svx.message.list("app_id",MessageListOptions(tag="test#test"))
