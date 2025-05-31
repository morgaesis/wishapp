# Infrastructure Deployment

## Key Components

- AWS Lambda (Rust runtime)
- API Gateway REST API
- Automated PR environments

## Workflow

- PRs: Auto-deploy preview environments (pr-{number})
- Main branch: Manual approval required for production
- Closed PRs: Automatic cleanup

## Development

```sh
npm test   # Run infrastructure tests
npm build  # Compile CDK code
cdk synth  # Generate CloudFormation
```

## Local Lambda Development

To test and debug Lambda functions locally, use `cargo lambda watch` to start a local runtime server and `cargo lambda invoke` to send test payloads.

1.  **Start the local Lambda runtime:**

    ```sh
    cargo lambda watch --invoke-address 127.0.0.1 --invoke-port 9000
    ```

    This command will compile your Lambda function and start a local server. It will appear to "hang" as it waits for invocations.

2.  **Invoke the Lambda function with a payload:**
    In a separate terminal, send a test payload to your running Lambda function:

    ```sh
    cargo lambda invoke --invoke-address 127.0.0.1 --invoke-port 9000 --data-ascii '{"command": "hello"}'
    ```

    Replace `'{"command": "hello"}'` with the appropriate JSON payload for your Lambda function.

    The Lambda function will process the payload and return a response. The `cargo lambda watch` terminal will show logs from the Lambda function's execution.

## CI/CD

- Uses GitHub Actions with OIDC
- Production: api.example.com
- Previews: pr-{number}.api.example.com
