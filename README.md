IP: 108.128.197.71

```bash
ssh -i ~/.ssh/keys/aws.pem ubuntu@108.128.197.71
```

Deploy:

```bash
cd telemetry-proxy
make deploy
```

The server runs with supervisord.
