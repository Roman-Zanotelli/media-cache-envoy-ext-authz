# deprecated
System will use existing local storage (private dedicated machines) and burst to s3 under certain conditions, saving expenses while maintaining reliable scalability.
An Envoy ext_auth filter will still be used to process the request similar to the initial step of this system, without additional services or fetching logic.

# media-cache-envoy-ext-authz
External envoy authentication service using grpc (tonic) and redis to keep track of requests and deny/accept/preload requests. Missing but valid resources are fetched from LTS service with a wait time until next reasonable request period
# overview
![media-cache-envoy-ext-authz excalidraw](https://github.com/user-attachments/assets/7ba5fde7-e64a-477f-855a-5b62ab300a64)
#### redis state system architecture
+ https://github.com/Roman-Zanotelli/media-cache-redis-state-sync
#### long term storage service architecture
+ TODO!
# additional notes
+ the goal of the auth service is to tell envoy whether the get is "OK" (200) and ready to be retrieved from the cache
+ if it exists but is loading reject the request and modify the envoy response to indicate to the end user the media is currently loading/processing (Can be cached with proper considerations)
+ if the resource does not exist reject the request and modify the envoy response to indicate to the end user the media does not exist (Cache this response)
+ this microservice could keep a small internal cache of recently processed requests to reduce server load, preventing redundant calls to redis or additional backend services (optional, and will consume more memory per instance but would reduce any additional external system strain, the overall decision can be made later with proper testing and has little to do with core functionality)
+ this program only interacts with envoy through ext_authz, redis for cache state, and a LTS service that responds with the status/availability of a resource in storage. it doesnt interact with the cache itself
+ envoy fowards the get request to minio for it to be handled there once an OK response is given, certain responses (highly used assets) could be cached driectly at envoy level increasing speed but also storage/memory consumption
+ inside redis duration for the resource ready state can be tracked for increased resposniveness
+ ideally the end users client will properly handle back off durations for loading resources, even if not rate limiting at an envoy level and an auth service internal response cache should prevent abuse
+ also the LTS service should communicate directly with the cache (or atleast redis) to ensure state before loading from AWS
+ the main reason the cache does not directly relay upstream to the AWS s3 storage is to prevent/minimize/optimize api calls to S3 (each api call costs money, allowing cache misses to be relayed unchecked to aws s3 can cause unecessary cost overtime versus using a private DB as a registery of the aws s3 storage state, with this private registry only valid calls are made resulting in reduced aws costs)
