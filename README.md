# Static Serve

Simple self hosting file server which works with cURL

##### Run with docker
```
docker-compose up -d
```

```bash
# Upload
curl -X POST  -F "data=@<filename.ext>" http://localhost:3000

# Download
curl http://localhost:3000/uploads/<filename.ext>

```

![Index](./doc/index.png)
