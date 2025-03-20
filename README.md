# media-cache-envoy-ext-authz-demo
Demo/PoC of an external envoy authentication service using grpc (tonic) and redis to keep track of requests and deny/accept/preload requests. Missing but valid resources are fetched from LTS service with a wait time until next reasonable request period.
This system will reduce and safeguard against AWS costs by caching Stored AWS data on local minio nodes with eviction policies for when full. It also lowers AWS API costs by keeping a record of AWS objects locally within the private cloud so calls to S3 invalid resources do not occur.
This repo serves moreso as a test implementation/experimentaion of how this system may be implemented and the code is not planned for any production use.
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

# deprecated
This sub-system architecture is no longer planned for use and remains here soley as a PoC on how you could cache AWS S3 data to reduce AWS costs, The costs of AWS does not scale against the cost of dedicated machines running a private minio cluster based off my calculations. Attached is an Excel file for rough cost estimation compared to OVH dedicated hosting; While AWS S3 provides initial low upfront costs, storage and Data transfer costs long term are not sustainable compared to the costs of running a private cluster on OVH dedicated storage nodes. AWS may still be used for "Bursting" when storage requirements are exceeded but dedicated nodes are not available for scaling; In the future this will also likely be removed for scalable dedicated hosting using a providers API (I believe OVHCloud supports this already) for provisioning managed dedicated machines programatically, but this is a low priority issue/feature.
# replacement
This sub-system architecture will be replaced by envoy + external_auth_service (similar to this one) that does not prefetch data but instead solely OK/DENY the request based off interaction parameters and other relevant system state. Redis may possible still be used to store metadata withing the system, more tests need to be done to determine if using redis would even help system-load and response times compared to just directly checking Minio through the auth service, or not checking at all (besides external non minio related checks through the ext-auth-service) and just caching/rate-limiting responses of Minio requests through envoy.
