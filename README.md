# media-cache-envoy-ext-authz
External envoy authentication service using grpc (tonic) and redis to keep track of requests and deny/accept/preload requests. Missing but valid resources are fetched from LTS service with a wait time until next request
# overview
![media-cache-envoy-ext-authz excalidraw](https://github.com/user-attachments/assets/7ba5fde7-e64a-477f-855a-5b62ab300a64)
# additional notes
+ the goal of the auth service is to tell envoy whether the get is "OK" (200) and ready to be retrieved from the cache
+ if it exists but is loading reject the request and modify the envoy response to indicate to the end user the media is currently loading/processing (Can be cached with proper considerations)
+ if the resource does not exist reject the request and modify the envoy response to indicate to the end user the media does not exist (Cache this response)
+ the microservice should keep a small internal cache of recently processed requests to reduce server load, preventing redundant calls to redis or additional backend services
+ this program only interacts with envoy through ext_authz, redis for cache state, and a LTS service that responds with the status/availability of a resource in storage. it doesnt inteact with the cache itself
+ envoy fowards the get request to minio for it to be handled there once an OK response is given, certain responses (highly used assets) could be cached driectly at envoy level increasing speed but also storage/memory consumption
+ inside redis duration for the resource ready state can be tracked for increased resposniveness
+ ideally the end users client will properly handle back off durations for loading resources, even if not rate limiting at an envoy level and an auth service internal response cache should prevent abuse
