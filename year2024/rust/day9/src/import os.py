import os

import ory_client
from ory_client.api import o_auth2_api
from ory_client.model.json_patch import JsonPatch
from ory_client.model.json_patch_document import JsonPatchDocument
from ory_client.model.o_auth2_client import OAuth2Client

ORY_HOST = os.environ["ORY_HOST"]
ORY_AUTH_TOKEN = os.environ["ORY_AUTH_TOKEN"]
FRONTEND_ORY_CLIENT_ID = os.getenv("FRONTEND_ORY_CLIENT_ID")
DOMAIN = os.getenv("DOMAIN")

configuration = ory_client.Configuration(
    host=ORY_HOST,
    access_token=ORY_AUTH_TOKEN,
)

with ory_client.ApiClient(configuration) as api_client:
    api_instance = o_auth2_api.OAuth2Api(api_client)
    id = FRONTEND_ORY_CLIENT_ID
    domain = DOMAIN

    try:
        api_response: OAuth2Client = api_instance.get_o_auth2_client(id).to_dict()
        redirect_uris = api_response["redirect_uris"]
        post_logout_redirect_uris = api_response["post_logout_redirect_uris"]

        callback = f"{DOMAIN}/auth/callback"
        if callback in redirect_uris:
            redirect_uris.remove(domain)

            updates.append(
                            JsonPatch(op="replace", path="/redirect_uris", value=redirect_uris)
                        )
                        print(f"Update client with callback: {callback}\n")

            json_patch_document = JsonPatchDocument(
                [JsonPatch(op="replace", path="/redirect_uris", value=redirect_uris)]
            )

            # Patch the OAuth2 client
            api_instance.patch_o_auth2_client(id, json_patch_document)
            print(f"Client updated and removed redirect url: {domain}\n")
        else:
            print(f"Client didn't have redirect url: {domain}\n")
    except Exception as e:
        print("Exception when calling updating OAuth2Api %s\n" % e)
