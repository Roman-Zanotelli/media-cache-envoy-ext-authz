# media-cache-envoy-ext-authz
External envoy authentication service using grpc (tonic) and redis to keep track of requests and deny/accept/preload requests. Missing but valid resources are fetched from LTS service with a wait time until next request
# overview
![media-cache-envoy-ext-authz excalidraw](https://github.com/user-attachments/assets/7ba5fde7-e64a-477f-855a-5b62ab300a64)
# additional notes
+ some responses can be cached at envoy level to reduce overall system strain
