# media-cache-envoy-ext-authz
External envoy authentication service using grpc (tonic) and redis to keep track of requests and deny/accept/preload requests. Missing but valid resources are fetched from LTS service with a wait time until next request
